use crate::vector::{Color, Point};

pub trait Texture: Send + Sync {
    fn get_value(&self, u: f64, v: f64, point: Point) -> Color;
    fn clone_box(&self) -> Box<dyn Texture>;
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Box<dyn Texture> {
        self.clone_box()
    }
}
