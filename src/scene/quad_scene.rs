// NOTE: Basic Quad Scene (Think a box but the sides are not connected)

use std::sync::Arc;

use crate::{
    camera::Camera,
    materials::{Materials, lambertian::LambertianMaterial},
    objects::{hittable::HittableList, quad::Quad, sphere::Sphere},
    scene::scene::Scene,
    texture::solid_color::SolidColorTexture,
    vector::{Point, Vector},
};

pub fn quad_scene() -> Scene {
    let left_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(1.0, 0.2, 0.2),
    )));
    let back_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.2, 1.0, 0.2),
    )));
    let right_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(-0.2, 0.2, 1.0),
    )));
    let top_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(1.0, 0.5, 0.0),
    )));
    let bottom_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.2, 0.8, 0.8),
    )));

    let left: Quad = Quad::new(
        Point::new(-3.0, -2.0, 5.0),
        Vector::new(0.0, 0.0, -4.0),
        Vector::new(0.0, 4.0, 0.0),
        left_material,
    );
    let back: Quad = Quad::new(
        Point::new(-2.0, -2.0, 0.0),
        Vector::new(4.0, 0.0, 0.0),
        Vector::new(0.0, 4.0, 0.0),
        back_material,
    );
    let right: Quad = Quad::new(
        Point::new(3.0, -2.0, 1.0),
        Vector::new(0.0, 0.0, 4.0),
        Vector::new(0.0, 4.0, 0.0),
        right_material,
    );
    let top: Quad = Quad::new(
        Point::new(-2.0, 3.0, 1.0),
        Vector::new(4.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 4.0),
        top_material,
    );
    let bottom: Quad = Quad::new(
        Point::new(-2.0, -3.0, 5.0),
        Vector::new(4.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -4.0),
        bottom_material,
    );

    let mut hittable_list: HittableList = HittableList::new();
    hittable_list.add_hittable(Arc::new(top));
    hittable_list.add_hittable(Arc::new(back));
    hittable_list.add_hittable(Arc::new(bottom));
    hittable_list.add_hittable(Arc::new(left));
    hittable_list.add_hittable(Arc::new(right));

    let mut camera = Camera::default();
    camera = camera.override_image_specs(1.0, 400);
    camera = camera.override_sampling_specs(100, 50);
    camera = camera.override_camera_pos(
        Point::new(0.0, 0.0, 9.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        80.0,
        0.0,
        2.0,
    );

    Scene::new(hittable_list, camera)
}
