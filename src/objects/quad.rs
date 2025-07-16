use std::{sync::Arc, thread::panicking};

use crate::{
    bvh::aabb::{Aabb, merge_aabb},
    materials::{Materials, lambertian::LambertianMaterial},
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
        let bottomleft = q;
        let topleft = q.addv(u);
        let bottomright = q.addv(v);
        let topright = q.addv(u).addv(v);

        let bounding_box_diagonal1: Aabb = Aabb::new_from_extrema_points(bottomleft, topright);
        let bounding_box_diagonal2: Aabb = Aabb::new_from_extrema_points(topleft, bottomright);
        let bounding_box = merge_aabb(&bounding_box_diagonal1, &bounding_box_diagonal2);

        let n = cross_product(u, v);
        let normal = n.unit();
        let d: f64 = dot_product(normal, q);

        let w = n.scale(1.0 / dot_product(n, n));

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
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let nd = dot_product(self.normal, ray.get_direction());
        if nd.abs() < 1e-8_f64 {
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

        let unit_interval = Interval::new(0.0, 1.0);
        if unit_interval.contains(alpha) && unit_interval.contains(beta) {
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
