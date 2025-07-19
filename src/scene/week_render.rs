// NOTE: Final Render Scene (From Ray Tracing, the Next Week)

use std::sync::Arc;

use crate::{
    camera::Camera,
    materials::{
        Materials, dielectric::DielectricMaterial, diffuse_light::DiffuseLightMaterial,
        lambertian::LambertianMaterial, metal::MetalMaterial,
    },
    objects::{
        cube::Cube,
        hittable::{Hittable, HittableList},
        quad::Quad,
        sphere::Sphere,
    },
    scene::scene::Scene,
    texture::{
        image::ImageTexture,
        perlin_noise::{PerlinNoiseEffect, PerlinNoiseTexture},
        solid_color::SolidColorTexture,
    },
    transformation::{rotation::Rotation, translation::Translation},
    utils::functions::random_double_in_range,
    vector::{Color, Point, Vector},
};

pub fn week_scene() -> Scene {
    let mut hittable_list: HittableList = HittableList::new();

    let ground_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.48, 0.83, 0.53),
    )));

    // NOTE: Somehow the parameters given in the book don't work?
    // They'll cause the blocks to spawn outside the scene
    let boxes_per_side = 20;
    let w = 100.0;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let x0 = -900.0 + (i as f64 * w);
            let y0 = -100.0;
            let z0 = 0.0 + (j as f64 * w);

            let x1 = x0 + w;
            let y1 = random_double_in_range(-50.0, 51.0);
            let z1 = z0 + w;

            let ground_box = Cube::new(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                ground_material.clone(),
            );
            hittable_list.add_hittable_list(ground_box.to_hittable_list());
        }
    }

    let light_material = Materials::Diffuse(DiffuseLightMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(7.0, 7.0, 7.0),
    )));
    let light_source: Quad = Quad::new(
        Point::new(123.0, 554.0, 147.0),
        Vector::new(300.0, 0.0, 0.0),
        Vector::new(0.0, 0.0, 265.0),
        light_material,
    );
    hittable_list.add_hittable(Arc::new(light_source));

    let centre_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.7, 0.3, 0.1),
    )));
    let centre_sphere = Sphere::new_moving_sphere(
        Point::new(400.0, 400.0, 200.0),
        Point::new(430.0, 400.0, 200.0),
        50.0,
        centre_material,
    );
    hittable_list.add_hittable(Arc::new(centre_sphere));

    let glass_material = Materials::Dielectric(DielectricMaterial::new(1.5));
    let dieletric_sphere = Sphere::new(Point::new(260.0, 150.0, 45.0), 50.0, glass_material);
    hittable_list.add_hittable(Arc::new(dieletric_sphere));

    let metal_material = Materials::Metal(MetalMaterial::new(Color::new(0.8, 0.8, 0.9), 1.0));
    let metal_sphere = Sphere::new(Point::new(0.0, 150.0, 145.0), 50.0, metal_material);
    hittable_list.add_hittable(Arc::new(metal_sphere));

    let earth_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        ImageTexture::new("./texture_assets/earthmap.jpg"),
    )));
    let earth_sphere = Sphere::new(Point::new(400.0, 200.0, 400.0), 100.0, earth_material);
    hittable_list.add_hittable(Arc::new(earth_sphere));

    let perlin_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        PerlinNoiseTexture::new(0.2, 1, PerlinNoiseEffect::Marble),
    )));
    let perlin_sphere = Sphere::new(Point::new(220.0, 280.0, 300.0), 80.0, perlin_material);
    hittable_list.add_hittable(Arc::new(perlin_sphere));

    let floating_sphere_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.73, 0.73, 0.73),
    )));

    // NOTE: Same thing here...
    // Somehow the parameters given in the book don't work?
    let mut floating_spheres_list: HittableList = HittableList::new();
    for i in 0..1000 {
        let location = Vector::new(
            random_double_in_range(-300.0, -165.0),
            random_double_in_range(270.0, 435.0),
            random_double_in_range(395.0, 560.0),
        );
        let cur_sphere = Sphere::new(location, 10.0, floating_sphere_material.clone());

        floating_spheres_list.add_hittable(Arc::new(cur_sphere));
    }
    let rotated_spheres = Rotation::new(Arc::new(floating_spheres_list), 0.0, 15.0, 0.0);
    hittable_list.add_hittable(Arc::new(rotated_spheres));

    // Camera Settings
    let mut camera = Camera::default();
    camera = camera.override_image_specs(1.0, 800);
    camera = camera.override_camera_pos(
        Point::new(478.0, 278.0, -600.0),
        Point::new(278.0, 278.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        40.0,
        0.0,
        2.0,
    );
    // FIX: Change sampling size and max_depth back to 10000 and 40
    // (set to 250 and 40 for debugging to speed up)
    camera = camera.override_sampling_specs(10000, 40);
    camera.set_background(Color::new(0.0, 0.0, 0.0));

    Scene::new(hittable_list, camera)
}
