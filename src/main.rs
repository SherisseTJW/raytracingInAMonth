mod bvh;
mod camera;
mod materials;
mod objects;
mod ray;
mod scene;
mod texture;
mod transformation;
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
    scene::{
        basic_light_scene::simple_light_scene,
        basic_scene::basic_scene,
        camera_fov_scene::camera_fov_scene,
        checker_ground_scene::checker_scene,
        cornell_box_scene::cornell_box_scene,
        cornell_smoke_scene::cornell_smoke_scene,
        earth_scene::earth_scene,
        perlin_earth_scene::perlin_earth_scene,
        perlin_scene::perlin_scene,
        quad_scene::quad_scene,
        week_render::week_scene,
        weekend_render::{self, weekend_scene},
    },
    texture::{
        checker::CheckerTexture, image::ImageTexture, perlin_noise, solid_color::SolidColorTexture,
        texture::Texture,
    },
    utils::{
        constants::PI,
        functions::{random_double, random_double_in_range},
    },
    vector::{Color, get_random_unit_vector},
};

fn main() {
    // NOTE: Scene 1 - Triple Spheres of different materials + Air Bubble within Glass Sphere
    // let basic_scene = basic_scene();
    // basic_scene.render();

    // NOTE: Scene 2 - Testing Camera FOV
    // let camera_fov_scene = camera_fov_scene();
    // camera_fov_scene.render()

    // NOTE: Final Render Scene 1 (From Ray Tracing in a Weekend)
    // let weekend_render = weekend_scene();
    // weekend_render.render();

    // NOTE: Final Render Scene 1 (From Ray Tracing in a Weekend)
    // but with a Checker Texture on the ground
    // let checker_scene = checker_scene();
    // checker_scene.render();

    // NOTE: Scene 5 - Earth
    // let earth_scene = earth_scene();
    // earth_scene.render();

    // NOTE: Perlin Noise ground + Main centre sphere
    // let perlin_scene = perlin_scene();
    // perlin_scene.render();

    // NOTE: Basic Quad Scene (Think a box but the sides are not connected)
    // let quad_scene = quad_scene();
    // quad_scene.render();

    // NOTE: Perlin Noise ground + Main centre sphere of Earth Image Texture
    // let perlin_earth = perlin_earth_scene();
    // perlin_earth.render();

    // NOTE: Simple Light Scene requiring a diffuse light
    // let simple_light_scene = simple_light_scene();
    // simple_light_scene.render();

    // NOTE: Cornell Box Scene
    // let cornell_box_scene = cornell_box_scene();
    // cornell_box_scene.render();

    // NOTE: Cornell Box Scene but with Smoke boxes
    // let cornell_smoke_scene = cornell_smoke_scene();
    // cornell_smoke_scene.render();

    // NOTE: Final Render Scene (From Ray Tracing, the Next Week)
    let week_render = week_scene();
    week_render.render();
}
