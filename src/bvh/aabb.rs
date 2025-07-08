// NOTE: Axis-Bounded Bounding Boxes

use crate::{ray::Ray, utils::interval::{merge_interval, Interval, EMPTY_INTERVAL}, vector::Point};

#[derive(Clone, Copy)]
pub struct Aabb {
    x: Interval,
    y: Interval,
    z: Interval
}

impl Aabb {
    pub fn new_from_interval(x: Interval, y: Interval, z: Interval) -> Aabb {
        Aabb { x, y, z }
    }

    pub fn new_from_extrema_points(a: Point, b: Point) -> Aabb {
        let (ax, ay, az) = a.get_point();
        let (bx, by, bz) = b.get_point();

        let x = if ax <= bx {
            Interval::new(ax, bx)
        } else {
            Interval::new(bx, ax)
        };

        let y = if ay <= by {
            Interval::new(ay, by)
        } else {
            Interval::new(by, ay)
        };

        let z = if az <= bz {
            Interval::new(az, bz)
        } else {
            Interval::new(bz, az)
        };

        Aabb { x, y, z }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<Interval> {
        let ray_origin = ray.get_origin();
        let ray_direction = ray.get_direction();

        let (r_min, r_max) = ray_t.get_min_max();

        let mut ray_t_min = r_min;
        let mut ray_t_max = r_max;

        for axis in 0..3 {
            let cur_axis_interval = self.get_axis_interval(axis);
            let inv_ray_direction = 1.0 / ray_direction.get_point_by_axis(axis);

            let (min, max) = cur_axis_interval.get_min_max(); 

            let t0 = (min - ray_origin.get_point_by_axis(axis)) * inv_ray_direction;
            let t1 = (max - ray_origin.get_point_by_axis(axis)) * inv_ray_direction;


            if t0 < t1 {
                ray_t_min = ray_t_min.max(t0);
                ray_t_max = ray_t_max.min(t1);
            }
            else {
                ray_t_min = ray_t_min.max(t1);
                ray_t_max = ray_t_max.min(t0);
            }

            if ray_t_max <= ray_t_min {
                return None
            }
        }

        Some(Interval::new(ray_t_min, ray_t_max))
    }

    pub fn get_axis_interval(&self, axis: i8) -> Interval {
        if axis == 0 {
            self.x
        }
        else if axis == 1 {
            self.y
        }
        else {
            self.z
        }
    }

    pub fn get_longest_axis(&self) -> i8 {
        let x_size = self.x.get_size();
        let y_size = self.y.get_size();
        let z_size = self.z.get_size();

        if x_size > y_size && x_size > z_size {
            0
        }
        else if y_size > x_size && y_size > z_size {
            1
        }
        else {
            2
        }
    }
}

impl Default for Aabb {
    fn default() -> Self {
        Aabb { x: EMPTY_INTERVAL, y: EMPTY_INTERVAL, z: EMPTY_INTERVAL }
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
        z: merge_interval(az, bz) 
    }
}
