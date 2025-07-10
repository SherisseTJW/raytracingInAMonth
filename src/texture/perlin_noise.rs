// NOTE: Reference Link: https://adrianb.io/2014/08/09/perlinnoise.html

use core::f64;
use std::{i64, isize};

use rand::{seq::SliceRandom, thread_rng};

use crate::{
    texture::texture::Texture,
    utils::functions::{random_double, random_int_in_range},
    vector::{Color, Point},
};

pub struct PerlinNoiseTexture {
    random_float: Vec<f64>,
    x_perm: Vec<u64>,
    y_perm: Vec<u64>,
    z_perm: Vec<u64>,
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

        let fx = x.floor();
        let fy = y.floor();
        let fz = z.floor();

        let u = x - fx;
        let v = y - fy;
        let w = z - fz;

        let mut c = [[[0.0; 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let i = ((fx as i64 + di) & 255) as usize;
                    let j = ((fy as i64 + dj) & 255) as usize;
                    let k = ((fz as i64 + dk) & 255) as usize;

                    c[di as usize][dj as usize][dk as usize] = self.random_float
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

                    let i = (u * fdi) + (1.0 - fdi) * (1.0 - u);
                    let j = (v * fdj) + (1.0 - fdj) * (1.0 - v);
                    let k = (w * fdk) + (1.0 - fdk) * (1.0 - w);

                    acc += i * j * k * c[di][dj][dk];
                }
            }
        }

        acc
    }

    fn generate_perm(point_count: i64) -> Vec<u64> {
        let mut perm = vec![];

        for i in 0..point_count {
            perm.push(i as u64);
        }

        perm.shuffle(&mut thread_rng());

        perm
    }
}

impl Texture for PerlinNoiseTexture {
    fn get_value(&self, u: f64, v: f64, point: Point) -> Color {
        let noise = self.gen_noise(point);
        Color::new(1.0, 1.0, 1.0).scale(noise)
    }
}
