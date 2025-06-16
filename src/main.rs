mod camera;
mod materials;
mod objects;
mod ray;
mod utils;
mod vector;

use camera::Camera;
use materials::{Materials, lambertian::LambertianMaterial};
use objects::{hittable::HittableList, sphere::Sphere};
use vector::{Point, Vector};

fn main() {
    let lambertian_mat = Materials::Lambertian(LambertianMaterial::new(Vector::new(0.5, 0.5, 0.5)));

    let sphere1: Sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, lambertian_mat);
    let sphere2: Sphere = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0, lambertian_mat);

    let mut world: HittableList = HittableList::new();
    world.add_hittable(Box::new(sphere1));
    world.add_hittable(Box::new(sphere2));

    let camera: Camera = Camera::default();
    camera.render(world);
}
