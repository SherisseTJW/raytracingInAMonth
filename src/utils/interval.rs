use crate::utils::constants::F_INF;
use core::fmt::{Display, Formatter, Result};

#[derive(Clone, Copy)]
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval { min, max }
    }

    pub fn get_min_max(&self) -> (f64, f64) {
        (self.min, self.max)
    }

    pub fn get_size(&self) -> f64 {
        self.max - self.min
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

    pub fn expand(&self, delta: f64) -> Interval {
        let padding = delta / 2.0;

        Interval {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn offset(&self, offset: f64) -> Interval {
        let min = self.min + offset;
        let max = self.max + offset;

        Interval { min, max }
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

pub fn merge_interval(a: Interval, b: Interval) -> Interval {
    let (a_min, a_max) = a.get_min_max();
    let (b_min, b_max) = b.get_min_max();

    let min = if a_min <= b_min { a_min } else { b_min };

    let max = if a_max >= b_max { a_max } else { b_max };

    Interval { min, max }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Interval from {} to {}", self.min, self.max)
    }
}
