// NOTE: Simple Light Scene requiring a diffuse light

use std::sync::Arc;

use crate::{
    camera::Camera,
    materials::{
        Materials, dielectric::DielectricMaterial, diffuse_light::DiffuseLightMaterial,
        lambertian::LambertianMaterial, metal::MetalMaterial,
    },
    objects::{hittable::HittableList, quad::Quad, sphere::Sphere},
    scene::scene::Scene,
    texture::{
        perlin_noise::{PerlinNoiseEffect, PerlinNoiseTexture},
        solid_color::SolidColorTexture,
    },
    transformation::rotation::Rotation,
    utils::functions::random_double_in_range,
    vector::{Color, Point, Vector},
};

pub fn simple_light_scene() -> Scene {
    let ground_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        PerlinNoiseTexture::new(3.0, 4, PerlinNoiseEffect::WhiteNoise),
    )));
    let centre_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        PerlinNoiseTexture::new(20.0, 15, PerlinNoiseEffect::Marble),
    )));
    let light_material = Materials::Diffuse(DiffuseLightMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(4.0, 4.0, 4.0),
    )));

    let ground: Sphere = Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    let centre: Sphere = Sphere::new(Point::new(0.0, 2.0, 0.0), 2.0, centre_material);
    let side_light_source: Quad = Quad::new(
        Point::new(3.0, 1.0, -2.0),
        Vector::new(2.0, 0.0, 0.0),
        Vector::new(0.0, 2.0, 0.0),
        light_material.clone(),
    );
    let top_light_source: Sphere =
        Sphere::new(Point::new(0.0, 7.0, 0.0), 2.0, light_material.clone());

    let mut hittable_list: HittableList = HittableList::new();
    hittable_list.add_hittable(Arc::new(ground));
    hittable_list.add_hittable(Arc::new(centre));
    // hittable_list.add_hittable(Arc::new(side_light_source));
    hittable_list.add_hittable(Arc::new(top_light_source));

    let mut camera = Camera::default();
    let look_from = Point::new(26.0, 3.0, 6.0);
    let look_at = Point::new(0.0, 2.0, 0.0);

    camera = camera.override_camera_pos(
        look_from,
        look_at,
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        0.0,
        look_from.subv(look_at).get_length(),
    );
    camera.set_background(Color::new(0.0, 0.0, 0.0));
    Scene::new(hittable_list, camera)
}
