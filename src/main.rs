mod ray;
mod vector;

use image::{Rgb, RgbImage};

use ray::{Ray, blue_gradient_vertical};
use vector::{Color, Point, Vector};

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f64 = 1.0;

fn main() {
    let camera_point: Point = Point::new(0.0, 0.0, 0.0);

    let viewport_u: Vector = Vector::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v: Vector = Vector::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    let delta_u: Vector = viewport_u.scale(1.0 / IMAGE_WIDTH as f64);
    let delta_v: Vector = viewport_v.scale(1.0 / IMAGE_HEIGHT as f64);

    let viewport_upper_left = camera_point
        .addv(viewport_u.scale(0.5).negate())
        .addv(viewport_v.scale(0.5).negate())
        .addv(Vector::new(0.0, 0.0, FOCAL_LENGTH).negate());

    let pixel00_loc = viewport_upper_left.addv(delta_u.addv(delta_v).scale(0.5));

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in 0..IMAGE_HEIGHT {
        eprintln!("Scanning line {}.. {} remaining", (i), (IMAGE_HEIGHT - i));

        for j in 0..IMAGE_WIDTH {
            let pixel_centre = pixel00_loc
                .addv(delta_u.scale(j as f64))
                .addv(delta_v.scale(i as f64));

            let ray_direction = pixel_centre.addv(camera_point.negate());

            let ray: Ray = Ray::new(camera_point, ray_direction);
            let color: Color = blue_gradient_vertical(ray);

            img.put_pixel(j, i, Rgb(color.to_color()));
        }
    }

    match img.save("image.png") {
        Ok(_) => println!("Successfully saved"),
        Err(e) => println!("{}", e),
    };
}
