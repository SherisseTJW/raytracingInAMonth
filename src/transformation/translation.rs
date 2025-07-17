use std::sync::Arc;

use crate::{
    bvh::aabb::Aabb,
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
    utils::interval::Interval,
    vector::Vector,
};

#[derive(Clone)]
pub struct Translation {
    hittable: Arc<dyn Hittable>,
    offset: Vector,
    bounding_box: Aabb,
}

impl Translation {
    pub fn new(hittable: Arc<dyn Hittable>, offset: Vector) -> Translation {
        let bounding_box = hittable.get_aabb().translate(offset);

        Translation {
            hittable,
            offset,
            bounding_box,
        }
    }
}

impl Hittable for Translation {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let offset_ray: Ray = Ray::new(
            ray.get_origin().subv(self.offset),
            ray.get_direction(),
            Some(ray.get_time()),
        );

        match self.hittable.hit(&offset_ray, interval) {
            Some(mut hit) => Some(hit.translate(&offset_ray, self.offset)),
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
