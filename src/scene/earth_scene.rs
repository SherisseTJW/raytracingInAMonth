// NOTE: Scene 5 - Earth

use std::sync::Arc;

use crate::{
    camera::Camera,
    materials::{Materials, lambertian::LambertianMaterial},
    objects::{hittable::HittableList, sphere::Sphere},
    scene::scene::Scene,
    texture::image::ImageTexture,
    vector::Point,
};

pub fn earth_scene() -> Scene {
    let centre_material = Materials::Lambertian(LambertianMaterial::new(Arc::new(
        ImageTexture::new("./texture_assets/earthmap.jpg"),
    )));

    let centre: Sphere = Sphere::new(Point::new(0.0, 0.0, 0.0), 0.3, centre_material);

    let mut hittable_list: HittableList = HittableList::new();
    hittable_list.add_hittable(Arc::new(centre));

    let camera = Camera::default();
    Scene::new(hittable_list, camera)
}
