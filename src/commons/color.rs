use std::io::Write;
use std::io::Result;


pub type Pixel = (u8, u8, u8);


impl Into<Pixel> for Vec3 {
    fn into(self) -> Pixel {
        let x = self.0.clamp(0., 1.);
        let y = self.1.clamp(0., 1.);
        let z = self.2.clamp(0., 1.);

        Pixel::from(
            (
                (255.999 * x).round() as u8,
                (255.999 * y).round() as u8,
                (255.999 * z).round() as u8,
            )
        )
    }
}

impl Into<Vec3> for Pixel {
    fn into(self) -> Vec3 {
        let x = (self.0 as f64) / 255.999;
        let y = (self.1 as f64) / 255.999;
        let z = (self.2 as f64) / 255.999;

        Vec3::new(x, y, z)
    }
}

use progress_bars::*;
use crate::commons::Vec3;

pub fn write_pixel<W: Write>(writer: &mut W, pixel: Pixel) -> Result<usize> {

    let mut total_bytes_written: usize = 0;

    total_bytes_written += writer.write(pixel.0.to_string().as_bytes())?;
    total_bytes_written += writer.write(b" ")?;
    total_bytes_written += writer.write(pixel.1.to_string().as_bytes())?;
    total_bytes_written += writer.write(b" ")?;
    total_bytes_written += writer.write(pixel.2.to_string().as_bytes())?;
    total_bytes_written += writer.write(b"\n")?;

    Ok(total_bytes_written)
}

pub fn write_ppm<W: Write, I: Iterator<Item=Pixel>>(
    writer: &mut W,
    (height, width): (usize, usize),
    pixels: I,
    progress_bar: ProgressBar
) -> Result<usize> {

    let mut total_bytes_written: usize = 0;

    total_bytes_written += writer.write(b"P3\n")?;
    total_bytes_written += writer.write(width.to_string().as_bytes())?;
    total_bytes_written += writer.write(b" ")?;
    total_bytes_written += writer.write(height.to_string().as_bytes())?;
    total_bytes_written += writer.write(b"\n255\n")?;

    for pixel in pixels {
        total_bytes_written += write_pixel(writer, pixel)?;
        progress_bar.set_position(total_bytes_written as u64);
    }

    Ok(total_bytes_written)
}

pub mod progress_bars {
    pub use indicatif::{ProgressBar, ProgressStyle};

    pub fn file_writer(expected_size: usize) -> ProgressBar {
        let progress_style =
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .progress_chars("#>-");

        let mut progress_bar = ProgressBar::new(
            expected_size as u64,
        );

        progress_bar.with_style(progress_style)
    }

    pub fn hidden() -> ProgressBar {
        ProgressBar::hidden()
    }
}

fn create_rainbow_color<C: From<Vec<Pixel>>>(image_height: usize, image_width: usize) -> C {
    let mut pixels: Vec<Pixel> = vec![];

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let red: f64 = (i as f64) / (image_width as f64 - 1.);
            let green: f64 = (j as f64) / (image_height as f64 - 1.);
            let blue: f64 = 0.25;

            let pixel: Pixel = (
                (255.999 * red).round() as u8,
                (255.999 * green).round() as u8,
                (255.999 * blue).round() as u8,
            );
            pixels.push(pixel);
        }
    }

    C::from(pixels)
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufWriter, Result};
    use super::{write_ppm, Pixel, create_rainbow_color};
    use super::progress_bars;
    use crossbeam_channel::{unbounded, Sender, Receiver};
    use crate::commons::Vec3;

    #[test]
    fn test_rainbow_256x256_write_to_file() -> Result<()> {

        let image_height: usize = 256;
        let image_width: usize = 256;

        let out_file = File::create(
            format!("./fixtures/rainbow_{}x{}.ppm", image_height, image_width)
        )?;
        let mut writer = BufWriter::new(out_file);

        let progress_bar = progress_bars::file_writer(
            (image_width as f64 * image_height as f64 * 10.14).round() as usize
        );

        let total_bytes_written = write_ppm(
            &mut writer,
            (image_height, image_width),
            create_rainbow_color::<Vec<Pixel>>(image_height, image_width).into_iter(),
            progress_bar.clone()
        )?;

        progress_bar.set_length(total_bytes_written as u64);
        progress_bar.set_position(total_bytes_written as u64);
        progress_bar.finish();

        Ok(())
    }

    #[test]
    fn convert_from_vec3 () {
        let v = Vec3::new(0.5, 0.3, 0.2);
        let expected = Pixel::from((128, 77, 51));
        let observed: Pixel = v.into();
        assert_eq!(observed, expected);
    }

    #[test]
    fn convert_into_vec3 () {
        let pixel = Pixel::from((128, 77, 51));
        let expected = Vec3::new(0.5, 0.3, 0.2);
        let observed: Vec3 = pixel.into();
        let tolerance: f64 = 1e-2;
        assert!((observed - expected).norm() <= tolerance, "Observed {} and expected {} much more different than allowed tolerance of {}", observed, expected, tolerance);
    }
}
