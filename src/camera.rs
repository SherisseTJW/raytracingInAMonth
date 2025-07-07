use image::{Rgb, RgbImage};
use rayon::ThreadPoolBuilder;
use rayon::prelude::*;
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use crate::ray;
use crate::utils::functions::degrees_to_radians;
use crate::vector::cross_product;
use crate::vector::get_random_vector_in_unit_disk;
use crate::{
    materials::{
        Materials,
        scatterable::{ScatterRecord, Scatterable},
    },
    objects::hittable::{Hittable, HittableList},
    ray::{Ray, blue_gradient_vertical},
    utils::{constants::F_INF, functions::random_double, interval::Interval},
    vector::{Color, Point, Vector},
};

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,

    // focal_length: f64,
    vertical_fov: f64,

    // NOTE: Unit basis vectors for camera coordinate frame
    u: Vector,
    v: Vector,
    w: Vector,

    defocus_angle: f64,
    focus_dist: f64,
    defocus_disk_u: Vector,
    defocus_disk_v: Vector,

    viewport_width: f64,
    viewport_height: f64,

    centre: Point,
    pixel00_loc: Point,
    pixel_delta_u: Vector,
    pixel_delta_v: Vector,

    samples_per_pixel: u32,
    max_depth: u32,
}

impl Camera {
    pub fn override_image_specs(&self, aspect_ratio: f64, image_width: u32) -> Camera {
        let centre = self.centre;
        let u = self.u;
        let v = self.v;
        let w = self.w;
        let vertical_fov = self.vertical_fov;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;
        let defocus_angle = self.defocus_angle;
        let focus_dist = self.focus_dist;
        let defocus_disk_u = self.defocus_disk_u;
        let defocus_disk_v = self.defocus_disk_v;

        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

        let theta_rad: f64 = degrees_to_radians(vertical_fov);
        let height: f64 = f64::tan(theta_rad / 2.0);

        let viewport_height: f64 = 2.0 * height * focus_dist;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let viewport_u: Vector = u.scale(viewport_width);
        let viewport_v: Vector = v.negate().scale(viewport_height);

        let pixel_delta_u: Vector = viewport_u.scale(1.0 / image_width as f64);
        let pixel_delta_v: Vector = viewport_v.scale(1.0 / image_height as f64);

        let viewport_upper_left = centre
            .subv(viewport_u.scale(0.5))
            .subv(viewport_v.scale(0.5))
            .subv(w.scale(focus_dist));

        let pixel00_loc: Point =
            viewport_upper_left.addv(pixel_delta_u.addv(pixel_delta_v).scale(0.5));

        Camera {
            aspect_ratio,
            image_width,
            image_height,

            vertical_fov,

            u,
            v,
            w,

            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,

            viewport_width,
            viewport_height,

            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,

            samples_per_pixel,
            max_depth,
        }
    }

    pub fn override_camera_pos(
        &self,
        look_from: Vector,
        look_at: Vector,
        v_up: Vector,
        vertical_fov: f64,
        defocus_angle: f64,
        focus_dist: f64,
    ) -> Camera {
        let aspect_ratio = self.aspect_ratio;
        let image_width = self.image_width;
        let image_height = self.image_height;
        let samples_per_pixel = self.samples_per_pixel;
        let max_depth = self.max_depth;

        let w = look_from.subv(look_at).unit();
        let u = cross_product(v_up, w).unit();
        let v = cross_product(w, u);

        let defocus_radius: f64 = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u: Vector = u.scale(defocus_radius);
        let defocus_disk_v: Vector = v.scale(defocus_radius);

        let theta_rad: f64 = degrees_to_radians(vertical_fov);
        let height: f64 = f64::tan(theta_rad / 2.0);

        let viewport_height: f64 = 2.0 * height * focus_dist;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let centre = look_from;
        let viewport_u: Vector = u.scale(viewport_width);
        let viewport_v: Vector = v.negate().scale(viewport_height);

        let pixel_delta_u: Vector = viewport_u.scale(1.0 / image_width as f64);
        let pixel_delta_v: Vector = viewport_v.scale(1.0 / image_height as f64);

        let viewport_upper_left = centre
            .subv(viewport_u.scale(0.5))
            .subv(viewport_v.scale(0.5))
            .subv(w.scale(focus_dist));

        let pixel00_loc: Point =
            viewport_upper_left.addv(pixel_delta_u.addv(pixel_delta_v).scale(0.5));

        Camera {
            aspect_ratio,
            image_width,
            image_height,

            vertical_fov,

            u,
            v,
            w,

            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,

            viewport_width,
            viewport_height,

            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,

            samples_per_pixel,
            max_depth,
        }
    }

    pub fn override_sampling_specs(&self, samples_per_pixel: u32, max_depth: u32) -> Camera {
        let aspect_ratio = self.aspect_ratio;
        let image_width = self.image_width;
        let image_height = self.image_height;
        let vertical_fov = self.vertical_fov;
        let u = self.u;
        let v = self.v;
        let w = self.w;
        let viewport_width = self.viewport_width;
        let viewport_height = self.viewport_height;
        let centre = self.centre;
        let pixel00_loc = self.pixel00_loc;
        let pixel_delta_u = self.pixel_delta_u;
        let pixel_delta_v = self.pixel_delta_v;
        let defocus_angle = self.defocus_angle;
        let focus_dist = self.focus_dist;
        let defocus_disk_u = self.defocus_disk_u;
        let defocus_disk_v = self.defocus_disk_v;

        Camera {
            aspect_ratio,
            image_width,
            image_height,

            vertical_fov,

            u,
            v,
            w,

            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,

            viewport_width,
            viewport_height,

            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,

            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(&self, world: HittableList) {
        ThreadPoolBuilder::new()
            .num_threads(6)
            .build_global()
            .unwrap();

        let img = std::sync::Mutex::new(RgbImage::new(self.image_width, self.image_height));
        let lines_done = Arc::new(AtomicUsize::new(0));

        (0..self.image_height).into_par_iter().for_each(|i| {
            for j in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    let color = Camera::ray_color(ray, &world, self.max_depth);
                    pixel_color = pixel_color.addv(color);
                }

                pixel_color = pixel_color.scale(1.0 / self.samples_per_pixel as f64);
                let mut img_lock = img.lock().unwrap();
                img_lock.put_pixel(j, i, Rgb(pixel_color.to_color()));
            }

            let done = lines_done.fetch_add(1, Ordering::Relaxed) + 1;
            eprintln!(
                "Current Progress: {}/{} lines done",
                done, self.image_height
            );
        });

        // Save the image after all threads finish
        match img.into_inner().unwrap().save("image.png") {
            Ok(_) => println!("successfully saved"),
            Err(e) => println!("{}", e),
        };
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let sample_square: Vector = self.sample_square();
        let (offset_x, offset_y, _) = sample_square.get_point();

        let sample_pixel_centre = self
            .pixel00_loc
            .addv(self.pixel_delta_u.scale(j as f64 + offset_x))
            .addv(self.pixel_delta_v.scale(i as f64 + offset_y));

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.centre
        } else {
            self.defocus_disk_sample()
        };

        let ray_direction = sample_pixel_centre.subv(ray_origin);
        let ray_time = random_double();

        Ray::new(ray_origin, ray_direction, Some(ray_time))
    }

    fn defocus_disk_sample(&self) -> Point {
        let point = get_random_vector_in_unit_disk();
        let (x, y, _) = point.get_point();

        self.centre
            .addv(self.defocus_disk_u.scale(x))
            .addv(self.defocus_disk_v.scale(y))
    }

    fn sample_square(&self) -> Vector {
        Vector::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(ray: Ray, world: &HittableList, depth: u32) -> Color {
        if depth == 0 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            let world_interval: Interval = Interval::new(0.001, F_INF);
            let hit_record = world.hit(&ray, &world_interval);

            match hit_record {
                Some(hit) => {
                    let material: Materials = hit.get_material();
                    let scatter_record: Option<ScatterRecord> = material.scatter(ray, hit);

                    match scatter_record {
                        Some(scatter) => Camera::ray_color(scatter.get_ray(), world, depth - 1)
                            .multiply(scatter.get_attenuation()),
                        None => Color::new(0.0, 0.0, 0.0),
                    }
                }
                None => blue_gradient_vertical(ray),
            }
        }
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width: u32 = 400;
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;

        // NOTE: Default camera will be straight on
        let look_from = Point::new(0.0, 0.0, 1.0);
        let look_at = Point::new(0.0, 0.0, -1.0);
        let v_up = Point::new(0.0, 1.0, 0.0);

        let vertical_fov = 40.0;

        let w = look_from.subv(look_at).unit();
        let u = cross_product(v_up, w).unit();
        let v = cross_product(w, u);

        let theta_rad: f64 = degrees_to_radians(vertical_fov);
        let height: f64 = f64::tan(theta_rad / 2.0);

        let defocus_angle = 0.0;
        let focus_dist = 2.0;

        let defocus_radius: f64 = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u: Vector = u.scale(defocus_radius);
        let defocus_disk_v: Vector = v.scale(defocus_radius);

        let viewport_height: f64 = 2.0 * height * focus_dist;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        let centre = look_from;
        let viewport_u: Vector = u.scale(viewport_width);
        let viewport_v: Vector = v.negate().scale(viewport_height);

        let pixel_delta_u: Vector = viewport_u.scale(1.0 / image_width as f64);
        let pixel_delta_v: Vector = viewport_v.scale(1.0 / image_height as f64);

        let viewport_upper_left = centre
            .subv(viewport_u.scale(0.5))
            .subv(viewport_v.scale(0.5))
            .subv(w.scale(focus_dist));

        let pixel00_loc: Point =
            viewport_upper_left.addv(pixel_delta_u.addv(pixel_delta_v).scale(0.5));

        let samples_per_pixel: u32 = 250;
        let max_depth: u32 = 50;

        Camera {
            aspect_ratio,
            image_width,
            image_height,

            vertical_fov,

            u,
            v,
            w,

            defocus_angle,
            focus_dist,
            defocus_disk_u,
            defocus_disk_v,

            viewport_width,
            viewport_height,

            centre,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,

            samples_per_pixel,
            max_depth,
        }
    }
}
