use image::{GenericImageView, ImageReader, RgbImage};
use std::{fmt::Display, sync::Arc};

use crate::{
    texture::texture::Texture,
    utils::interval::Interval,
    vector::{Color, Point},
};

pub struct ImageTexture {
    image: Arc<RgbImage>,
    normalised_interval: Interval,
}

impl ImageTexture {
    pub fn new(image_filepath: &str) -> ImageTexture {
        let image = match ImageReader::open(image_filepath) {
            Ok(raw_img) => match raw_img.decode() {
                Ok(decoded_img) => {
                    let (width, height): (u32, u32) = decoded_img.dimensions();
                    if width <= 0 || height <= 0 {
                        panic!(
                            "Invalid image dimensions at {} for image texture",
                            image_filepath
                        )
                    }

                    decoded_img.to_rgb8()
                }
                Err(err) => panic!(
                    "Could not decode image at {} for image texture\n{}",
                    image_filepath, err
                ),
            },
            Err(err) => panic!(
                "Could not open image at {} for image texture\n{}",
                image_filepath, err
            ),
        };

        ImageTexture {
            image: Arc::new(image),
            normalised_interval: Interval::new(0.0, 1.0),
        }
    }
}

impl Texture for ImageTexture {
    fn get_value(&self, u: f64, v: f64, _point: Point) -> Color {
        let (width, height): (u32, u32) = self.image.dimensions();

        let clamped_u = self.normalised_interval.clamp(u);
        let clamped_v = 1.0 - self.normalised_interval.clamp(v);

        let i = (clamped_u * width as f64) as u32;
        let j = (clamped_v * height as f64) as u32;

        let pixel_color = self.image.get_pixel(i, j).0;
        let color_scale = 1.0 / 255.0;

        let r = pixel_color[0] as f64 * color_scale;
        let g = pixel_color[1] as f64 * color_scale;
        let b = pixel_color[2] as f64 * color_scale;

        Color::new(r, g, b)
    }
}

impl Display for ImageTexture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ImageTexture")
    }
}
