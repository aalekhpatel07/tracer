pub mod prelude;

pub mod gen {
    use crate::prelude::*;
    use std::sync::Arc;
    use rand::{thread_rng, Rng};


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
}
