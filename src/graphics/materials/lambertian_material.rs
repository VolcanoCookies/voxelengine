use glam::DVec3;

use crate::{hittable::Hit, random::random_vec_unit, ray::Ray};

use super::{Material, Scatter};

#[derive(Debug)]
pub struct LambertianMaterial {
    albedo: DVec3,
}

impl LambertianMaterial {
    const SCATTER_LIMIT: f64 = 0.000000001;

    pub fn new(albedo: DVec3) -> Self {
        Self { albedo }
    }
}

impl Material for LambertianMaterial {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
        let mut scatter_dir = hit.normal + random_vec_unit();
        if scatter_dir.length_squared() < Self::SCATTER_LIMIT {
            scatter_dir = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_dir);
        let attenuation = self.albedo;
        Some(Scatter::new(attenuation, scattered))
    }
}
