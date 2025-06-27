mod camera;
mod materials;
mod objects;
mod ray;
mod utils;
mod vector;

use camera::Camera;
use materials::{
    Materials, dielectric::DielectricMaterial, lambertian::LambertianMaterial, metal::MetalMaterial,
};
use objects::{hittable::HittableList, sphere::Sphere};
use vector::{Point, Vector};

use crate::utils::constants::PI;

fn main() {
    // NOTE: Scene 1 - Triple Spheres of different materials + Air Bubble within Glass Sphere

    let ground_material =
        Materials::Lambertian(LambertianMaterial::new(Vector::new(0.8, 0.8, 0.0)));
    let centre_material =
        Materials::Lambertian(LambertianMaterial::new(Vector::new(0.1, 0.2, 0.5)));
    let left_material = Materials::Dielectric(DielectricMaterial::new(1.50));
    let right_material = Materials::Metal(MetalMaterial::new(Vector::new(0.8, 0.6, 0.2), 1.0));
    let air_bubble_material = Materials::Dielectric(DielectricMaterial::new(1.0 / 1.50));

    let ground: Sphere = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground_material);
    let centre: Sphere = Sphere::new(Point::new(0.0, 0.0, -1.2), 0.5, centre_material);
    let left: Sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, left_material);
    let right: Sphere = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, right_material);
    let air_bubble: Sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.4, air_bubble_material);

    let mut world: HittableList = HittableList::new();
    world.add_hittable(Box::new(ground));
    world.add_hittable(Box::new(centre));
    world.add_hittable(Box::new(left));
    world.add_hittable(Box::new(right));
    world.add_hittable(Box::new(air_bubble));

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
    // world.add_hittable(Box::new(left));
    // world.add_hittable(Box::new(right));

    // TODO: Expose some settings for camera
    let camera: Camera = Camera::default();
    camera.render(world);
}
