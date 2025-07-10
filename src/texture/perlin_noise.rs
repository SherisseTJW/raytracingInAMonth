// NOTE: Reference Link: https://adrianb.io/2014/08/09/perlinnoise.html

use core::f64;
use std::{i64, isize};

use rand::{distr::weighted, seq::SliceRandom, thread_rng};

use crate::{
    texture::texture::Texture,
    utils::functions::{random_double, random_int_in_range},
    vector::{Color, Point, Vector, dot_product, get_random_unit_vector},
};

pub struct PerlinNoiseTexture {
    scale: f64,
    random_vector: Vec<Vector>,
    x_perm: Vec<u64>,
    y_perm: Vec<u64>,
    z_perm: Vec<u64>,
}

impl PerlinNoiseTexture {
    const POINT_COUNT: i64 = 256;

    pub fn new(scale: f64) -> PerlinNoiseTexture {
        let mut random_vector = vec![];

        for i in 0..Self::POINT_COUNT {
            random_vector.push(get_random_unit_vector());
        }

        let x_perm = Self::generate_perm();
        let y_perm = Self::generate_perm();
        let z_perm = Self::generate_perm();

        PerlinNoiseTexture {
            scale,
            random_vector,
            x_perm,
            y_perm,
            z_perm,
        }
    }

    fn gen_turbulence(&self, point: Point, depth: i32) -> f64 {
        let mut acc = 0.0;
        let mut tmp_point = point;
        let mut weight = 1.0;

        for i in 0..depth {
            acc += weight * self.gen_noise(&tmp_point);
            weight *= 0.5;
            tmp_point = tmp_point.scale(2.0);
        }

        acc.abs()
    }

    fn gen_noise(&self, point: &Point) -> f64 {
        let (x, y, z) = point.get_point();

        let fx = x.floor();
        let fy = y.floor();
        let fz = z.floor();

        let u = x - fx;
        let v = y - fy;
        let w = z - fz;

        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);

        let mut c = [[[get_random_unit_vector(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = ((fx as i64 + di) & 255) as usize;
                    let j = ((fy as i64 + dj) & 255) as usize;
                    let k = ((fz as i64 + dk) & 255) as usize;

                    c[di as usize][dj as usize][dk as usize] = self.random_vector
                        [(self.x_perm[i] ^ self.y_perm[j] ^ self.z_perm[k]) as usize];
                }
            }
        }

        let mut acc = 0.0;

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let fdi = di as f64;
                    let fdj = dj as f64;
                    let fdk = dk as f64;
                    let weighted_vec = Vector::new(u - fdi, v - fdj, w - fdk);

                    let i = (uu * fdi) + (1.0 - fdi) * (1.0 - uu);
                    let j = (vv * fdj) + (1.0 - fdj) * (1.0 - vv);
                    let k = (ww * fdk) + (1.0 - fdk) * (1.0 - ww);

                    acc += i * j * k * dot_product(c[di][dj][dk], weighted_vec);
                }
            }
        }

        acc
    }

    fn generate_perm() -> Vec<u64> {
        let mut perm = vec![];

        for i in 0..Self::POINT_COUNT {
            perm.push(i as u64);
        }

        perm.shuffle(&mut thread_rng());

        perm
    }

    fn simulate_white_noise(&self, point: Point, depth: i32) -> Color {
        let noise = self.gen_turbulence(point.scale(self.scale), depth);
        Color::new(1.0, 1.0, 1.0).scale(noise).scale(0.5)
    }

    fn simulate_marble_effect(&self, point: Point, depth: i32) -> Color {
        let (_, _, z) = point.get_point();
        let noise = self.gen_turbulence(point, depth);

        Color::new(1.0, 1.0, 1.0)
            .scale(1.0 + f64::sin(self.scale * z + 10.0 * noise))
            .scale(0.5)
    }
}

impl Texture for PerlinNoiseTexture {
    fn get_value(&self, u: f64, v: f64, point: Point) -> Color {
        // self.simulate_white_noise(point, 7)
        self.simulate_marble_effect(point, 7)
    }
}
