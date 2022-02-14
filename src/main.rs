use rand::{thread_rng, Rng};
use rayon::prelude::*;
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;
use tracer::commons::progress_bars;
use tracer::commons::progress_bars::*;
use tracer::commons::write_ppm;
use tracer::commons::HittableList;
use tracer::commons::Sphere;
use tracer::commons::{Camera, Hittable, LinAlgOp, LinAlgRandGen, Pixel, Point, Ray, Vec3};

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
        // Diffusion parameters: Probability distribution for scattering rays that hit from different angles.

        // Diffuse 1: Probability distribution scales by cos^3(phi).
        // let target: Vec3 = hit_record.point + hit_record.normal + Vec3::random_in_unit_sphere();

        // Diffuse 2: Approximate Lambertian with probability distribution cos(phi). More uniform scatter
        // than 1.
        // let target: Vec3 = hit_record.point + hit_record.normal + Vec3::random_unit_vector();

        // Diffuse 3: Uniform scatter for all points away from hit-point, independent of the normal angle.
        // let target: Vec3 = hit_record.point + Vec3::random_in_hemisphere(hit_record.normal);

        // Let's use Diffuse 2.
        let target: Vec3 = hit_record.point + hit_record.normal + Vec3::random_unit_vector();

        let new_ray = Ray::new(&hit_record.point, &(target - hit_record.point));
        return 0.5 * ray_color(&new_ray, world, depth - 1);
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

    // Some objects: Spheres
    let sphere_1: Arc<dyn Hittable> = Arc::new(Sphere::new(Point::new(0., 0., -1.), 0.5));
    let sphere_2: Arc<dyn Hittable> = Arc::new(Sphere::new(Point::new(0., -100.5, -1.), 100.));

    world.push(sphere_1.clone());
    world.push(sphere_2.clone());
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
    let viewport_height = 2.0;
    let focal_length = 1.0;
    let origin = Point::new(0., 0., 0.);
    let camera = Camera::new(origin, aspect_ratio, viewport_height, focal_length);

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
