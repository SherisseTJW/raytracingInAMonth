use crate::{
    materials::Materials,
    ray::Ray,
    utils::interval::Interval,
    vector::{Point, Vector, dot_product},
};

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord>;
}

#[derive(Clone, Copy)]
pub struct HitRecord {
    point: Point,
    normal: Vector,
    t: f64,
    front: bool,
    material: Materials,
}

impl HitRecord {
    pub fn new(
        point: Point,
        out_normal: Vector,
        t: f64,
        ray: &Ray,
        material: Materials,
    ) -> HitRecord {
        if hit_front(ray, out_normal) {
            HitRecord {
                point,
                normal: out_normal,
                t,
                front: true,
                material,
            }
        } else {
            HitRecord {
                point,
                normal: out_normal.negate(),
                t,
                front: false,
                material,
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
        self.material
    }
}

pub struct HittableList {
    // NOTE: https://stackoverflow.com/a/74974361
    hittable_list: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            hittable_list: vec![],
        }
    }

    pub fn add_hittable(&mut self, hittable: Box<dyn Hittable>) {
        self.hittable_list.push(hittable);
    }
}

impl Hittable for HittableList {
    // Return the HitRecord of the closest object that was hit
    // ( Blocks objects behind )
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let mut nearest_hit_record: Option<HitRecord> = None;

        for hittable in &self.hittable_list {
            if let Some(cur_record) = hittable.hit(ray, interval) {
                match nearest_hit_record {
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
}

pub fn hit_front(ray: &Ray, out_normal: Vector) -> bool {
    dot_product(ray.get_direction(), out_normal) < 0.0
}
