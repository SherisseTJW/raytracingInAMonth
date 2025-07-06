use crate::{
    materials::Materials, bvh::aabb::{merge_aabb, Aabb}, ray::Ray, utils::interval::Interval, vector::{dot_product, Point, Vector}
};

use super::hittable::{HitRecord, Hittable};

pub struct Sphere {
    centre: Ray,
    radius: f64,
    material: Materials,
    bounding_box: Aabb
}

impl Sphere {
    pub fn new(static_centre: Point, radius: f64, material: Materials) -> Sphere {
        let centre = Ray::new(static_centre, Vector::new(0.0, 0.0, 0.0), None);

        let radius_vector = Vector::new(radius, radius, radius);
        let bounding_box = Aabb::new_from_extrema_points(static_centre.subv(radius_vector), static_centre.addv(radius_vector));

        Sphere {
            centre,
            radius,
            material,
            bounding_box
        }
    }

    pub fn new_moving_sphere(
        start_centre: Point,
        end_centre: Point,
        radius: f64,
        material: Materials,
    ) -> Sphere {
        let centre = Ray::new(start_centre, start_centre.subv(end_centre), None);

        let radius_vector = Vector::new(radius, radius, radius);
        let start_aabb = Aabb::new_from_extrema_points(centre.at(0.0).subv(radius_vector), centre.at(0.0).subv(radius_vector));
        let end_aabb = Aabb::new_from_extrema_points(centre.at(1.0).subv(radius_vector), centre.at(1.0).subv(radius_vector));
        let bounding_box =  merge_aabb(start_aabb, end_aabb);

        Sphere {
            centre,
            radius,
            material,
            bounding_box
        }
    }

    pub fn get_centre(&self) -> Ray {
        self.centre
    }
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

    fn get_aabb(&self) -> Aabb {
        self.bounding_box
    }
}
