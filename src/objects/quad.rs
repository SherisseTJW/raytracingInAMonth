use std::sync::Arc;

use crate::{
    bvh::aabb::{Aabb, merge_aabb},
    materials::Materials,
    objects::hittable::{HitRecord, Hittable},
    ray::Ray,
    utils::interval::Interval,
    vector::{Point, Vector, cross_product, dot_product},
};

// NOTE: 2-D Plane
#[derive(Clone)]
pub struct Quad {
    q: Point, // Bottom left corner
    u: Vector,
    v: Vector,
    w: Vector,
    normal: Vector,
    d: f64,
    material: Materials,
    bounding_box: Aabb,
}

impl Quad {
    pub fn new(q: Point, u: Vector, v: Vector, material: Materials) -> Quad {
        let bounding_box_diagonal1: Aabb = Aabb::new_from_extrema_points(q, q.addv(u).addv(v));
        let bounding_box_diagonal2: Aabb = Aabb::new_from_extrema_points(q.addv(u), q.addv(v));
        let bounding_box = merge_aabb(&bounding_box_diagonal1, &bounding_box_diagonal2);

        let normal = cross_product(u, v).unit();
        let d: f64 = dot_product(normal, q);

        let w = normal.scale(1.0 / dot_product(normal, normal));

        Quad {
            q,
            u,
            v,
            w,
            normal,
            d,
            material,
            bounding_box,
        }
    }

    fn is_interior(alpha: f64, beta: f64) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);

        if unit_interval.contains(alpha) && unit_interval.contains(beta) {
            true
        } else {
            false
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let nd = dot_product(self.normal, ray.get_direction());
        if nd.abs() < 1e-8 {
            return None;
        }

        let t = (self.d - dot_product(self.normal, ray.get_origin())) / nd;
        if !interval.contains(t) {
            return None;
        }

        // NOTE: We still need to project onto the plane and get the intersection point in terms of the
        // basis of u and v (?)
        let intersection = ray.at(t);
        let hit_vector = intersection.subv(self.q);
        let alpha: f64 = dot_product(self.w, cross_product(hit_vector, self.v));
        let beta: f64 = dot_product(self.w, cross_product(self.u, hit_vector));

        if Self::is_interior(alpha, beta) {
            Some(HitRecord::new(
                intersection,
                self.normal,
                t,
                ray,
                self.material.clone(),
                alpha,
                beta,
            ))
        } else {
            None
        }
    }

    fn get_aabb(&self) -> Aabb {
        self.bounding_box
    }

    fn clone_box(&self) -> Arc<dyn Hittable> {
        Arc::new(self.clone())
    }
}
