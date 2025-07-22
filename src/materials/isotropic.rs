use std::sync::Arc;

use crate::{
    materials::scatterable::{ScatterRecord, Scatterable},
    objects::hittable::HitRecord,
    ray::Ray,
    texture::texture::Texture,
    vector::get_random_unit_vector,
};

#[derive(Clone)]
pub struct IsotropicMaterial {
    texture: Arc<dyn Texture>,
}

impl IsotropicMaterial {
    pub fn new(texture: Arc<dyn Texture>) -> IsotropicMaterial {
        IsotropicMaterial { texture }
    }
}

impl Scatterable for IsotropicMaterial {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let (u, v) = hit_record.get_texture_coordinates();
        let scatter_ray = Ray::new(
            hit_record.get_point(),
            get_random_unit_vector(),
            Some(ray.get_time()),
        );
        let attenuation = self.texture.get_value(u, v, hit_record.get_point());

        Some(ScatterRecord::new(scatter_ray, attenuation))
    }
}
