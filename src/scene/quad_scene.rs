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
    let top_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(1.0, 0.5, 0.0),
    )));
    let back_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.2, 1.0, 0.2),
    )));
    let bottom_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(1.0, 0.5, 0.0),
    )));
    let left_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(1.0, 0.2, 0.2),
    )));
    let right_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.2, 0.2, 1.0),
    )));

    let top: Quad = Quad::new(
        Point::new(-2.0, -3.0, 1.0),
        Vector::new(4.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -4.0),
        top_material,
    );
    let back: Quad = Quad::new(
        Point::new(-2.0, -2.0, 0.0),
        Vector::new(4.0, 0.0, 0.0),
        Vector::new(0.0, 4.0, 0.0),
        back_material,
    );
    let bottom: Quad = Quad::new(
        Point::new(-2.0, -3.0, 5.0),
        Vector::new(4.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -4.0),
        bottom_material,
    );
    let left: Quad = Quad::new(
        Point::new(-3.0, -2.0, 5.0),
        Vector::new(0.0, 0.0, -4.0),
        Vector::new(0.0, 4.0, 0.0),
        left_material,
    );
    let right: Quad = Quad::new(
        Point::new(3.0, -2.0, 1.0),
        Vector::new(0.0, 0.0, 4.0),
        Vector::new(0.0, 4.0, 0.0),
        right_material,
    );

    // TEST: Checking if we can see other stuff
    let ground_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.8, 0.8, 0.0),
    )));
    let ground: Sphere = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground_material);

    let mut hittable_list: HittableList = HittableList::new();
    hittable_list.add_hittable(Arc::new(top));
    // hittable_list.add_hittable(Arc::new(back));
    // hittable_list.add_hittable(Arc::new(bottom));
    // hittable_list.add_hittable(Arc::new(left));
    // hittable_list.add_hittable(Arc::new(right));
    // hittable_list.add_hittable(Arc::new(ground));

    let camera = Camera::default();
    camera.override_image_specs(1.0, 400);
    camera.override_sampling_specs(100, 50);
    camera.override_camera_pos(
        Point::new(0.0, 0.0, 9.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        80.0,
        0.0,
        2.0,
    );

    Scene::new(hittable_list, camera)
}
