use crate::{
    objects::hittable::HitRecord,
    ray::Ray,
    vector::{Color, get_random_unit_vector, reflect},
};

use super::scatterable::{ScatterRecord, Scatterable};

#[derive(Clone, Copy)]
pub struct MetalMaterial {
    albedo: Color,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: Color, fuzz: f64) -> MetalMaterial {
        MetalMaterial { albedo, fuzz }
    }
}

impl Scatterable for MetalMaterial {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let surface_normal_vec = hit_record.get_normal();
        let reflected_ray_direction = reflect(ray.get_direction(), surface_normal_vec);

        let fuzz_vec = get_random_unit_vector().scale(self.fuzz);
        let fuzzed_ray_direction = reflected_ray_direction.unit().addv(fuzz_vec);

        let scatter_ray = Ray::new(hit_record.get_point(), fuzzed_ray_direction);

        Some(ScatterRecord::new(scatter_ray, self.albedo))
    }
}
