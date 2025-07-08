use crate::{
    texture::{solid_color::SolidColorTexture, texture::Texture},
    vector::{Color, Point},
};

#[derive(Clone)]
pub struct CheckerTexture {
    even_texture: Box<dyn Texture>,
    odd_texture: Box<dyn Texture>,
    scale: f64,
}

impl CheckerTexture {
    pub fn new(
        even_texture: Box<dyn Texture>,
        odd_texture: Box<dyn Texture>,
        scale: f64,
    ) -> CheckerTexture {
        CheckerTexture {
            even_texture,
            odd_texture,
            scale,
        }
    }

    pub fn new_from_solid_color(even_color: Color, odd_color: Color, scale: f64) -> CheckerTexture {
        let even_texture = Box::new(SolidColorTexture::new_from_color(even_color));
        let odd_texture = Box::new(SolidColorTexture::new_from_color(odd_color));

        CheckerTexture {
            even_texture,
            odd_texture,
            scale,
        }
    }
}

impl Texture for CheckerTexture {
    fn get_value(&self, u: f64, v: f64, point: Point) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}
