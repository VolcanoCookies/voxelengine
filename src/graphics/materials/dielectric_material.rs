use glam::DVec3;

use crate::{
    hittable::Hit,
    math::{reflect, refract},
    ray::Ray,
};

use super::{Material, Scatter};

#[derive(Debug)]
pub struct DielectricMaterial {
    ref_idx: f64,
}

impl DielectricMaterial {
    pub fn new(ref_idx: f64) -> Self {
        Self { ref_idx }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for DielectricMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let attenuation = DVec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if hit.front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };
        let unit_direction = ray.direction.normalize();

        let cos_theta = (-unit_direction).dot(hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rand::random() {
                reflect(unit_direction, hit.normal)
            } else {
                refract(unit_direction, hit.normal, refraction_ratio)
            };

        let scattered = Ray::new(hit.point, direction);

        Some(Scatter::new(attenuation, scattered))
    }
}
