pub mod default_material;
pub mod dielectric_material;
pub mod lambertian_material;
pub mod metal_material;

use std::fmt::Debug;

use crate::{hittable::Hit, ray::Ray};

pub trait Material: Debug {
    fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}

#[derive(Debug)]
pub struct Scatter {
    pub attenuation: glam::DVec3,
    pub scattered: Ray,
}

impl Scatter {
    pub fn new(attenuation: glam::DVec3, scattered: Ray) -> Self {
        Self {
            attenuation,
            scattered,
        }
    }
}
