use image::{Rgb, RgbImage};

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in 0..IMAGE_HEIGHT {
        eprintln!("Scanning line {}.. {} remaining", (i), (IMAGE_HEIGHT - i));

        for j in 0..IMAGE_WIDTH {
            let r = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = i as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.0;

            // NOTE: We multiply by 255.99 specifically here so as to ensure that values close to
            // 255 are mapped to 255 instead of rounded down to 254
            // E.g. ir = (255.0 * 0.999999) = 254.999745 => 254
            let ir = (255.99 * r) as u8;
            let ig = (255.99 * g) as u8;
            let ib = (255.99 * b) as u8;

            img.put_pixel(i, j, Rgb([ir, ig, ib]));
        }
    }

    match img.save("image.png") {
        Ok(_) => println!("Successfully saved"),
        Err(e) => println!("{}", e),
    };
}
