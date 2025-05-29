use core::fmt::{Display, Formatter, Result};

pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Vector {
        Vector { x, y, z }
    }

    pub fn get_point(&self) -> (f64, f64, f64) {
        (self.x, self.y, self.z)
    }

    pub fn get_length(&self) -> f64 {
        let length_squared = (self.x * self.x) + (self.y * self.y) + (self.z * self.z);

        length_squared.sqrt()
    }

    pub fn scale(&self, scalar: f64) -> Vector {
        let new_x = self.x * scalar;
        let new_y = self.y * scalar;
        let new_z = self.z * scalar;

        Vector {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }

    pub fn addv(&self, vector: Vector) -> Vector {
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

    pub fn unit(&self) -> Vector {
        let length = self.get_length();
        self.scale(1.0 / length)
    }

    pub fn to_color(&self) -> [u8; 3] {
        fn f(val: f64) -> u8 {
            if val < 0.0 {
                0
            } else if val >= 1.0 {
                255
            } else {
                (255.8 * val) as u8
            }
        }

        [f(self.x), f(self.y), f(self.z)]
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

pub fn dot_product(u: Vector, v: Vector) -> Vector {
    let (u1, u2, u3) = u.get_point();
    let (v1, v2, v3) = v.get_point();

    Vector {
        x: u1 * v1,
        y: u2 * v2,
        z: u3 * v3,
    }
}
