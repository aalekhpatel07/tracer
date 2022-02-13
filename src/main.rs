use std::fs::File;
use std::io::{BufWriter, stdout};
use std::sync::Arc;
// use crossbeam_channel::{unbounded, Receiver, Sender};
use tracer::commons::{Vec3, Point, Pixel, Ray, LinAlgOp, Hittable};
use tracer::commons::write_ppm;
use tracer::commons::progress_bars;
use tracer::commons::Sphere;
use tracer::commons::HittableList;



pub fn interpolate_linear(start: Vec3, end: Vec3, time: f64) -> Vec3 {
    (1.0 - time) * start + time * end
}

pub fn ray_color(ray: &Ray, world: &HittableList) -> Pixel {

    if let Some(hit_record) = world.hit(ray, 0., f64::INFINITY) {
        return (0.5 * (hit_record.normal + [1., 1., 1.].into())).into();
    }

    let unit_vector_in_direction_of_ray = ray.direction.unit_vector();
    let time = 0.5 * (unit_vector_in_direction_of_ray.1 + 1.);

    interpolate_linear(
        Vec3::new(1., 1., 1.), // White
        Vec3::new(0.5, 0.7, 1.0), // Blue
        time,
    ).into()
}


fn main() {

    let mut world = HittableList::new();


    // Sphere
    let sphere_1: Arc<Box<dyn Hittable>> = Arc::new(Box::new(Sphere::new(Point::new(0., 0., -1.), 0.5)));
    let sphere_2: Arc<Box<dyn Hittable>> = Arc::new(Box::new(Sphere::new(Point::new(0., -100.5, -1.), 100.)));

    world.push(sphere_1.clone());
    world.push(sphere_2.clone());

    // println!("{}", sphere_1);

    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio).round() as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);

    let lower_left_corner = origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    // Render

    let mut pixels = vec![];

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let u = (i as f64) / (image_width - 1) as f64;
            let v = (j as f64) / (image_height - 1) as f64;
            let ray: Ray = Ray::new(&origin, &(lower_left_corner + u * horizontal + v * vertical - origin));

            pixels.push(ray_color(&ray, &world));
        }
    }

    // let stdout = stdout();
    let out_file = File::create("./fixtures/gradient.ppm").unwrap();
    let mut writer = BufWriter::new(out_file);

    let progress_bar = progress_bars::file_writer(((image_width as f64) * (image_height as f64) * 11.) as usize);

    write_ppm(&mut writer, (image_height, image_width), pixels.into_iter(), progress_bar);

}

#[cfg(test)]
mod tests {

    #[test]
    fn test_something() {

    }
}