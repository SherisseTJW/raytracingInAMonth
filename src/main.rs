mod bvh;
mod camera;
mod materials;
mod objects;
mod ray;
mod texture;
mod utils;
mod vector;

use std::sync::Arc;

use camera::Camera;
use materials::{
    Materials, dielectric::DielectricMaterial, lambertian::LambertianMaterial, metal::MetalMaterial,
};
use objects::{hittable::HittableList, sphere::Sphere};
use rand::random;
use vector::{Point, Vector};

use crate::{
    bvh::bvh::BvhNode,
    texture::{checker::CheckerTexture, image::ImageTexture, solid_color::SolidColorTexture, texture::Texture},
    utils::{
        constants::PI,
        functions::{random_double, random_double_in_range},
    },
    vector::{get_random_unit_vector, Color},
};

fn main() {
    // NOTE: Scene 1 - Triple Spheres of different materials + Air Bubble within Glass Sphere

    // let ground_material =
    //     Materials::Lambertian(LambertianMaterial::new(Vector::new(0.8, 0.8, 0.0)));
    // let centre_material =
    //     Materials::Lambertian(LambertianMaterial::new(Vector::new(0.1, 0.2, 0.5)));
    // let left_material = Materials::Dielectric(DielectricMaterial::new(1.50));
    // let right_material = Materials::Metal(MetalMaterial::new(Vector::new(0.8, 0.6, 0.2), 1.0));
    // let air_bubble_material = Materials::Dielectric(DielectricMaterial::new(1.0 / 1.50));
    //
    // let moving: Sphere = Sphere::new_moving_sphere(
    //     Point::new(0.0, 0.0, 0.0),
    //     Point::new(0.0, random_double_in_range(0.0, 0.5), 0.0),
    //     0.2,
    //     centre_material,
    // );
    //
    // let ground: Sphere = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground_material);
    // let left: Sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, left_material);
    // let right: Sphere = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, right_material);
    // let air_bubble: Sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.4, air_bubble_material);
    //
    // let mut hittable_list: HittableList = HittableList::new();
    // hittable_list.add_hittable(Arc::new(ground));
    // hittable_list.add_hittable(Arc::new(moving));
    // hittable_list.add_hittable(Arc::new(left));
    // hittable_list.add_hittable(Arc::new(right));
    // hittable_list.add_hittable(Arc::new(air_bubble));
    //
    // let size = hittable_list.get_num_hittables();
    // let mut hittables = hittable_list.get_hittables();
    //
    // let camera = Camera::default();
    // camera.render(BvhNode::new(&mut hittables, 0 as usize, size));

    // NOTE: Scene 2 - Testing Camera FOV

    // let r = f64::cos(PI / 4.0);
    //
    // let left_material = Materials::Lambertian(LambertianMaterial::new(Vector::new(0.0, 0.0, 1.0)));
    // let right_material = Materials::Lambertian(LambertianMaterial::new(Vector::new(1.0, 0.0, 0.0)));
    //
    // let left: Sphere = Sphere::new(Point::new(-r, 0.0, -1.0), r, left_material);
    // let right: Sphere = Sphere::new(Point::new(r, 0.0, -1.0), r, right_material);
    //
    // let mut world: HittableList = HittableList::new();
    // world.add_hittable(Arc::new(left));
    // world.add_hittable(Arc::new(right));
    //
    // let camera = Camera::default();
    // camera.render(world);

    // NOTE: Scene 3 - Final Render

    // let mut world: HittableList = HittableList::new();
    //
    // let ground_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
    //     CheckerTexture::new_from_solid_color(
    //         Color::new(0.2, 0.3, 0.1),
    //         Color::new(0.9, 0.9, 0.9),
    //         0.32,
    //     ),
    // )));
    // let ground = Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    // world.add_hittable(Arc::new(ground));
    //
    // for a in -8..8 {
    //     for b in -8..8 {
    //         let rdm_mat = random_double();
    //         let centre = Point::new(
    //             a as f64 + 0.9 * random_double(),
    //             0.2,
    //             b as f64 + 0.9 * random_double(),
    //         );
    //
    //         if centre.subv(Point::new(4.0, 0.2, 0.0)).get_length() > 0.9 {
    //             if rdm_mat < 0.6 {
    //                 let albedo = get_random_unit_vector().multiply(get_random_unit_vector());
    //                 let texture = Arc::new(SolidColorTexture::new_from_color(albedo));
    //
    //                 let mat = Materials::Lambertian(LambertianMaterial::new(texture));
    //                 let sphere = Sphere::new(centre, 0.2, mat);
    //                 world.add_hittable(Arc::new(sphere));
    //             } else if rdm_mat < 0.85 {
    //                 let albedo = get_random_unit_vector();
    //                 let fuzz = random_double_in_range(0.0, 0.5);
    //
    //                 let mat = Materials::Metal(MetalMaterial::new(albedo, fuzz));
    //                 let sphere = Sphere::new(centre, 0.2, mat);
    //                 world.add_hittable(Arc::new(sphere));
    //             } else {
    //                 let mat = Materials::Dielectric(DielectricMaterial::new(1.5));
    //                 let sphere = Sphere::new(centre, 0.2, mat);
    //                 world.add_hittable(Arc::new(sphere));
    //             }
    //         }
    //     }
    // }
    //
    // let mat1 = Materials::Dielectric(DielectricMaterial::new(1.5));
    // let sphere1 = Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0, mat1);
    // world.add_hittable(Arc::new(sphere1));
    //
    // let mat2 = Materials::Lambertian(LambertianMaterial::new(Arc::new(
    //     SolidColorTexture::new_from_rgb(0.4, 0.2, 0.1),
    // )));
    // let sphere2 = Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0, mat2);
    // world.add_hittable(Arc::new(sphere2));
    //
    // let mat3 = Materials::Metal(MetalMaterial::new(Color::new(0.7, 0.6, 0.5), 0.0));
    // let sphere3 = Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0, mat3);
    // world.add_hittable(Arc::new(sphere3));
    //
    // let size = world.get_num_hittables();
    // let mut hittables = world.get_hittables();
    //
    // let mut camera: Camera = Camera::default();
    // camera = camera.override_sampling_specs(250, 50);
    // camera = camera.override_camera_pos(
    //     Point::new(13.0, 2.0, 3.0),
    //     Point::new(0.0, 0.0, 0.0),
    //     Vector::new(0.0, 1.0, 0.0),
    //     20.0,
    //     0.6,
    //     10.0,
    // );
    // camera = camera.override_image_specs(16.0 / 9.0, 800);
    // camera.render(BvhNode::new(&mut hittables, 0 as usize, size));

    // NOTE: Scene 5 - Earth
    let centre_material =
        Materials::Lambertian(LambertianMaterial::new(Arc::new(ImageTexture::new("./texture_assets/earthmap.jpg"))));

    let centre: Sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 0.3, centre_material);

    let mut hittable_list: HittableList = HittableList::new();
    hittable_list.add_hittable(Arc::new(centre));

    let size = hittable_list.get_num_hittables();
    let mut hittables = hittable_list.get_hittables();

    let camera = Camera::default();
    camera.render(BvhNode::new(&mut hittables, 0 as usize, size));
}

