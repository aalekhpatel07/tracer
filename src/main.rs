use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;
use tracer::prelude::*;

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
    // Scene
    let world = create_random_world();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;

    let screen = Image::new(image_width, aspect_ratio);

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

    // Render.
    let samples_per_pixel: usize = 100;
    let max_depth: isize = 100;

    // Progress bar. Draw every 1% to prevent frequent Rwlock-ing.
    let pixel_pb = progress_bars::default(screen.width * screen.height);
    pixel_pb.set_draw_delta((screen.width as u64 * screen.height as u64) / 100);

    let pixels = par_process_pixels(
        Arc::new(screen.clone()),
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
        progress_bars::file_writer(((screen.width as f64) * (screen.height as f64) * 11.) as usize);

    let total_bytes_written = write_ppm(
        &mut writer,
        (screen.height, screen.width),
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
