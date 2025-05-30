mod objects;
mod ray;
mod vector;

use image::{Rgb, RgbImage};

use objects::sphere::Sphere;
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
        .subv(viewport_u.scale(0.5))
        .subv(viewport_v.scale(0.5))
        .subv(Vector::new(0.0, 0.0, FOCAL_LENGTH));

    let pixel00_loc = viewport_upper_left.addv(delta_u.addv(delta_v).scale(0.5));

    let mut img = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    let sphere: Sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);

    for i in 0..IMAGE_HEIGHT {
        eprintln!("Scanning line {}.. {} remaining", (i), (IMAGE_HEIGHT - i));

        for j in 0..IMAGE_WIDTH {
            let pixel_centre = pixel00_loc
                .addv(delta_u.scale(j as f64))
                .addv(delta_v.scale(i as f64));

            let ray_direction = pixel_centre.addv(camera_point.negate());

            let ray: Ray = Ray::new(camera_point, ray_direction);
            let t = sphere.hit_at(&ray);

            if t > 0.0 {
                let surface_vec = ray.at(t);

                let surface_normal_vec = surface_vec.subv(sphere.get_centre()).unit();
                let (x, y, z) = surface_normal_vec.get_point();

                // NOTE: Normalised so all x, y, and z in the range of [-1.0, 1.0]
                // so, add 1 to move the range to [0.0, 2.0]
                // and scale by 0.5 to get a range of [0.0, 1.0]
                // which valid RGB values have to lie within
                let color: Color = Color::new(x + 1.0, y + 1.0, z + 1.0).scale(0.5);
                img.put_pixel(j, i, Rgb(color.to_color()));
            } else {
                let color: Color = blue_gradient_vertical(ray);
                img.put_pixel(j, i, Rgb(color.to_color()));
            }
        }
    }

    match img.save("image.png") {
        Ok(_) => println!("Successfully saved"),
        Err(e) => println!("{}", e),
    };
}
