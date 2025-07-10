// NOTE: Perlin Noise ground + Main centre sphere

use std::sync::Arc;

use crate::{
    camera::Camera,
    materials::{
        Materials, dielectric::DielectricMaterial, lambertian::LambertianMaterial,
        metal::MetalMaterial,
    },
    objects::{hittable::HittableList, sphere::Sphere},
    scene::scene::Scene,
    texture::{perlin_noise::PerlinNoiseTexture, solid_color::SolidColorTexture},
    utils::functions::random_double_in_range,
    vector::{Point, Vector},
};

pub fn perlin_scene() -> Scene {
    let ground_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        PerlinNoiseTexture::new(256),
    )));
    let centre_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        PerlinNoiseTexture::new(256),
    )));

    let ground: Sphere = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground_material);
    let centre: Sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.3, centre_material);

    let mut hittable_list: HittableList = HittableList::new();
    hittable_list.add_hittable(Arc::new(ground));
    hittable_list.add_hittable(Arc::new(centre));

    let camera = Camera::default();
    Scene::new(hittable_list, camera)
}
