use crate::vector::{Color, Point, Vector};

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn get_origin(&self) -> Point {
        self.origin
    }

    pub fn get_direction(&self) -> Vector {
        self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin.addv(self.direction.scale(t))
    }
}

pub fn blue_gradient_vertical(ray: Ray) -> Color {
    let unit_direction = ray.get_direction().unit();
    let (_, y, _) = unit_direction.get_point();

    let a = 0.5 * (y + 1.0);

    let blue = Color::new(0.5, 0.7, 1.0);
    let white = Color::new(1.0, 1.0, 1.0);

    white.scale(1.0 - a).addv(blue.scale(a))
}
