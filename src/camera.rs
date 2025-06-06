use image::{Rgb, RgbImage};

use crate::{
    objects::hittable::{Hittable, HittableList},
    ray::{Ray, blue_gradient_vertical},
    utils::{constants::F_INF, functions::random_double, interval::Interval},
    vector::{Color, Point, Vector},
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,

    focal_length: f64,
    viewport_width: f64,
    viewport_height: f64,

    centre: Point,
    pixel100_loc: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,

    samples_per_pixel: u32,
}

impl Camera {
    pub fn render(&self, world: HittableList) {
        let mut img = RgbImage::new(self.image_width, self.image_height);

        for i in 0..self.image_height {
            eprintln!(
                "Scanning line {}.. {} remaining",
                (i),
                (self.image_height - i)
            );

            for j in 0..self.image_width {
                let mut pixel_color: Color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray: Ray = self.get_ray(i, j);
                    let color: Color = self.ray_color(ray, &world);
                    pixel_color = pixel_color.addv(color);
                }

                pixel_color = pixel_color.scale(1.0 / self.samples_per_pixel as f64);
                img.put_pixel(j, i, Rgb(pixel_color.to_color()));
            }
        }

        match img.save("image.png") {
            Ok(_) => println!("Successfully saved"),
            Err(e) => println!("{}", e),
        };
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let sample_square: Vector = self.sample_square();
        let (offset_x, offset_y, _) = sample_square.get_point();

        let sample_pixel_centre = self
            .pixel100_loc
            .addv(self.pixel_delta_u.scale(j as f64 + offset_x))
            .addv(self.pixel_delta_v.scale(i as f64 + offset_y));

        let ray_direction = sample_pixel_centre.subv(self.centre);

        Ray::new(self.centre, ray_direction)
    }

    fn sample_square(&self) -> Vector {
        Vector::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(&self, ray: Ray, world: &HittableList) -> Color {
        let world_interval: Interval = Interval::new(0.0, F_INF);
        let hit_record = world.hit(&ray, &world_interval);

        let intensity = Interval::new(0.000, 0.999);

        match hit_record {
            Some(hit) => {
                let surface_normal_vec = hit.get_normal();
                let (x, y, z) = surface_normal_vec.get_point();

                // let r = 256.0 * intensity.clamp(x);
                // let g = 256.0 * intensity.clamp(y);
                // let b = 256.0 * intensity.clamp(z);
                // Color::new(r, g, b)

                // NOTE: normalised so all x, y, and z in the range of [-1.0, 1.0]
                // so, add 1 to move the range to [0.0, 2.0]
                // and scale by 0.5 to get a range of [0.0, 1.0]
                // which valid rgb values have to lie within
                Color::new(x + 1.0, y + 1.0, z + 1.0).scale(0.5)
            }
            None => blue_gradient_vertical(ray),
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width: u32 = 400;
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let centre = Point::new(0.0, 0.0, 0.0);
        let viewport_u: Vector = Vector::new(viewport_width, 0.0, 0.0);
        let viewport_v: Vector = Vector::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u: Vector = viewport_u.scale(1.0 / image_width as f64);
        let pixel_delta_v: Vector = viewport_v.scale(1.0 / image_height as f64);

        let viewport_upper_left = centre
            .subv(viewport_u.scale(0.5))
            .subv(viewport_v.scale(0.5))
            .subv(Vector::new(0.0, 0.0, focal_length));

        let pixel00_loc: Point =
            viewport_upper_left.addv(pixel_delta_u.addv(pixel_delta_v).scale(0.5));

        let samples_per_pixel: u32 = 100;

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            focal_length,
            viewport_width,
            viewport_height,
            centre,
            pixel100_loc: pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
        }
    }
}
