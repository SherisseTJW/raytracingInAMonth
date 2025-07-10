use std::sync::Arc;

use crate::vector::{Color, Point};

pub trait Texture: Send + Sync {
    fn get_value(&self, u: f64, v: f64, point: Point) -> Color;
}
