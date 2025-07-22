use crate::vector::{Color, Point};

pub trait Emission {
    fn emit(&self, u: f64, v: f64, point: Point) -> Color;
}
