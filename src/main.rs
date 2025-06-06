mod camera;
mod objects;
mod ray;
mod utils;
mod vector;

use camera::Camera;
use objects::{hittable::HittableList, sphere::Sphere};
use vector::Point;

fn main() {
    let sphere1: Sphere = Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5);
    let sphere2: Sphere = Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0);

    let mut world: HittableList = HittableList::new();
    world.add_hittable(Box::new(sphere1));
    world.add_hittable(Box::new(sphere2));

    let camera: Camera = Camera::default();
    camera.render(world);
}
