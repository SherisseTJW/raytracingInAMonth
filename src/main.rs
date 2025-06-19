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

fn main() {
    let ground_material =
        Materials::Lambertian(LambertianMaterial::new(Vector::new(0.8, 0.8, 0.0)));
    let centre_material =
        Materials::Lambertian(LambertianMaterial::new(Vector::new(0.1, 0.2, 0.5)));
    let left_material = Materials::Dielectric(DielectricMaterial::new(1.5));
    let right_material = Materials::Metal(MetalMaterial::new(Vector::new(0.8, 0.6, 0.2), 1.0));

    let ground: Sphere = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, ground_material);
    let centre: Sphere = Sphere::new(Point::new(0.0, 0.0, -1.2), 0.5, centre_material);
    let left: Sphere = Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, left_material);
    let right: Sphere = Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, right_material);

    let mut world: HittableList = HittableList::new();
    world.add_hittable(Box::new(ground));
    world.add_hittable(Box::new(centre));
    world.add_hittable(Box::new(left));
    world.add_hittable(Box::new(right));

    let camera: Camera = Camera::default();
    camera.render(world);
}
