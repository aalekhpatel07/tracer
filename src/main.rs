use rand::{thread_rng, Rng};
use std::fs::File;
use std::io::BufWriter;
use std::sync::Arc;
use tracer::prelude::*;
use tracer::gen::*;

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
    let render_config = RenderConfig::new(32, 8);

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