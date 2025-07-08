use crate::{
    objects::hittable::HitRecord,
    ray::Ray,
    texture::{solid_color::SolidColor, texture::Texture},
    vector::{Color, Vector, get_random_unit_vector_on_hemisphere},
};

use super::scatterable::{ScatterRecord, Scatterable};

#[derive(Clone)]
pub struct LambertianMaterial {
    texture: Box<dyn Texture>,
}

impl LambertianMaterial {
    pub fn new(texture: Box<dyn Texture>) -> LambertianMaterial {
        LambertianMaterial { texture }
    }
}

impl Scatterable for LambertianMaterial {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        let surface_normal_vec = hit_record.get_normal();
        let scatter_ray_direction: Vector =
            get_random_unit_vector_on_hemisphere(surface_normal_vec).addv(surface_normal_vec);

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
