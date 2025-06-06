use rand::Rng;

use crate::utils::constants::PI;

pub fn degrees_to_radians(degree: f64) -> f64 {
    (degree * PI) / 180.0
}

pub fn random_double() -> f64 {
    let mut rng = rand::rng();
    rng.random()
}

pub fn random_double_in_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::rng();
    min + (max - min) * rng.random_range(min..max)
}
