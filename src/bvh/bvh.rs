use std::cmp::Ordering;

use rand::{rng, Rng};

use crate::{bvh::aabb::{merge_aabb, Aabb}, objects::hittable::{HitRecord, Hittable, HittableList}, ray::Ray, utils::interval::Interval};

pub struct BvhNode {
    left_child: Option<Box<BvhNode>>,
    right_child: Option<Box<BvhNode>>,
    bounding_box: Aabb
}

impl BvhNode {
    pub fn new_from_hittable(hittable: &Box<dyn Hittable>) -> BvhNode {
        BvhNode { left_child: None, right_child: None, bounding_box: hittable.get_aabb() }
    }

    pub fn new(hittable_list: &mut Vec<Box<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        let mut rng = rng();
        let axis = rng.random_range(0..3);

        let object_span = end - start;
        if object_span == 1 {
            BvhNode {
                left_child: None,
                right_child: None,
                bounding_box: hittable_list[start].get_aabb(),
            }
        }
        else if object_span == 2 {
            let left_child = BvhNode::new_from_hittable(&hittable_list[start]);
            let right_child = BvhNode::new_from_hittable(&hittable_list[start + 1]);
            let bounding_box = merge_aabb(&left_child.get_aabb(), &right_child.get_aabb());

            BvhNode {
                left_child: Some(Box::new(left_child)),
                right_child: Some(Box::new(right_child)),
                bounding_box
            }
        }
        else {
            // NOTE: Unlikely to integer overflow.. I don't think we have that many hittables
            let mid = (start + end) / 2;

            let hittable_comparator = |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| -> Ordering {
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

            hittable_list[start..end].sort_by(hittable_comparator);

            let left_child = BvhNode::new(hittable_list, start, mid);
            let right_child = BvhNode::new(hittable_list, mid, end);
            let bounding_box = merge_aabb(&left_child.get_aabb(), &right_child.get_aabb());

            BvhNode {
                left_child: Some(Box::new(left_child)),
                right_child: Some(Box::new(right_child)),
                bounding_box
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let bb_hit_interval: Option<Interval> = self.bounding_box.hit(ray, interval);
        if let None = bb_hit_interval {
            return None
        }

        // NOTE: Hit something in this bb, must be either left or right or both
        match (&self.left_child, &self.right_child) {
            (None, None) => None, 
            (Some(left), None) => {
                left.hit(ray, interval)
            }
            (None, Some(right)) => {
                right.hit(ray, interval)
            }
            (Some(left), Some(right)) => {
                let left_hit: Option<HitRecord> = left.hit(ray, interval);

                match left_hit {
                    Some(left_hitrecord) => {
                        let (min, _) = interval.get_min_max();
                        let valid_t_interval = Interval::new(min, left_hitrecord.get_t()); 

                        let right_hit: Option<HitRecord> = right.hit(ray, &valid_t_interval);

                        match right_hit {
                            // NOTE: Sub-Primitive in both left AND right
                            Some(_) => right_hit,
                            // NOTE: Sub-Primitive only in left
                            None => left_hit
                        }
                    },
                    // NOTE: If didn't hit left, must have hit right
                    None => {
                        right.hit(ray, interval)
                    }
                }
            }
        }
    }

    fn get_aabb(&self) -> Aabb {
        self.bounding_box
    }
}
