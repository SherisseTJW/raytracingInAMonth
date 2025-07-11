use std::{fmt::Display, sync::Arc};

use crate::vector::{Color, Point};

pub trait Texture: Send + Sync + Display {
    fn get_value(&self, u: f64, v: f64, point: Point) -> Color;
}
