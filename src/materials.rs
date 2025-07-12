pub mod dielectric;
pub mod diffuse_light;
pub mod emission;
pub mod lambertian;
pub mod metal;
pub mod scatterable;

use crate::{
    materials::{diffuse_light::DiffuseLightMaterial, emission::Emission},
    objects::hittable::HitRecord,
    ray::Ray,
    vector::{Color, Point},
};

use dielectric::DielectricMaterial;
use lambertian::LambertianMaterial;
use metal::MetalMaterial;
use scatterable::{ScatterRecord, Scatterable};

#[derive(Clone)]
pub enum Materials {
    Lambertian(LambertianMaterial),
    Metal(MetalMaterial),
    Dielectric(DielectricMaterial),
    Diffuse(DiffuseLightMaterial),
}

impl Scatterable for Materials {
    fn scatter(&self, ray: Ray, hit_record: HitRecord) -> Option<ScatterRecord> {
        match self {
            Materials::Lambertian(mat) => mat.scatter(ray, hit_record),
            Materials::Metal(mat) => mat.scatter(ray, hit_record),
            Materials::Dielectric(mat) => mat.scatter(ray, hit_record),
            _ => None,
        }
    }
}

impl Emission for Materials {
    fn emit(&self, u: f64, v: f64, point: Point) -> Color {
        match self {
            Materials::Diffuse(mat) => mat.emit(u, v, point),
            _ => Color::new(0.0, 0.0, 0.0), // Default: Black (does not emit light)
        }
    }
}
