use std::sync::Arc;

use crate::{
    bvh::aabb::Aabb,
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
    utils::{constants::F_INF, functions::degrees_to_radians, interval::Interval},
    vector::{Point, Vector},
};

// NOTE: For now, only y-rotation as in the book is supported
// TODO: Supprt the other rotations
#[derive(Clone)]
pub struct Rotation {
    hittable: Arc<dyn Hittable>,
    x_rotation: f64,
    y_rotation: f64,
    z_rotation: f64,
    bounding_box: Aabb,
}

impl Rotation {
    pub fn new(
        hittable: Arc<dyn Hittable>,
        x_rotation: f64,
        y_rotation: f64,
        z_rotation: f64,
    ) -> Rotation {
        let radians = degrees_to_radians(y_rotation);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let hittable_bounding_box = hittable.get_aabb();
        let (x_min, x_max) = hittable_bounding_box.get_axis_interval(0).get_min_max();
        let (y_min, y_max) = hittable_bounding_box.get_axis_interval(1).get_min_max();
        let (z_min, z_max) = hittable_bounding_box.get_axis_interval(2).get_min_max();

        let mut min_x = F_INF;
        let mut min_y = F_INF;
        let mut min_z = F_INF;
        let mut max_x = -F_INF;
        let mut max_y = -F_INF;
        let mut max_z = -F_INF;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * x_max + (1 - i) as f64 * x_min;
                    let y = j as f64 * y_max + (1 - j) as f64 * y_min;
                    let z = k as f64 * z_max + (1 - k) as f64 * z_min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    min_x = f64::min(min_x, new_x);
                    min_y = f64::min(min_y, y);
                    min_z = f64::min(min_z, new_z);

                    max_x = f64::max(max_x, new_x);
                    max_y = f64::max(max_y, y);
                    max_z = f64::max(max_z, new_z);
                }
            }
        }

        let bounding_box = Aabb::new_from_extrema_points(
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        );

        Rotation {
            hittable,
            x_rotation,
            y_rotation,
            z_rotation,
            bounding_box,
        }
    }
}

impl Hittable for Rotation {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let radians = degrees_to_radians(self.y_rotation);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let (x_origin, y_origin, z_origin) = ray.get_origin().get_point();
        let (x_dir, y_dir, z_dir) = ray.get_direction().get_point();

        let new_origin = Vector::new(
            (x_origin * cos_theta) - (z_origin * sin_theta),
            y_origin,
            (x_origin * sin_theta) + (z_origin * cos_theta),
        );

        let new_direction = Vector::new(
            (x_dir * cos_theta) - (z_dir * sin_theta),
            y_dir,
            (x_dir * sin_theta) + (z_dir * cos_theta),
        );

        let new_ray = Ray::new(new_origin, new_direction, Some(ray.get_time()));

        match self.hittable.hit(&new_ray, interval) {
            Some(mut hit) => {
                hit.rotate(sin_theta, cos_theta);
                Some(hit)
            }
            None => None,
        }
    }

    fn get_aabb(&self) -> Aabb {
        self.bounding_box
    }

    fn clone_box(&self) -> Arc<dyn Hittable> {
        Arc::new(self.clone())
    }
}
