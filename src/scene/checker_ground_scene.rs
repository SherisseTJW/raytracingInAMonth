// NOTE: Final Render Scene 1 (From Ray Tracing in a Weekend)
// but with a Checker Texture on the ground

use std::sync::Arc;

use crate::{
    camera::Camera,
    materials::{
        Materials, dielectric::DielectricMaterial, lambertian::LambertianMaterial,
        metal::MetalMaterial,
    },
    objects::{hittable::HittableList, sphere::Sphere},
    scene::scene::Scene,
    texture::{checker::CheckerTexture, solid_color::SolidColorTexture},
    utils::functions::{random_double, random_double_in_range},
    vector::{Color, Point, Vector, get_random_unit_vector},
};

pub fn checker_scene() -> Scene {
    let mut hittable_list: HittableList = HittableList::new();

    let ground_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        CheckerTexture::new_from_solid_color(
            Color::new(0.2, 0.3, 0.1),
            Color::new(0.9, 0.9, 0.9),
            0.32,
        ),
    )));

    let ground = Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    hittable_list.add_hittable(Arc::new(ground));

    for a in -8..8 {
        for b in -8..8 {
            let rdm_mat = random_double();
            let centre = Point::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if centre.subv(Point::new(4.0, 0.2, 0.0)).get_length() > 0.9 {
                if rdm_mat < 0.6 {
                    let albedo = get_random_unit_vector().multiply(get_random_unit_vector());
                    let texture = Arc::new(SolidColorTexture::new_from_color(albedo));

                    let mat = Materials::Lambertian(LambertianMaterial::new(texture));
                    let sphere = Sphere::new(centre, 0.2, mat);
                    hittable_list.add_hittable(Arc::new(sphere));
                } else if rdm_mat < 0.85 {
                    let albedo = get_random_unit_vector();
                    let fuzz = random_double_in_range(0.0, 0.5);

                    let mat = Materials::Metal(MetalMaterial::new(albedo, fuzz));
                    let sphere = Sphere::new(centre, 0.2, mat);
                    hittable_list.add_hittable(Arc::new(sphere));
                } else {
                    let mat = Materials::Dielectric(DielectricMaterial::new(1.5));
                    let sphere = Sphere::new(centre, 0.2, mat);
                    hittable_list.add_hittable(Arc::new(sphere));
                }
            }
        }
    }

    let mat1 = Materials::Dielectric(DielectricMaterial::new(1.5));
    let sphere1 = Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, mat1);
    hittable_list.add_hittable(Arc::new(sphere1));

    let mat2 = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        SolidColorTexture::new_from_rgb(0.4, 0.2, 0.1),
    )));
    let sphere2 = Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, mat2);
    hittable_list.add_hittable(Arc::new(sphere2));

    let mat3 = Materials::Metal(MetalMaterial::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let sphere3 = Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, mat3);
    hittable_list.add_hittable(Arc::new(sphere3));

    let mut camera: Camera = Camera::default();
    camera = camera.override_sampling_specs(250, 50);
    camera = camera.override_camera_pos(
        Point::new(13.0, 2.0, 3.0),
        Point::new(0.0, 0.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
        20.0,
        0.6,
        10.0,
    );
    camera = camera.override_image_specs(16.0 / 9.0, 800);

    Scene::new(hittable_list, camera)
}
