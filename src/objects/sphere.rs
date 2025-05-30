use crate::{
    ray::Ray,
    vector::{Point, dot_product},
};

use super::hittable::{HitRecord, Hittable};

pub struct Sphere {
    centre: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Point, radius: f64) -> Sphere {
        Sphere { centre, radius }
    }

    pub fn get_centre(&self) -> Point {
        self.centre
    }

    // pub fn hit(&self, ray: &Ray) -> bool {
    //     let ray_direction = ray.get_direction();
    //     let ray_origin = ray.get_origin();
    //
    //     let oc = self.centre.subv(ray_origin);
    //
    //     let a = dot_product(ray_direction, ray_direction);
    //     let b = -2.0 * dot_product(ray_direction, oc);
    //     let c = dot_product(oc, oc) - (self.radius * self.radius);
    //
    //     let discriminant = (b * b) - (4.0 * a * c);
    //
    //     // Either 1 real root or 2 real roots
    //     // i.e., hit at least 1 point on the surface of the sphere
    //     discriminant >= 0.0
    // }

    pub fn hit_at(&self, ray: &Ray) -> f64 {
        let ray_direction = ray.get_direction();
        let ray_origin = ray.get_origin();

        let oc = self.centre.subv(ray_origin);

        // NOTE: Using simplified intersection formula
        let a = dot_product(ray_direction, ray_direction);
        let h = dot_product(ray_direction, oc);
        let c = dot_product(oc, oc) - (self.radius * self.radius);

        let h_discriminant = (h * h) - (a * c);

        if h_discriminant < 0.0 {
            -1.0
        } else {
            (h - h_discriminant.sqrt()) / a
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let ray_direction = ray.get_direction();
        let ray_origin = ray.get_origin();

        let oc = self.centre.subv(ray_origin);

        // NOTE: Using simplified intersection formula
        let a = dot_product(ray_direction, ray_direction);
        let h = dot_product(ray_direction, oc);
        let c = dot_product(oc, oc) - (self.radius * self.radius);

        let h_discriminant = (h * h) - (a * c);

        if h_discriminant < 0.0 {
            None
        } else {
            let h_discriminant_sqrt = h_discriminant.sqrt();

            let neg_root = (h - h_discriminant_sqrt) / a;
            let pos_root = (h + h_discriminant_sqrt) / a;

            if (neg_root > t_min) && (neg_root < t_max) {
                let surface_vec = ray.at(neg_root);
                let surface_normal_vec = surface_vec.subv(self.get_centre()).unit();
                // let (x, y, z) = surface_normal_vec.get_point();

                Some(HitRecord::new(
                    surface_vec,
                    surface_normal_vec,
                    neg_root,
                    ray,
                ))
            } else if (pos_root > t_min) && (pos_root < t_max) {
                let surface_vec = ray.at(pos_root);
                let surface_normal_vec = surface_vec.subv(self.get_centre()).unit();
                // let (x, y, z) = surface_normal_vec.get_point();

                Some(HitRecord::new(
                    surface_vec,
                    surface_normal_vec,
                    pos_root,
                    ray,
                ))
            } else {
                None
            }
        }
    }
}
