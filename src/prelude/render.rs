use std::sync::Arc;
use rand::{Rng, thread_rng};
use rayon::prelude::*;
use crate::prelude::*;
use crate::prelude::progress_bars::{ProgressBar, ParallelProgressIterator};
// use crate::prelude::vector::{Vec3, LinAlgOp, LinAlgRandGen};
// use crate::prelude::hittable_list::HittableList;
// use crate::prelude::ray::Ray;
// use crate::prelude::hittable::Hittable;
// use crate::prelude::{Camera, Pixel, Scatter};
// use crate::prelude::utils::progress_bars::ProgressBar;


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
    image: Arc<Image>,
    max_depth: isize,
) -> Pixel {
    let mut rng = thread_rng();
    let mut pixel_color: Vec3 = Vec3::new(0., 0., 0.);

    for _ in 0..samples_per_pixel {
        let u = (col as f64 + rng.gen::<f64>()) / (image.width - 1) as f64;
        let v = (row as f64 + rng.gen::<f64>()) / (image.height - 1) as f64;
        let ray: Ray = camera.get_ray(u, v);
        pixel_color += ray_color(&ray, &world, max_depth);
    }

    gamma2_correct(pixel_color / samples_per_pixel as f64, 2).into()
}

/// Process all the pixels in parallel on CPU.
///
pub fn par_process_pixels(
    image: Arc<Image>,
    camera: Arc<Camera>,
    world: Arc<HittableList>,
    samples_per_pixel: usize,
    max_depth: isize,
    progress_bar: ProgressBar,
) -> Vec<Pixel> {
    let rows = 0..image.height;
    let cols = 0..image.width;

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
                image.height - item.0 - 1,
                item.1,
                camera.clone(),
                world.clone(),
                samples_per_pixel,
                image.clone(),
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
