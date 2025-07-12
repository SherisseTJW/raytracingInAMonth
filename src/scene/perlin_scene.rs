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
    texture::{
        perlin_noise::{PerlinNoiseEffect, PerlinNoiseTexture},
        solid_color::SolidColorTexture,
    },
    utils::functions::random_double_in_range,
    vector::{Point, Vector},
};

pub fn perlin_scene() -> Scene {
    let ground_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        PerlinNoiseTexture::new(3.0, 4, PerlinNoiseEffect::WhiteNoise),
    )));
    let centre_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        PerlinNoiseTexture::new(20.0, 15, PerlinNoiseEffect::Marble),
    )));

    let ground: Sphere = Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    let centre: Sphere = Sphere::new(Point::new(0.0, 2.0, 0.0), 2.0, centre_material);

    let mut hittable_list: HittableList = HittableList::new();
    hittable_list.add_hittable(Arc::new(ground));
    hittable_list.add_hittable(Arc::new(centre));

    let mut camera = Camera::default();
    camera = camera.override_camera_pos(
        Point::new(13.0, 12.0, 8.0),
        Point::new(0.0, 2.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        0.0,
        2.0,
    );
    Scene::new(hittable_list, camera)
}
