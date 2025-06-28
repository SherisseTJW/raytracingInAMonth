use core::fmt::{Display, Formatter, Result};

use crate::utils::functions::random_double_in_range;

pub type Point = Vector;
pub type Color = Vector;

#[derive(Clone, Copy)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    // pub fn get_random_vector(self, min: f64, max: f64) -> Vector {
    //     Vector {
    //         x: random_double_in_range(min, max),
    //         y: random_double_in_range(min, max),
    //         z: random_double_in_range(min, max),
    //     }
    // }

    pub fn get_point(self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn get_length_squared(self) -> f64 {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn get_length(self) -> f64 {
        let length_squared = (self.x * self.x) + (self.y * self.y) + (self.z * self.z);

        length_squared.sqrt()
    }

    pub fn scale(self, scalar: f64) -> Vector {
        let new_x = self.x * scalar;
        let new_y = self.y * scalar;
        let new_z = self.z * scalar;

        Vector {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }

    pub fn multiply(self, u: Vector) -> Vector {
        let (x, y, z) = u.get_point();

        let new_x = self.x * x;
        let new_y = self.y * y;
        let new_z = self.z * z;

        Vector {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }

    pub fn addv(self, vector: Vector) -> Vector {
        let vector_point = vector.get_point();

        let new_x = self.x + vector_point.0;
        let new_y = self.y + vector_point.1;
        let new_z = self.z + vector_point.2;

        Vector {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }

    pub fn subv(self, vector: Vector) -> Vector {
        let vector_point = vector.get_point();

        let new_x = self.x - vector_point.0;
        let new_y = self.y - vector_point.1;
        let new_z = self.z - vector_point.2;

        Vector {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }

    pub fn negate(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn unit(self) -> Vector {
        let length = self.get_length();
        self.scale(1.0 / length)
    }

    pub fn near_zero(self) -> bool {
        let epsilon = 1e-8;

        self.x.abs() < epsilon && self.y.abs() < epsilon && self.z.abs() < epsilon
    }

    pub fn to_color(self) -> [u8; 3] {
        fn f(val: f64) -> u8 {
            if val < 0.0 {
                0
            } else if val >= 1.0 {
                255
            } else {
                (255.8 * val) as u8
            }
        }

        fn linear_to_gamma(val: f64) -> f64 {
            if val > 0.0 { val.sqrt() } else { 0.0 }
        }

        let r = linear_to_gamma(self.x);
        let g = linear_to_gamma(self.y);
        let b = linear_to_gamma(self.z);

        [f(r), f(g), f(b)]
    }
}

impl Display for Vector {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub fn cross_product(u: Vector, v: Vector) -> Vector {
    let (u1, u2, u3) = u.get_point();
    let (v1, v2, v3) = v.get_point();

    let new_x = (u2 * v3) - (u3 * v2);
    let new_y = (u3 * v1) - (u1 * v3);
    let new_z = (u1 * v2) - (u2 * v1);

    Vector {
        x: new_x,
        y: new_y,
        z: new_z,
    }
}

pub fn dot_product(u: Vector, v: Vector) -> f64 {
    let (u1, u2, u3) = u.get_point();
    let (v1, v2, v3) = v.get_point();

    (u1 * v1) + (u2 * v2) + (u3 * v3)
}

pub fn get_random_unit_vector() -> Vector {
    loop {
        let cur_vector = Vector::new(
            random_double_in_range(-1.0, 1.0),
            random_double_in_range(-1.0, 1.0),
            random_double_in_range(-1.0, 1.0),
        );
        // let cur_vector = self.get_random_vector(-1.0, 1.0);
        let length_squared = cur_vector.get_length_squared();

        if length_squared <= 1.0 && length_squared > 1e-160 {
            break cur_vector.scale(1.0 / length_squared.sqrt());
        }
    }
}

pub fn get_random_unit_vector_on_hemisphere(normal: Vector) -> Vector {
    let unit_vector = get_random_unit_vector();

    if dot_product(unit_vector, normal) > 0.0 {
        unit_vector
    } else {
        unit_vector.negate()
    }
}

pub fn get_random_vector_in_unit_disk() -> Vector {
    loop {
        let point = Point::new(
            random_double_in_range(-1.0, 1.0),
            random_double_in_range(-1.0, 1.0),
            0.0,
        );

        if point.get_length_squared() < 1.0 {
            break point;
        }
    }
}

pub fn reflect(u: Vector, normal: Vector) -> Vector {
    let b = dot_product(u, normal);

    u.subv(normal.scale(2.0 * b))
}

pub fn refract(r: Vector, normal: Vector, etai_over_etat: f64) -> Vector {
    let cos_theta = f64::min(dot_product(r.negate(), normal), 1.0);

    let r_out_perp: Vector = r.addv(normal.scale(cos_theta)).scale(etai_over_etat);
    let r_out_para: Vector = normal.scale(-f64::abs(1.0 - r_out_perp.get_length_squared()).sqrt());

    r_out_perp.addv(r_out_para)
}
