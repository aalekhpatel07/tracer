use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;
use tracer::commons::progress_bars::*;
use tracer::commons::write_ppm;
use tracer::commons::HittableList;
use tracer::commons::Sphere;
use tracer::commons::{progress_bars, Material, Scatter};
use tracer::commons::{Angle, Degrees};
use tracer::commons::{Camera, Hittable, LinAlgOp, Pixel, Point, Ray, Vec3};

pub fn interpolate_linear(start: Vec3, end: Vec3, time: f64) -> Vec3 {
    (1.0 - time) * start + time * end
}

pub fn gamma2_correct(color: Vec3, gamma: usize) -> Vec3 {
    [color.0, color.1, color.2]
        .map(|x| x.powf(1. / gamma as f64))
        .into()
}

pub fn ray_color(ray: &Ray, world: &HittableList, depth: isize) -> Vec3 {
    if depth <= 0 {
        return Vec3::new(0., 0., 0.);
    }

    if let Some(hit_record) = world.hit(ray, 0.001, f64::INFINITY) {
        return if let Some((attenuation, scattered)) = hit_record.material.scatter(ray, &hit_record)
        {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Vec3::default()
        };
    }

    let unit_vector_in_direction_of_ray = ray.direction.unit_vector();
    let time = 0.5 * (unit_vector_in_direction_of_ray.1 + 1.);

    interpolate_linear(
        Vec3::new(1., 1., 1.),    // White
        Vec3::new(0.5, 0.7, 1.0), // Blue
        time,
    )
}

#[allow(clippy::too_many_arguments)]
fn process_pixel(
    row: usize,
    col: usize,
    camera: Arc<Camera>,
    world: Arc<HittableList>,
    samples_per_pixel: usize,
    image_width: usize,
    image_height: usize,
    max_depth: isize,
) -> Pixel {
    let mut rng = thread_rng();
    let mut pixel_color: Vec3 = Vec3::new(0., 0., 0.);

    for _ in 0..samples_per_pixel {
        let u = (col as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
        let v = (row as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
        let ray: Ray = camera.get_ray(u, v);
        pixel_color += ray_color(&ray, &world, max_depth);
    }

    gamma2_correct(pixel_color / samples_per_pixel as f64, 2).into()
}

/// Process all the pixels in parallel on CPU.
///
pub fn par_process_pixels(
    image_width: usize,
    image_height: usize,
    camera: Arc<Camera>,
    world: Arc<HittableList>,
    samples_per_pixel: usize,
    max_depth: isize,
    progress_bar: ProgressBar,
) -> Vec<Pixel> {
    let rows = 0..image_height;
    let cols = 0..image_width;

    let cross: Arc<Vec<(usize, usize)>> = Arc::new(
        rows.flat_map(|row| cols.clone().map(move |col| (row, col)))
            .collect::<Vec<(usize, usize)>>(),
    );

    // Too bad we cannot have a progress_bar
    // with rayon. Technically we can but that
    // causes a bottleneck as the progress bar
    // is behind a RwLock.

    // One helpful fix is to throttle the draw rate
    // with `pb.set_draw_delta(20_000)`.

    // To prevent frequent updating of the progress bar.
    // https://github.com/console-rs/indicatif/issues/170#issuecomment-617128991

    let mut pixels = cross
        .as_slice()
        .par_iter() // Rayon goes brrrr...
        .progress_with(progress_bar)
        .map(|item: &(usize, usize)| {
            let value = process_pixel(
                image_height - item.0 - 1,
                item.1,
                camera.clone(),
                world.clone(),
                samples_per_pixel,
                image_width,
                image_height,
                max_depth,
            );
            (*item, value)
        })
        .collect::<Vec<((usize, usize), Pixel)>>();

    // Since we have the (row, col) as the first component,
    // the sort would happen on the first component and
    // we'd get the pixels in the correct order that will
    // then be written to a ppm file.
    pixels.par_sort();
    pixels
        .into_iter()
        .map(|((_r, _c), px): ((usize, usize), Pixel)| px)
        .collect::<Vec<Pixel>>()
}

pub fn create_random_world() -> HittableList {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Material::Lambertian {
        albedo: Vec3::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Material::Lambertian {
        albedo: Vec3::new(0.1, 0.2, 0.5),
    });
    // let material_center = Arc::new(Material::Dielectric { index_of_refraction: 1.5 });
    let material_left = Arc::new(Material::Dielectric {
        index_of_refraction: 1.5,
    });
    // let material_left = Arc::new(Material::Metal { albedo: Vec3::new(0.8, 0.8, 0.8), fuzz: 0.3 });
    let material_right = Arc::new(Material::Metal {
        albedo: Vec3::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });

    // Some objects: Spheres
    let sphere_1: Arc<dyn Hittable> = Arc::new(Sphere::new(
        Point::new(0., -100.5, -1.),
        100.0,
        material_ground,
    ));

    let sphere_2: Arc<dyn Hittable> =
        Arc::new(Sphere::new(Point::new(0., 0., -1.), 0.5, material_center));

    let sphere_3: Arc<dyn Hittable> = Arc::new(Sphere::new(
        Point::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    ));

    let sphere_4: Arc<dyn Hittable> =
        Arc::new(Sphere::new(Point::new(1., 0., -1.), 0.5, material_right));

    let sphere_5: Arc<dyn Hittable> =
        Arc::new(Sphere::new(Point::new(-1., 0., -1.), -0.4, material_left));

    world.push(sphere_1);
    world.push(sphere_2);
    world.push(sphere_3);
    world.push(sphere_4);
    world.push(sphere_5);
    world
}

fn main() {
    let world = create_random_world();
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio).round() as usize;
    let max_depth: isize = 100;

    // Camera
    let look_from = Point::new(3., 3., 2.);
    let look_at = Point::new(0., 0., -1.);
    let view_up = Vec3::new(0., 1., 0.);
    let vertical_field_of_view = Angle::Degrees(Degrees(20.0));
    let aperture = 0.1;
    let focus_distance = (look_from - look_at).norm();

    let camera = Camera::new(
        look_from,
        look_at,
        view_up,
        vertical_field_of_view,
        aspect_ratio,
        aperture,
        focus_distance,
    );

    // Process pixel data.
    let samples_per_pixel: usize = 100;

    let pixel_pb = progress_bars::default(image_width * image_height);

    // Redraw every 1% of the progress.
    pixel_pb.set_draw_delta((image_width as u64 * image_height as u64) / 100);

    let pixels = par_process_pixels(
        image_width,
        image_height,
        Arc::new(camera),
        Arc::new(world),
        samples_per_pixel,
        max_depth,
        pixel_pb,
    );

    // Output
    let out_file = File::create("./fixtures/gradient.ppm").unwrap();
    let mut writer = BufWriter::new(out_file);

    let progress_bar =
        progress_bars::file_writer(((image_width as f64) * (image_height as f64) * 11.) as usize);

    let total_bytes_written = write_ppm(
        &mut writer,
        (image_height, image_width),
        pixels.into_iter(),
        progress_bar.clone(),
    )
    .unwrap();
    progress_bar.set_position(total_bytes_written as u64);
    progress_bar.finish();
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_something() {}
}
