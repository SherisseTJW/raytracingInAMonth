use crate::{
    objects::hittable::HitRecord,
    ray::Ray,
    texture::{solid_color::SolidColorTexture, texture::Texture},
    vector::{Color, Vector, get_random_unit_vector, get_random_unit_vector_on_hemisphere},
};
use std::{fmt::Display, sync::Arc};

use super::scatterable::{ScatterRecord, Scatterable};

#[derive(Clone)]
pub struct LambertianMaterial {
    texture: Arc<dyn Texture>,
}

impl LambertianMaterial {
    pub fn new(texture: Arc<dyn Texture>) -> LambertianMaterial {
        LambertianMaterial { texture }
    }
}

impl Scatterable for LambertianMaterial {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let surface_normal_vec = hit_record.get_normal();
        let scatter_ray_direction: Vector = get_random_unit_vector().addv(surface_normal_vec);

        let scatter_ray = if scatter_ray_direction.near_zero() {
            Ray::new(
                hit_record.get_point(),
                surface_normal_vec,
                Some(ray.get_time()),
            )
        } else {
            Ray::new(
                hit_record.get_point(),
                scatter_ray_direction,
                Some(ray.get_time()),
            )
        };

        let (u, v) = hit_record.get_texture_coordinates();
        Some(ScatterRecord::new(
            scatter_ray,
            self.texture.get_value(u, v, hit_record.get_point()),
        ))
    }
}

impl Display for LambertianMaterial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LambertianMaterial with Texture: {}", self.texture)
    }
}
