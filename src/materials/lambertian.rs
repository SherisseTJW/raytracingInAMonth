use crate::{
    objects::hittable::HitRecord,
    ray::Ray,
    vector::{Color, Vector},
};

use super::scatterable::{ScatterRecord, Scatterable};

#[derive(Clone, Copy)]
pub struct LambertianMaterial {
    albedo: Color,
}

impl LambertianMaterial {
    pub fn new(albedo: Color) -> LambertianMaterial {
        LambertianMaterial { albedo }
    }
}

impl Scatterable for LambertianMaterial {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let surface_normal_vec = hit_record.get_normal();
        let scatter_ray_direction: Vector =
            Vector::get_random_unit_vector_on_hemisphere(surface_normal_vec)
                .addv(surface_normal_vec);

        let scatter_ray = if scatter_ray_direction.near_zero() {
            Ray::new(hit_record.get_point(), surface_normal_vec)
        } else {
            Ray::new(hit_record.get_point(), scatter_ray_direction)
        };

        Some(ScatterRecord::new(scatter_ray, self.albedo))
    }
}
