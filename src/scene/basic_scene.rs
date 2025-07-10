// NOTE: Scene 1 - Triple Spheres of different materials + Air Bubble within Glass Sphere

use std::sync::Arc;

use crate::{
    camera::Camera,
    materials::{
        Materials, dielectric::DielectricMaterial, lambertian::LambertianMaterial,
        metal::MetalMaterial,
    },
    objects::{hittable::HittableList, sphere::Sphere},
    scene::scene::Scene,
    texture::solid_color::SolidColorTexture,
    utils::functions::random_double_in_range,
    vector::{Point, Vector},
};

pub fn basic_scene() -> Scene {
    let ground_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.8, 0.8, 0.0),
    )));
    let centre_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.1, 0.2, 0.5),
    )));
    let left_material = Materials::Dielectric(DielectricMaterial::new(1.50));
    let right_material = Materials::Metal(MetalMaterial::new(Vector::new(0.8, 0.6, 0.2), 1.0));
    let air_bubble_material = Materials::Dielectric(DielectricMaterial::new(1.0 / 1.50));

    let moving: Sphere = Sphere::new_moving_sphere(
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.0, random_double_in_range(0.0, 0.5), 0.0),
        0.2,
        centre_material,
    );

    let ground: Sphere = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground_material);
    let left: Sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, left_material);
    let right: Sphere = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, right_material);
    let air_bubble: Sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.4, air_bubble_material);

    let mut hittable_list: HittableList = HittableList::new();
    hittable_list.add_hittable(Arc::new(ground));
    hittable_list.add_hittable(Arc::new(moving));
    hittable_list.add_hittable(Arc::new(left));
    hittable_list.add_hittable(Arc::new(right));
    hittable_list.add_hittable(Arc::new(air_bubble));

    let camera = Camera::default();
    Scene::new(hittable_list, camera)
}
