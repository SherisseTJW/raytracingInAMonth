use crate::{
    objects::hittable::HitRecord,
    ray::Ray,
    vector::{Color, Vector, refract},
};

use super::scatterable::{ScatterRecord, Scatterable};

#[derive(Clone, Copy)]
pub struct DielectricMaterial {
    refraction_index: f64,
}

impl DielectricMaterial {
    pub fn new(refraction_index: f64) -> DielectricMaterial {
        DielectricMaterial { refraction_index }
    }
}

impl Scatterable for DielectricMaterial {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let attenuation: Color = Color::new(1.0, 1.0, 1.0);

        let ri: f64 = if hit_record.get_front() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.get_direction().unit();
        let refracted_ray_direction: Vector = refract(unit_direction, hit_record.get_normal(), ri);

        let scattered_ray = Ray::new(hit_record.get_point(), refracted_ray_direction);

        Some(ScatterRecord::new(scattered_ray, attenuation))
    }
}
