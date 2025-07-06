use std::cmp::Ordering;

use rand::{rng, Rng};

use crate::{bvh::aabb::{merge_aabb, Aabb}, objects::hittable::{HitRecord, Hittable, HittableList}, ray::Ray, utils::interval::Interval};

pub struct BvhNode {
    left_child: Option<Box<BvhNode>>,
    right_child: Option<Box<BvhNode>>,
    bounding_box: Aabb
}

impl BvhNode {
    pub fn new(hittable_list: Vec<Box<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        let mut rng = rng();
        let axis = rng.random_range(0..3);

        let hittable_comparator = |a: Box<dyn Hittable>, b: Box<dyn Hittable>| -> Ordering {
            let (a_min, _) = a.get_aabb().get_axis_interval(axis).get_min_max();
            let (b_min, _) = b.get_aabb().get_axis_interval(axis).get_min_max();

            if a_min < b_min {
                Ordering::Less
            }
            else if a_min > b_min {
                Ordering::Greater
            }
            else {
                Ordering::Equal
            }
        };

        let object_span = end - start;
        if object_span == 1 {
            BvhNode {
                left_child: None,
                right_child: None,
                bounding_box: hittable_list[start].get_aabb(),
            }
        }
        else if object_span == 2 {

        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let bb_hit_interval: Option<Interval> = self.bounding_box.hit(ray, interval);

        match bb_hit_interval {
            // NOTE: Hit something in this bb, must be either left or right or both
            Some(_) => {
                let left_hit: Option<HitRecord> = self.left_child.hit(ray, interval);

                match left_hit {
                    Some(left_hitrecord) => {
                        let (min, _) = interval.get_min_max();
                        let valid_t_interval = Interval::new(min, left_hitrecord.get_t()); 

                        let right_hit: Option<HitRecord> = self.right_child.hit(ray, &valid_t_interval);

                        match right_hit {
                            // NOTE: Sub-Primitive in both left AND right
                            Some(_) => right_hit,
                            // NOTE: Sub-Primitive only in left
                            None => left_hit
                        }
                    },
                    // NOTE: If didn't hit left, must have hit right
                    None => {
                        self.right_child.hit(ray, interval)
                    }
                }
            },
            // NOTE: Didn't hit anything in this bb at all
            None => {
                None
            }
        }
    }

    fn get_aabb(&self) -> Aabb {
        self.bounding_box
    }
}
