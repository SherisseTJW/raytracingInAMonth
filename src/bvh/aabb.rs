// NOTE: Axis-Bounded Bounding Boxes

use crate::{
    ray::Ray,
    utils::{
        constants::F_INF,
        functions::degrees_to_radians,
        interval::{EMPTY_INTERVAL, Interval, merge_interval},
    },
    vector::{Point, Vector},
};
use core::{
    f64,
    fmt::{Display, Formatter, Result},
};

#[derive(Clone, Copy)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Aabb {
    pub fn new_from_interval(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb {
            x: Self::pad_axis(&x),
            y: Self::pad_axis(&y),
            z: Self::pad_axis(&z),
        }
    }

    pub fn new_from_extrema_points(a: Point, b: Point) -> Aabb {
        let (ax, ay, az) = a.get_point();
        let (bx, by, bz) = b.get_point();

        let x = Interval::new(f64::min(ax, bx), f64::max(ax, bx));
        let y = Interval::new(f64::min(ay, by), f64::max(ay, by));
        let z = Interval::new(f64::min(az, bz), f64::max(az, bz));

        Aabb {
            x: Self::pad_axis(&x),
            y: Self::pad_axis(&y),
            z: Self::pad_axis(&z),
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<Interval> {
        let ray_origin = ray.get_origin();
        let ray_direction = ray.get_direction();

        let (r_min, r_max) = ray_t.get_min_max();

        let mut ray_t_min = r_min;
        let mut ray_t_max = r_max;

        for axis in 0..3 {
            let cur_axis_interval = self.get_axis_interval(axis);
            let (min, max) = cur_axis_interval.get_min_max();

            let inv_ray_direction = 1.0 / ray_direction.get_point_by_axis(axis);

            let t0 = (min - ray_origin.get_point_by_axis(axis)) * inv_ray_direction;
            let t1 = (max - ray_origin.get_point_by_axis(axis)) * inv_ray_direction;

            if t0 < t1 {
                ray_t_min = f64::max(ray_t_min, t0);
                ray_t_max = f64::min(ray_t_max, t1);
            } else {
                ray_t_min = f64::max(ray_t_min, t1);
                ray_t_max = f64::min(ray_t_max, t0);
            }

            if ray_t_max <= ray_t_min {
                return None;
            }
        }

        Some(Interval::new(ray_t_min, ray_t_max))
    }

    pub fn translate(&self, offset: Vector) -> Aabb {
        let (x_offset, y_offset, z_offset) = offset.get_point();

        let x = self.x.offset(x_offset);
        let y = self.y.offset(y_offset);
        let z = self.z.offset(z_offset);

        // Pad if needed, unlikely but just in case
        Aabb::new_from_interval(x, y, z)
    }

    pub fn rotate(&self, x_rotation: f64, y_rotation: f64, z_rotation: f64) -> Aabb {
        let radians = degrees_to_radians(y_rotation);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let (x_min, x_max) = self.x.get_min_max();
        let (y_min, y_max) = self.y.get_min_max();
        let (z_min, z_max) = self.z.get_min_max();

        let mut min_x = F_INF;
        let mut min_y = F_INF;
        let mut min_z = F_INF;
        let mut max_x = -F_INF;
        let mut max_y = -F_INF;
        let mut max_z = -F_INF;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * x_max + (1 - i) as f64 * x_min;
                    let y = j as f64 * y_max + (1 - j) as f64 * y_min;
                    let z = k as f64 * z_max + (1 - k) as f64 * z_min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    min_x = f64::min(min_x, new_x);
                    min_y = f64::min(min_y, y);
                    min_z = f64::min(min_z, new_z);

                    max_x = f64::max(max_x, new_x);
                    max_y = f64::max(max_y, y);
                    max_z = f64::max(max_z, new_z);
                }
            }
        }

        Aabb::new_from_extrema_points(
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        )
    }

    pub fn get_axis_interval(&self, axis: i8) -> Interval {
        if axis == 0 {
            self.x
        } else if axis == 1 {
            self.y
        } else {
            self.z
        }
    }

    pub fn get_longest_axis(&self) -> i8 {
        let x_size = self.x.get_size();
        let y_size = self.y.get_size();
        let z_size = self.z.get_size();

        if x_size > y_size && x_size > z_size {
            0
        } else if y_size > x_size && y_size > z_size {
            1
        } else {
            2
        }
    }

    fn pad_axis(axis: &Interval) -> Interval {
        let delta: f64 = 0.001;

        if axis.get_size() < delta {
            axis.expand(delta)
        } else {
            *axis
        }
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Aabb {
            x: EMPTY_INTERVAL,
            y: EMPTY_INTERVAL,
            z: EMPTY_INTERVAL,
        }
    }
}

pub fn merge_aabb(a: &Aabb, b: &Aabb) -> Aabb {
    let ax = a.get_axis_interval(0);
    let ay = a.get_axis_interval(1);
    let az = a.get_axis_interval(2);

    let bx = b.get_axis_interval(0);
    let by = b.get_axis_interval(1);
    let bz = b.get_axis_interval(2);

    Aabb {
        x: merge_interval(ax, bx),
        y: merge_interval(ay, by),
        z: merge_interval(az, bz),
    }
}

impl Display for Aabb {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "Aabb with:\nx-axis: {}\ny-axis: {}\nz-axis: {}",
            self.x, self.y, self.z
        )
    }
}
