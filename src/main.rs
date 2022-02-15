use rand::{thread_rng, Rng};
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

pub fn create_random_world_complex() -> HittableList {
    let mut rng = thread_rng();

    let mut world = HittableList::new();

    let ground_material = Material::Lambertian {
        albedo: Vec3::new(0.5, 0.5, 0.5),
    };

    let ground_sphere = Sphere::new(Point::new(0., -1000., 0.), 1000., Arc::new(ground_material));

    world.push(Arc::new(ground_sphere));

    for a in -11..11 {
        for b in -11..11 {
            let rand_x: f64 = rng.gen();
            let rand_z: f64 = rng.gen();

            let center = Point::new(a as f64 + 0.9 * rand_x, 0.2, b as f64 + 0.9 * rand_z);

            if (center - Point::new(4., 0.2, 0.)).norm() > 0.9 {
                let sphere_material = {
                    let choice_of_material: f64 = rng.gen();

                    if choice_of_material < 0.8 {
                        // diffuse: lambertian (matte)
                        Material::Lambertian {
                            albedo: Vec3::rand_uniform(0., 1.) * Vec3::rand_uniform(0., 1.),
                        }
                    } else if choice_of_material < 0.95 {
                        // metal
                        Material::Metal {
                            albedo: Vec3::rand_uniform(0.5, 1.),
                            fuzz: rng.gen_range((0.)..(0.5)),
                        }
                    } else {
                        // glass : dielectric
                        Material::Dielectric {
                            index_of_refraction: 1.5,
                        }
                    }
                };

                world.push(Arc::new(Sphere::new(
                    center,
                    0.2,
                    Arc::new(sphere_material),
                )));
            }
        }
    }

    let material_1 = Arc::new(Material::Dielectric {
        index_of_refraction: 1.5,
    });
    let material_2 = Arc::new(Material::Lambertian {
        albedo: Vec3::new(0.4, 0.2, 0.1),
    });
    let material_3 = Arc::new(Material::Metal {
        albedo: Vec3::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });

    world.push(Arc::new(Sphere::new(
        Point::new(0., 1., 0.),
        1.,
        material_1,
    )));
    world.push(Arc::new(Sphere::new(
        Point::new(-4., 1., 0.),
        1.,
        material_2,
    )));
    world.push(Arc::new(Sphere::new(
        Point::new(4., 1., 0.),
        1.,
        material_3,
    )));

    world
}

fn main() {
    // Scene
    let world = create_random_world_complex();

    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width: usize = 1200;

    let screen = Image::new(image_width, aspect_ratio);

    // Camera
    let look_from = Point::new(13., 2., 3.);
    let look_at = Point::new(0., 0., 0.);
    let view_up = Vec3::new(0., 1., 0.);
    let vertical_field_of_view = Angle::Degrees(Degrees(20.0));
    let aperture = 0.1;
    let focus_distance = 10.0;

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
    let render_config = RenderConfig::new(500, 10);

    // Progress bar.
    // Draw every 1% to prevent frequent Rwlock-ing.
    let pixel_pb = progress_bars::default(screen.width * screen.height);
    pixel_pb.set_draw_delta((screen.width as u64 * screen.height as u64) / 100);

    let pixels = process_pixels(
        Arc::new(screen.clone()),
        Arc::new(camera),
        Arc::new(world),
        Arc::new(render_config),
        pixel_pb,
    );

    // Output
    let out_file = File::create("./static/complex_scene.ppm").unwrap();
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
