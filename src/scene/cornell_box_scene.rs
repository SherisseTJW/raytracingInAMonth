// NOTE: Cornell Box Scene

use std::sync::Arc;

use crate::{
    camera::{self, Camera},
    materials::{Materials, diffuse_light::DiffuseLightMaterial, lambertian::LambertianMaterial},
    objects::{cube::Cube, hittable::HittableList, quad::Quad},
    scene::scene::Scene,
    texture::solid_color::SolidColorTexture,
    transformation::{rotation::Rotation, translation::Translation},
    vector::{Color, Point, Vector},
};

pub fn cornell_box_scene() -> Scene {
    let red_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.65, 0.05, 0.05),
    )));
    let white_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.73, 0.73, 0.73),
    )));
    let green_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.12, 0.45, 0.15),
    )));
    let light_material = Materials::Diffuse(DiffuseLightMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(15.0, 15.0, 15.0),
    )));

    let left: Quad = Quad::new(
        Point::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 555.0, 0.0),
        Vector::new(0.0, 0.0, 555.0),
        green_material,
    );
    let back: Quad = Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 555.0),
        white_material.clone(),
    );
    let right: Quad = Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 555.0, 0.0),
        Vector::new(0.0, 0.0, 555.0),
        red_material,
    );
    let top: Quad = Quad::new(
        Point::new(555.0, 555.0, 555.0),
        Vector::new(-555.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -555.0),
        white_material.clone(),
    );
    let bottom: Quad = Quad::new(
        Point::new(0.0, 0.0, 555.0),
        Vector::new(555.0, 0.0, 0.0),
        Vector::new(0.0, 555.0, 0.0),
        white_material.clone(),
    );
    let light_source: Quad = Quad::new(
        Point::new(343.0, 554.0, 332.0),
        Vector::new(-130.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, -105.0),
        light_material,
    );

    // NOTE: Original, unrotated and untranslated boxes
    // let box_1 = Cube::new(
    //     Point::new(130.0, 0.0, 65.0),
    //     Point::new(295.0, 165.0, 230.0),
    //     white_material.clone(),
    // );
    // let box_2 = Cube::new(
    //     Point::new(265.0, 0.0, 295.0),
    //     Point::new(430.0, 330.0, 460.0),
    //     white_material.clone(),
    // );

    // NOTE: Final, rotated and translated boxes
    let mut box_1: Cube = Cube::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0),
        white_material.clone(),
    );
    box_1.rotate(0.0, 15.0, 0.0);
    box_1.translate(Vector::new(265.0, 0.0, 295.0));
    let mut box_2: Cube = Cube::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0),
        white_material.clone(),
    );
    box_2.rotate(0.0, -18.0, 0.0);
    box_2.translate(Vector::new(130.0, 0.0, 65.0));

    let mut hittable_list: HittableList = HittableList::new();
    hittable_list.add_hittable(Arc::new(top));
    hittable_list.add_hittable(Arc::new(back));
    hittable_list.add_hittable(Arc::new(bottom));
    hittable_list.add_hittable(Arc::new(left));
    hittable_list.add_hittable(Arc::new(right));
    hittable_list.add_hittable(Arc::new(light_source));
    hittable_list.add_hittable_list(box_1.to_hittable_list());
    hittable_list.add_hittable_list(box_2.to_hittable_list());

    let mut camera = Camera::default();
    camera = camera.override_image_specs(1.0, 600);
    camera = camera.override_camera_pos(
        Point::new(278.0, 278.0, -800.0),
        Point::new(278.0, 278.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        40.0,
        0.0,
        2.0,
    );
    camera = camera.override_sampling_specs(200, 50);
    camera.set_background(Color::new(0.0, 0.0, 0.0));

    Scene::new(hittable_list, camera)
}

