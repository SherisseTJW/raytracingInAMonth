mod vector;

use image::{Rgb, RgbImage};

use vector::Vector;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in 0..IMAGE_HEIGHT {
        eprintln!("Scanning line {}.. {} remaining", (i), (IMAGE_HEIGHT - i));

        for j in 0..IMAGE_WIDTH {
            let color_vec = Vector::new(
                j as f64 / (IMAGE_WIDTH - 1) as f64,
                i as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.0,
            );

            let color = color_vec.to_color();

            img.put_pixel(i, j, Rgb(color));
        }
    }

    match img.save("image.png") {
        Ok(_) => println!("Successfully saved"),
        Err(e) => println!("{}", e),
    };
}
