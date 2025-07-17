use std::sync::Arc;

use crate::{
    bvh::aabb::{Aabb, merge_aabb},
    materials::Materials,
    ray::Ray,
    utils::interval::{EMPTY_INTERVAL, Interval},
    vector::{Point, Vector, dot_product},
};

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
    fn get_aabb(&self) -> Aabb;
    fn clone_box(&self) -> Arc<dyn Hittable>;
}

#[derive(Clone)]
pub struct HitRecord {
    point: Point,
    normal: Vector,
    t: f64,
    front: bool,
    material: Materials,
    u: f64,
    v: f64,
}

impl HitRecord {
    pub fn new(
        point: Point,
        out_normal: Vector,
        t: f64,
        ray: &Ray,
        material: Materials,
        u: f64,
        v: f64,
    ) -> HitRecord {
        if hit_front(ray, out_normal) {
            HitRecord {
                point,
                normal: out_normal,
                t,
                front: true,
                material,
                u,
                v,
            }
        } else {
            HitRecord {
                point,
                normal: out_normal.negate(),
                t,
                front: false,
                material,
                u,
                v,
            }
        }
    }

    pub fn get_point(&self) -> Point {
        self.point
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn get_normal(&self) -> Vector {
        self.normal
    }

    pub fn get_front(&self) -> bool {
        self.front
    }

    pub fn get_material(&self) -> Materials {
        self.material.clone()
    }

    pub fn get_texture_coordinates(&self) -> (f64, f64) {
        (self.u, self.v)
    }

    pub fn translate(&mut self, offset: Vector) {
        self.point.addv(offset);
    }

    pub fn rotate(&self, ray: &Ray, sin_theta: f64, cos_theta: f64) -> HitRecord {
        let (x_point, y_point, z_point) = self.point.get_point();
        let (x_normal, y_normal, z_normal) = self.normal.get_point();

        let new_point = Point::new(
            (x_point * cos_theta) + (z_point * sin_theta),
            y_point,
            (x_point * -sin_theta) + (z_point * cos_theta),
        );

        let new_normal = Vector::new(
            (x_normal * cos_theta) + (z_normal * sin_theta),
            y_normal,
            (x_normal * -sin_theta) + (z_normal * cos_theta),
        );

        HitRecord::new(
            new_point,
            new_normal,
            self.t,
            ray,
            self.material.clone(),
            self.u,
            self.v,
        )
    }
}

#[derive(Clone)]
pub struct HittableList {
    // NOTE: https://stackoverflow.com/a/74974361
    hittable_list: Vec<Arc<dyn Hittable>>,
    bounding_box: Aabb,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            hittable_list: vec![],
            bounding_box: Aabb::default(),
        }
    }

    pub fn add_hittable(&mut self, hittable: Arc<dyn Hittable>) {
        let new_bounding_box = merge_aabb(&self.bounding_box, &hittable.get_aabb());
        self.bounding_box = new_bounding_box;

        self.hittable_list.push(hittable);
    }

    pub fn add_hittable_list(&mut self, hittable_list: HittableList) {
        let new_hittables = hittable_list.get_hittables();

        for hittable in new_hittables {
            self.add_hittable(hittable);
        }
    }

    pub fn get_num_hittables(&self) -> usize {
        self.hittable_list.len()
    }

    pub fn get_hittables(self) -> Vec<Arc<dyn Hittable>> {
        self.hittable_list
    }
}

impl Hittable for HittableList {
    // Return the HitRecord of the closest object that was hit
    // ( Blocks objects behind )
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut nearest_hit_record: Option<HitRecord> = None;

        for hittable in &self.hittable_list {
            if let Some(cur_record) = hittable.hit(ray, interval) {
                match &nearest_hit_record {
                    Some(nearest_record) => {
                        let nearest_t = nearest_record.get_t();

                        if cur_record.get_t() < nearest_t {
                            nearest_hit_record = Some(cur_record);
                        }
                    }
                    None => nearest_hit_record = Some(cur_record),
                }
            }
        }

        nearest_hit_record
    }

    fn get_aabb(&self) -> Aabb {
        self.bounding_box
    }

    fn clone_box(&self) -> Arc<dyn Hittable> {
        Arc::new(self.clone())
    }
}

pub fn hit_front(ray: &Ray, out_normal: Vector) -> bool {
    dot_product(ray.get_direction(), out_normal) < 0.0
}
