use std::rc::Rc;

use glam::DVec3;

use crate::{graphics::materials::Material, hittable::Hittable, math::Interval};

#[derive(Debug)]
pub struct Sphere {
    center: DVec3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Rc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, interval: Interval) -> Option<crate::hittable::Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;
        let disc = half_b * half_b - a * c;

        if disc < 0.0 {
            return None;
        }

        let squared = disc.sqrt();

        let mut root = (-half_b - squared) / a;
        if !interval.surrounds(root as f64) {
            root = (-half_b + squared) / a;
            if !interval.surrounds(root as f64) {
                return None;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let front_face = ray.direction.dot(normal) < 0.0;
        let normal = if front_face { normal } else { -normal };

        Some(crate::hittable::Hit::new(
            root,
            point,
            normal,
            front_face,
            self.material.clone(),
        ))
    }
}
