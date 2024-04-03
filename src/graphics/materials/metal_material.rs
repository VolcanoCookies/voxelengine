use glam::DVec3;

use crate::{hittable::Hit, math::reflect, random::random_vec_unit, ray::Ray};

use super::{Material, Scatter};

#[derive(Debug)]
pub struct MetalMaterial {
    albedo: DVec3,
    fuzz: f64,
}

impl MetalMaterial {
    pub fn new(albedo: DVec3, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for MetalMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let reflected = reflect(ray.direction.normalize(), hit.normal);
        let scattered = Ray::new(hit.point, reflected + self.fuzz * random_vec_unit());
        let attenuation = self.albedo;

        if scattered.direction.dot(hit.normal) <= 0.0 {
            return None;
        }

        Some(Scatter::new(attenuation, scattered))
    }
}
