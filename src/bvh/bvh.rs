use crate::{bvh::aabb::Aabb, objects::hittable::{HitRecord, Hittable, HittableList}, ray::Ray, utils::interval::Interval};

pub struct BvhNode {
    left_child: Box<dyn Hittable>,
    right_child: Box<dyn Hittable>,
    bounding_box: Aabb
}

impl BvhNode {
    pub fn new(hittable_list: Vec<Box<dyn Hittable>>, start: usize, end: usize) -> BvhNode {

    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let bb_hit_interval: Option<Interval> = self.bounding_box.hit(ray, interval);

        match bb_hit_interval {
            Some(hit_interval) => {

            },
            None => {
                None
            }
        }
    }

    fn get_aabb(&self) -> Aabb {
        self.bounding_box
    }
}
