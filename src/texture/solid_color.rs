use crate::{
    texture::texture::Texture,
    vector::{Color, Point},
};

#[derive(Clone)]
pub struct SolidColor {
    albedo: Color,
}

impl SolidColor {
    pub fn new_from_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            albedo: Color::new(r, g, b),
        }
    }

    pub fn new_from_color(color: Color) -> SolidColor {
        SolidColor { albedo: color }
    }
}

impl Texture for SolidColor {
    fn get_value(&self, u: f64, v: f64, point: Point) -> Color {
        self.albedo
    }

    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}
