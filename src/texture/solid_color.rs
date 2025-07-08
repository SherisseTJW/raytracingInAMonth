use crate::{
    texture::texture::Texture,
    vector::{Color, Point},
};

#[derive(Clone)]
pub struct SolidColorTexture {
    albedo: Color,
}

impl SolidColorTexture {
    pub fn new_from_rgb(r: f64, g: f64, b: f64) -> SolidColorTexture {
        SolidColorTexture {
            albedo: Color::new(r, g, b),
        }
    }

    pub fn new_from_color(color: Color) -> SolidColorTexture {
        SolidColorTexture { albedo: color }
    }
}

impl Texture for SolidColorTexture {
    fn get_value(&self, u: f64, v: f64, point: Point) -> Color {
        self.albedo
    }

    fn clone_box(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}
