// NOTE: Reference Link: https://adrianb.io/2014/08/09/perlinnoise.html

use crate::{
    texture::texture::Texture,
    utils::functions::{random_double, random_int_in_range},
    vector::{Color, Point},
};

pub struct PerlinNoiseTexture {
    random_float: Vec<f64>,
    x_perm: Vec<i64>,
    y_perm: Vec<i64>,
    z_perm: Vec<i64>,
}

impl PerlinNoiseTexture {
    pub fn new(point_count: i64) -> PerlinNoiseTexture {
        let mut random_float = vec![];

        for i in 0..point_count {
            random_float.push(random_double());
        }

        let x_perm = Self::generate_perm(point_count);
        let y_perm = Self::generate_perm(point_count);
        let z_perm = Self::generate_perm(point_count);

        PerlinNoiseTexture {
            random_float,
            x_perm,
            y_perm,
            z_perm,
        }
    }

    fn gen_noise(&self, point: Point) -> f64 {
        let (x, y, z) = point.get_point();

        let i = ((4.0 * x) as i64 & 255) as usize;
        let j = ((4.0 * y) as i64 & 255) as usize;
        let k = ((4.0 * z) as i64 & 255) as usize;

        let idx = (self.x_perm[i] ^ self.y_perm[j] ^ self.z_perm[k]) as usize;
        self.random_float[idx]
    }

    fn generate_perm(point_count: i64) -> Vec<i64> {
        let mut perm = vec![];

        for i in 0..point_count {
            perm.push(i);
        }

        for i in (1..point_count).rev() {
            let cur = i as usize;
            let target = random_int_in_range(0, i) as usize;

            let tmp = perm[cur];
            perm[cur] = perm[target];
            perm[target] = tmp;
        }

        perm
    }
}

impl Texture for PerlinNoiseTexture {
    fn get_value(&self, u: f64, v: f64, point: Point) -> Color {
        let noise = self.gen_noise(point);
        Color::new(1.0, 1.0, 1.0).scale(noise)
    }
}
