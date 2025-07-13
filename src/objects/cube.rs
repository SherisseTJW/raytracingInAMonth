use std::sync::Arc;

use crate::{
    materials::Materials,
    objects::{hittable::HittableList, quad::Quad},
    vector::{Point, Vector},
};

pub struct Cube {
    sides: Vec<Quad>,
}

impl Cube {
    pub fn new(vertex_1: Point, vertex_2: Point, material: Materials) -> Cube {
        let (ax, ay, az) = vertex_1.get_point();
        let (bx, by, bz) = vertex_2.get_point();

        let min_x = f64::min(ax, bx);
        let min_y = f64::min(ay, by);
        let min_z = f64::min(az, bz);
        let max_x = f64::max(ax, bx);
        let max_y = f64::max(ay, by);
        let max_z = f64::max(az, bz);

        let dx = Vector::new(max_x - min_x, 0.0, 0.0);
        let dy = Vector::new(0.0, max_y - min_y, 0.0);
        let dz = Vector::new(0.0, 0.0, max_z - min_z);

        let mut sides = vec![];

        // Front
        sides.push(Quad::new(
            Point::new(min_x, min_y, max_z),
            dx,
            dy,
            material.clone(),
        ));

        // Right
        sides.push(Quad::new(
            Point::new(max_x, min_y, max_z),
            dz.negate(),
            dy,
            material.clone(),
        ));

        // Back
        sides.push(Quad::new(
            Point::new(max_x, min_y, min_z),
            dx.negate(),
            dy,
            material.clone(),
        ));

        // Left
        sides.push(Quad::new(
            Point::new(min_x, min_y, min_z),
            dz,
            dy,
            material.clone(),
        ));

        // Top
        sides.push(Quad::new(
            Point::new(min_x, max_y, max_z),
            dx,
            dz.negate(),
            material.clone(),
        ));

        // Bottom
        sides.push(Quad::new(
            Point::new(min_x, min_y, min_z),
            dx,
            dz,
            material.clone(),
        ));

        Cube { sides }
    }

    pub fn to_hittable_list(self) -> HittableList {
        let mut hittable_list: HittableList = HittableList::new();

        for side in self.sides {
            hittable_list.add_hittable(Arc::new(side));
        }

        hittable_list
    }
}
