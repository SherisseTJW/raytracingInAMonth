pub mod lambertian;
pub mod metal;
pub mod scatterable;

use crate::{objects::hittable::HitRecord, ray::Ray};

use lambertian::LambertianMaterial;
use scatterable::{ScatterRecord, Scatterable};

#[derive(Clone, Copy)]
pub enum Materials {
    Lambertian(LambertianMaterial),
    Metal,
}

impl Scatterable for Materials {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        match self {
            Materials::Lambertian(mat) => mat.scatter(ray, hit_record),
            Materials::Metal => None,
        }
    }
}
