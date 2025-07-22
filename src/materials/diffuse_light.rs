use std::sync::Arc;

use crate::{
    materials::emission::Emission,
    texture::texture::Texture,
    vector::{Color, Point},
};

#[derive(Clone)]
pub struct DiffuseLightMaterial {
    texture: Arc<dyn Texture>,
}

impl DiffuseLightMaterial {
    pub fn new(texture: Arc<dyn Texture>) -> DiffuseLightMaterial {
        DiffuseLightMaterial { texture }
    }
}

impl Emission for DiffuseLightMaterial {
    fn emit(&self, u: f64, v: f64, point: Point) -> Color {
        self.texture.get_value(u, v, point)
    }
}
