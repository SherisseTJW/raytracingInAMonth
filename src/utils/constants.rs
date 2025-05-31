use std::f64;

// Math
pub const F_INF: f64 = f64::INFINITY;
pub const PI: f64 = f64::consts::PI;

// Functions
pub fn degrees_to_radians(degree: f64) -> f64 {
    (degree * PI) / 180.0
}
