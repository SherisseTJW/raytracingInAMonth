use std::sync::Arc;

use crate::{
    bvh::aabb::Aabb,
    materials::Materials,
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
    utils::{
        constants::F_INF,
        functions::random_double,
        interval::{Interval, UNIVERSAL_INTERVAL},
    },
    vector::Vector,
};

#[derive(Clone)]
pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    optical_density: f64,
    material: Materials,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable>,
        optical_density: f64,
        material: Materials,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            optical_density,
            material,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let hit_record_1 = self.boundary.hit(ray, &UNIVERSAL_INTERVAL)?;

        let internal_interval = Interval::new(hit_record_1.get_t() + 0.001, F_INF);
        let hit_record_2 = self.boundary.hit(ray, &internal_interval)?;

        let (min, max) = interval.get_min_max();

        let mut hit_1_t = if hit_record_1.get_t() < min {
            min
        } else {
            hit_record_1.get_t()
        };

        let hit_2_t = if hit_record_2.get_t() > max {
            max
        } else {
            hit_record_2.get_t()
        };

        if hit_1_t >= hit_2_t {
            return None;
        }

        hit_1_t = f64::max(hit_1_t, 0.0);

        let ray_length = ray.get_direction().get_length();
        let internal_distance = (hit_2_t - hit_1_t) * ray_length;
        let hit_distance = (-1.0 / self.optical_density) * random_double().ln();

        if hit_distance > internal_distance {
            return None;
        }

        let t = hit_1_t + hit_distance / ray_length;
        let point = ray.at(t);

        // NOTE: Arbitrary Values (Normal, front face and texture coordinates don't matter)
        let normal = Vector::new(1.0, 0.0, 0.0);
        let front = true;
        let u = 0.0;
        let v = 0.0;

        Some(HitRecord::new(
            point,
            normal,
            t,
            ray,
            self.material.clone(),
            u,
            v,
        ))
    }

    fn get_aabb(&self) -> Aabb {
        self.boundary.get_aabb()
    }

    fn clone_box(&self) -> Arc<dyn Hittable> {
        Arc::new(self.clone())
    }
}
