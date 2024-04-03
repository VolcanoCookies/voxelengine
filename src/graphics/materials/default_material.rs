use crate::{hittable::Hit, random::random_vec_unit, ray::Ray};

use super::{Material, Scatter};

#[derive(Debug)]
pub struct SolidColorMaterial {
    color: glam::DVec3,
}

impl Material for SolidColorMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let attenuation = self.color;
        let scattered = Ray::new(hit.point, hit.normal + random_vec_unit());
        Some(Scatter::new(attenuation, scattered))
    }
}
