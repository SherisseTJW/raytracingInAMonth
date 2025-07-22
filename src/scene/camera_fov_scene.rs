// NOTE: Scene 2 - Testing Camera FOV

use std::sync::Arc;

use crate::{
    camera::Camera,
    materials::{Materials, lambertian::LambertianMaterial},
    objects::{hittable::HittableList, sphere::Sphere},
    scene::scene::Scene,
    texture::solid_color::SolidColorTexture,
    utils::constants::PI,
    vector::{Point, Vector},
};

pub fn camera_fov_scene() -> Scene {
    let r = f64::cos(PI / 4.0);

    let left_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.0, 0.0, 1.0),
    )));
    let right_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(1.0, 0.0, 0.0),
    )));

    let left: Sphere = Sphere::new(Point::new(-r, 0.0, -1.0), r, left_material);
    let right: Sphere = Sphere::new(Point::new(r, 0.0, -1.0), r, right_material);

    let mut hittable_list: HittableList = HittableList::new();
    hittable_list.add_hittable(Arc::new(left));
    hittable_list.add_hittable(Arc::new(right));

    let camera = Camera::default();
    Scene::new(hittable_list, camera)
}
