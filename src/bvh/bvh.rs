use std::cmp::Ordering;

use crate::{
    bvh::aabb::{Aabb, merge_aabb},
    objects::hittable::{self, HitRecord, Hittable, HittableList},
    ray::Ray,
    utils::interval::{EMPTY_INTERVAL, Interval},
};

pub struct BvhNode {
    left_child: Option<Box<dyn Hittable>>,
    right_child: Option<Box<dyn Hittable>>,
    bounding_box: Aabb,
    hittable: Option<Box<dyn Hittable>>,
}

impl BvhNode {
    pub fn new_from_hittable(hittable: &Box<dyn Hittable>) -> BvhNode {
        BvhNode {
            left_child: None,
            right_child: None,
            bounding_box: hittable.get_aabb(),
            hittable: Some(hittable.clone_box()),
        }
    }

    pub fn new(hittable_list: &mut Vec<Box<dyn Hittable>>, start: usize, end: usize) -> BvhNode {
        let object_span = end - start;
        if object_span == 0 {
            BvhNode {
                left_child: None,
                right_child: None,
                bounding_box: Aabb::default(),
                hittable: None,
            }
        } else if object_span == 1 {
            let hittable = hittable_list[start].clone_box();

            BvhNode {
                left_child: None,
                right_child: None,
                bounding_box: hittable.get_aabb(),
                hittable: Some(hittable),
            }
        } else if object_span == 2 {
            let left_child = BvhNode::new_from_hittable(&hittable_list[start]);
            let right_child = BvhNode::new_from_hittable(&hittable_list[start + 1]);
            let bounding_box = merge_aabb(&left_child.get_aabb(), &right_child.get_aabb());

            BvhNode {
                left_child: Some(Box::new(left_child)),
                right_child: Some(Box::new(right_child)),
                bounding_box,
                hittable: None,
            }
        } else {
            // NOTE: Not going to overflow.. I'm not going to create that many
            let mid = (start + end) / 2;
            let mut bounding_box = Aabb::default();

            for i in start..end {
                bounding_box = merge_aabb(&bounding_box, &hittable_list[i].get_aabb());
            }
            let axis = bounding_box.get_longest_axis();

            let hittable_comparator = |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| -> Ordering {
                let (a_min, _) = a.get_aabb().get_axis_interval(axis).get_min_max();
                let (b_min, _) = b.get_aabb().get_axis_interval(axis).get_min_max();

                if a_min < b_min {
                    Ordering::Less
                } else if a_min > b_min {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            };

            hittable_list[start..end].sort_by(hittable_comparator);

            let left_child = BvhNode::new(hittable_list, start, mid);
            let right_child = BvhNode::new(hittable_list, mid, end);

            BvhNode {
                left_child: Some(Box::new(left_child)),
                right_child: Some(Box::new(right_child)),
                bounding_box,
                hittable: None,
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let bb_hit_interval: Option<Interval> = self.bounding_box.hit(ray, interval);
        if let None = bb_hit_interval {
            return None;
        }

        // NOTE: Hit something in this bb, must be either left or right or both
        match (&self.left_child, &self.right_child) {
            (Some(left), Some(right)) => {
                let left_hit: Option<HitRecord> = left.hit(ray, interval);

                match &left_hit {
                    Some(left_hitrecord) => {
                        let (min, _) = interval.get_min_max();
                        let valid_t_interval = Interval::new(min, left_hitrecord.get_t());

                        let right_hit: Option<HitRecord> = right.hit(ray, &valid_t_interval);

                        match right_hit {
                            Some(_) => right_hit,
                            None => left_hit,
                        }
                    }
                    // NOTE: If didn't hit left, must have hit right
                    None => right.hit(ray, interval),
                }
            }
            _ => {
                if let Some(hittable) = &self.hittable {
                    hittable.hit(ray, interval)
                } else {
                    None
                }
            }
        }
    }

    fn get_aabb(&self) -> Aabb {
        self.bounding_box
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for BvhNode {
    fn clone(&self) -> Self {
        BvhNode {
            left_child: self.left_child.as_ref().map(|l| l.clone_box()),
            right_child: self.right_child.as_ref().map(|r| r.clone_box()),
            bounding_box: self.bounding_box,
            hittable: self.hittable.clone(),
        }
    }
}
