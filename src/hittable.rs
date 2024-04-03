use std::rc::Rc;

use glam::DVec3;

use crate::{graphics::materials::Material, math::Interval, ray::Ray};

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<Hit>;
}

#[derive(Debug)]
pub struct Hit {
    pub depth: f64,
    pub point: DVec3,
    pub normal: DVec3,
    pub front_face: bool,
    pub material: Rc<dyn Material>,
}

impl Hit {
    pub fn new(
        depth: f64,
        point: DVec3,
        normal: DVec3,
        front_face: bool,
        material: Rc<dyn Material>,
    ) -> Self {
        Self {
            depth,
            point,
            normal,
            front_face,
            material,
        }
    }
}

pub struct HittableCollection {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableCollection {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, ray: &Ray, interval: Interval) -> Option<Hit> {
        let mut closest_hit = None;
        let mut closest_so_far = interval.end;

        for object in &self.objects {
            if let Some(hit) = object.hit(ray, interval) {
                if hit.depth >= closest_so_far {
                    continue;
                }
                closest_so_far = hit.depth;
                closest_hit = Some(hit);
            }
        }

        closest_hit
    }
}
