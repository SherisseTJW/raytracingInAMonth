use std::sync::Arc;

use crate::{
    bvh::aabb::Aabb,
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
    utils::{constants::F_INF, functions::degrees_to_radians, interval::Interval},
    vector::{Point, Vector},
};

// NOTE: For now, only y-rotation as in the book is supported
// TODO: Support the other rotations
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
        let bounding_box = hittable
            .get_aabb()
            .rotate(x_rotation, y_rotation, z_rotation);

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
                let rotated_hit_record = hit.rotate(sin_theta, cos_theta);
                Some(rotated_hit_record)
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
