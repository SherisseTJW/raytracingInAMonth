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
    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
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
}

#[derive(Clone)]
pub struct HittableList {
    // NOTE: https://stackoverflow.com/a/74974361
    hittable_list: Vec<Box<dyn Hittable>>,
    bounding_box: Aabb,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            hittable_list: vec![],
            bounding_box: Aabb::default(),
        }
    }

    pub fn add_hittable(&mut self, hittable: Box<dyn Hittable>) {
        let new_bounding_box = merge_aabb(&self.bounding_box, &hittable.get_aabb());
        self.bounding_box = new_bounding_box;

        self.hittable_list.push(hittable);
    }

    pub fn get_num_hittables(&self) -> usize {
        self.hittable_list.len()
    }

    pub fn get_hittables(self) -> Vec<Box<dyn Hittable>> {
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

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

pub fn hit_front(ray: &Ray, out_normal: Vector) -> bool {
    dot_product(ray.get_direction(), out_normal) < 0.0
}
