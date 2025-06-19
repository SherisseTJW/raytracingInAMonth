pub mod dielectric;
pub mod lambertian;
pub mod metal;
pub mod scatterable;

use crate::{objects::hittable::HitRecord, ray::Ray};

use dielectric::DielectricMaterial;
use lambertian::LambertianMaterial;
use metal::MetalMaterial;
use scatterable::{ScatterRecord, Scatterable};

#[derive(Clone, Copy)]
pub enum Materials {
    Lambertian(LambertianMaterial),
    Metal(MetalMaterial),
    Dielectric(DielectricMaterial),
}

impl Scatterable for Materials {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        match self {
            Materials::Lambertian(mat) => mat.scatter(ray, hit_record),
            Materials::Metal(mat) => mat.scatter(ray, hit_record),
            Materials::Dielectric(mat) => mat.scatter(ray, hit_record),
        }
    }
}
