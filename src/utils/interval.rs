use crate::utils::constants::F_INF;

pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn contains(&self, val: f64) -> bool {
        val >= self.min && val <= self.max
    }

    pub fn surrounds(&self, val: f64) -> bool {
        val > self.min && val < self.max
    }

    pub fn clamp(&self, val: f64) -> f64 {
        if val < self.min {
            self.min
        } else if val > self.max {
            self.max
        } else {
            val
        }
    }

    pub const fn get_empty_interval() -> Interval {
        Interval {
            min: F_INF,
            max: -F_INF,
        }
    }

    pub const fn get_universal_interval() -> Interval {
        Interval {
            min: -F_INF,
            max: F_INF,
        }
    }
}

pub const EMPTY_INTERVAL: Interval = Interval::get_empty_interval();
pub const UNIVERSAL_INTERVAL: Interval = Interval::get_universal_interval();
