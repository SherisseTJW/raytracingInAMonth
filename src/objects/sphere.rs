use crate::{
    materials::Materials,
    ray::Ray,
    utils::interval::Interval,
    vector::{Point, Vector, dot_product},
};

use super::hittable::{HitRecord, Hittable};

pub struct Sphere {
    centre: Ray,
    radius: f64,
    material: Materials,
}

impl Sphere {
    pub fn new(static_centre: Point, radius: f64, material: Materials) -> Sphere {
        let centre = Ray::new(static_centre, Vector::new(0.0, 0.0, 0.0), None);

        Sphere {
            centre,
            radius,
            material,
        }
    }

    pub fn new_moving_sphere(
        start_centre: Point,
        end_centre: Point,
        radius: f64,
        material: Materials,
    ) -> Sphere {
        let centre = Ray::new(start_centre, start_centre.subv(end_centre), None);

        Sphere {
            centre,
            radius,
            material,
        }
    }

    pub fn get_centre(&self) -> Ray {
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
    //
    // pub fn hit_at(&self, ray: &Ray) -> f64 {
    //     let current_centre = self.centre.at(ray.get_time());
    //     let ray_direction = ray.get_direction();
    //     let ray_origin = ray.get_origin();
    //
    //     let oc = current_centre.subv(ray_origin);
    //
    //     // NOTE: Using simplified intersection formula
    //     let a = dot_product(ray_direction, ray_direction);
    //     let h = dot_product(ray_direction, oc);
    //     let c = dot_product(oc, oc) - (self.radius * self.radius);
    //
    //     let h_discriminant = (h * h) - (a * c);
    //
    //     if h_discriminant < 0.0 {
    //         -1.0
    //     } else {
    //         (h - h_discriminant.sqrt()) / a
    //     }
    // }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: &Interval) -> Option<HitRecord> {
        let current_centre = self.centre.at(ray.get_time());
        let ray_direction = ray.get_direction();
        let ray_origin = ray.get_origin();

        let oc = current_centre.subv(ray_origin);

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

            if interval.surrounds(neg_root) {
                let surface_vec = ray.at(neg_root);
                let surface_normal_vec = surface_vec.subv(current_centre).unit();

                Some(HitRecord::new(
                    surface_vec,
                    surface_normal_vec,
                    neg_root,
                    ray,
                    self.material,
                ))
            } else if interval.surrounds(pos_root) {
                let surface_vec = ray.at(pos_root);
                let surface_normal_vec = surface_vec.subv(current_centre).unit();

                Some(HitRecord::new(
                    surface_vec,
                    surface_normal_vec,
                    pos_root,
                    ray,
                    self.material,
                ))
            } else {
                None
            }
        }
    }
}
