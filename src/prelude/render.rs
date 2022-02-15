use crate::prelude::*;
use rand::{thread_rng, Rng};
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct RenderConfig {
    pub samples_per_pixel: usize,
    pub max_depth: isize,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self {
            samples_per_pixel: 100,
            max_depth: 100,
        }
    }
}

impl RenderConfig {
    pub fn new(samples_per_pixel: usize, max_depth: isize) -> Self {
        Self {
            samples_per_pixel,
            max_depth,
        }
    }
}

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

// Export the parallel pixel processor if feature `parallel` is enabled (default).
#[cfg(not(feature = "parallel"))]
pub use process_pixels_factory::process_pixels_seq as process_pixels;

// Export the sequential pixel processor if feature `parallel` is disabled.
#[cfg(feature = "parallel")]
pub use process_pixels_factory::process_pixels_par as process_pixels;

mod process_pixels_factory {
    use super::process_pixel;
    use crate::prelude::*;
    use std::sync::Arc;

    use crate::prelude::progress_bars::ProgressBar;

    #[cfg(feature = "parallel")]
    use crate::prelude::progress_bars::ParallelProgressIterator;

    #[cfg(feature = "parallel")]
    use rayon::prelude::*;

    #[cfg(feature = "parallel")]
    pub fn process_pixels_par(
        image: Arc<Image>,
        camera: Arc<Camera>,
        world: Arc<HittableList>,
        render_config: Arc<RenderConfig>,
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
                    render_config.samples_per_pixel,
                    image.clone(),
                    render_config.max_depth,
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

    #[cfg(not(feature = "parallel"))]
    pub fn process_pixels_seq(
        image: Arc<Image>,
        camera: Arc<Camera>,
        world: Arc<HittableList>,
        render_config: Arc<RenderConfig>,
        progress_bar: ProgressBar,
    ) -> Vec<Pixel> {
        let rows = 0..image.height;
        let cols = 0..image.width;

        let cross: Arc<Vec<(usize, usize)>> = Arc::new(
            rows.flat_map(|row| cols.clone().map(move |col| (row, col)))
                .collect::<Vec<(usize, usize)>>(),
        );

        cross
            .iter()
            // .progress_with(progress_bar)
            .map(|item: &(usize, usize)| {
                let value = process_pixel(
                    image.height - item.0 - 1,
                    item.1,
                    camera.clone(),
                    world.clone(),
                    render_config.samples_per_pixel,
                    image.clone(),
                    render_config.max_depth,
                );
                progress_bar.inc(1);
                value
            })
            .collect::<Vec<Pixel>>()
    }
}
