use crate::{bvh::bvh::BvhNode, camera::Camera, objects::hittable::HittableList};

pub struct Scene {
    hittable_list: HittableList,
    camera: Camera
}

impl Scene {
    pub fn new(hittable_list: HittableList, camera: Camera) -> Scene {
        Scene { hittable_list, camera }
    }

    pub fn render(&self) {
        let size = self.hittable_list.get_num_hittables();
        let mut hittables = self.hittable_list.clone().get_hittables();

        self.camera.render(BvhNode::new(&mut hittables, 0 as usize, size));
    }
}
