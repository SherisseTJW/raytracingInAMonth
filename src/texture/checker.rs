use std::sync::Arc;
use crate::{
    texture::{solid_color::SolidColorTexture, texture::Texture},
    vector::{Color, Point},
};

#[derive(Clone)]
pub struct CheckerTexture {
    even_texture: Arc<dyn Texture>,
    odd_texture: Arc<dyn Texture>,
    inv_scale: f64,
}

impl CheckerTexture {
    pub fn new(
        even_texture: Arc<dyn Texture>,
        odd_texture: Arc<dyn Texture>,
        scale: f64,
    ) -> CheckerTexture {
        CheckerTexture {
            even_texture,
            odd_texture,
            inv_scale: 1.0 / scale,
        }
    }

    pub fn new_from_solid_color(even_color: Color, odd_color: Color, scale: f64) -> CheckerTexture {
        let even_texture = Arc::new(SolidColorTexture::new_from_color(even_color));
        let odd_texture = Arc::new(SolidColorTexture::new_from_color(odd_color));

        CheckerTexture {
            even_texture,
            odd_texture,
            inv_scale: 1.0 / scale,
        }
    }
}

impl Texture for CheckerTexture {
    fn get_value(&self, u: f64, v: f64, point: Point) -> Color {
        let (x, y, z) = point.get_point();

        let x_val = (self.inv_scale * x).floor() as i64;
        let y_val = (self.inv_scale * y).floor() as i64;
        let z_val = (self.inv_scale * z).floor() as i64;

        if (x_val + y_val + z_val) % 2 == 0 {
            self.odd_texture.get_value(u, v, point)
        }
        else {
            self.even_texture.get_value(u, v, point)
        }
    }

    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}
