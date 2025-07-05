use crate::{
    objects::hittable::HitRecord,
    ray::Ray,
    utils::functions::random_double,
    vector::{Color, Vector, dot_product, reflect, refract},
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

    // NOTE: Schlick Approximation
    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0_squared = r0 * r0;

        r0_squared + (1.0 - r0_squared) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Scatterable for DielectricMaterial {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let normal = hit_record.get_normal();

        let ri: f64 = if hit_record.get_front() {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = ray.get_direction().unit();

        let cos_theta = f64::min(dot_product(unit_direction.negate(), normal), 1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let reflect_angle: f64 = DielectricMaterial::reflectance(cos_theta, ri);

        let ray_direction: Vector = if cannot_refract || reflect_angle > random_double() {
            reflect(unit_direction, normal)
        } else {
            refract(unit_direction, normal, ri)
        };

        let scattered_ray = Ray::new(hit_record.get_point(), ray_direction, Some(ray.get_time()));

        Some(ScatterRecord::new(scattered_ray, Color::new(1.0, 1.0, 1.0)))
    }
}
