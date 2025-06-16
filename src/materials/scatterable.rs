use crate::{objects::hittable::HitRecord, ray::Ray, vector::Color};

pub struct ScatterRecord {
    ray: Ray,
    attenuation: Color,
}

impl ScatterRecord {
    pub fn new(ray: Ray, attenuation: Color) -> ScatterRecord {
        ScatterRecord { ray, attenuation }
    }

    pub fn get_ray(&self) -> Ray {
        self.ray
    }

    pub fn get_attenuation(&self) -> Color {
        self.attenuation
    }
}

pub trait Scatterable {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord>;
}
