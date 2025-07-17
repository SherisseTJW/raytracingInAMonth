use std::sync::Arc;

use crate::{
    materials::Materials,
    objects::{
        hittable::{Hittable, HittableList},
        quad::Quad,
    },
    transformation::{rotation::Rotation, translation::Translation},
    vector::{Point, Vector},
};

pub struct Cube {
    sides: HittableList,
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

        let mut sides = HittableList::new();

        // Front
        sides.add_hittable(Arc::new(Quad::new(
            Point::new(min_x, min_y, max_z),
            dx,
            dy,
            material.clone(),
        )));

        // Right
        sides.add_hittable(Arc::new(Quad::new(
            Point::new(max_x, min_y, max_z),
            dz.negate(),
            dy,
            material.clone(),
        )));

        // Back
        sides.add_hittable(Arc::new(Quad::new(
            Point::new(max_x, min_y, min_z),
            dx.negate(),
            dy,
            material.clone(),
        )));

        // Left
        sides.add_hittable(Arc::new(Quad::new(
            Point::new(min_x, min_y, min_z),
            dz,
            dy,
            material.clone(),
        )));

        // Top
        sides.add_hittable(Arc::new(Quad::new(
            Point::new(min_x, max_y, max_z),
            dx,
            dz.negate(),
            material.clone(),
        )));

        // Bottom
        sides.add_hittable(Arc::new(Quad::new(
            Point::new(min_x, min_y, min_z),
            dx,
            dz,
            material.clone(),
        )));

        Cube { sides }
    }

    pub fn translate(&self, offset: Vector) -> Translation {
        Translation::new(Arc::new(self.sides.clone()), offset)
    }

    pub fn rotate(&self, x_rotation: f64, y_rotation: f64, z_rotation: f64) -> Rotation {
        Rotation::new(
            Arc::new(self.sides.clone()),
            x_rotation,
            y_rotation,
            z_rotation,
        )
    }

    pub fn to_hittable_list(self) -> HittableList {
        self.sides
    }
}
