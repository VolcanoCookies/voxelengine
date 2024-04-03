use glam::{DAffine3, DVec3, Vec3};
use rand::Rng;

use crate::{hittable::HittableCollection, math::Interval, ray::Ray};

#[derive(Debug)]
pub struct Camera {
    pub transform: DAffine3,
    pub vertical_fov: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub viewport: Viewport,
    rng: rand::rngs::ThreadRng,
}

impl Camera {
    pub fn new(transform: DAffine3, vertical_fov: f64, width: i32, height: i32) -> Self {
        Self {
            transform,
            vertical_fov,
            image_width: width,
            image_height: height,
            viewport: Viewport::new(1.0, 100),
            rng: rand::thread_rng(),
        }
    }

    const SAMPLE_COUNT: i32 = 25;

    pub fn initialize(&mut self) {
        let aspect_ratio = self.image_width as f64 / self.image_height as f64;

        let focal_length = 1.0;
        let theta = self.vertical_fov.to_radians();
        let half_height = (theta / 2.0).tan();
        let viewport_height = 2.0 * half_height;
        let viewport_width = aspect_ratio * viewport_height;

        let local_viewport_u = DVec3::new(viewport_width, 0.0, 0.0);
        let local_viewport_v = DVec3::new(0.0, -viewport_height, 0.0);

        let viewport_u = self.transform.transform_point3(local_viewport_u)
            - self.transform.z_axis * focal_length;
        let viewport_v = self.transform.transform_point3(local_viewport_v)
            - self.transform.z_axis * focal_length;

        println!("Viewport U: {:?}", viewport_u);
        println!("Viewport V: {:?}", viewport_v);

        let diff_u = viewport_u - self.transform.translation;
        let diff_v = viewport_v - self.transform.translation;

        println!("Diff U: {:?}", diff_u);
        println!("Diff V: {:?}", diff_v);

        let pixel_du = local_viewport_u / self.image_width as f64;
        let pixel_dv = local_viewport_v / self.image_height as f64;
        let local_origin = DVec3::new(-viewport_width / 2.0, viewport_height / 2.0, -focal_length);
        let pixel_00 = local_origin + (pixel_du + pixel_dv) / 2.0;

        println!("Pixel DU: {:?}", pixel_du);
        println!("Pixel DV: {:?}", pixel_dv);
        println!("Pixel 00: {:?}", pixel_00);
        println!("Origin: {:?}", local_origin);

        let pixel_du = self.transform.transform_vector3(pixel_du);
        let pixel_dv = self.transform.transform_vector3(pixel_dv);
        let origin = self.transform.transform_point3(local_origin);
        let pixel_00 = self.transform.transform_point3(pixel_00);

        self.viewport = Viewport {
            aspect_ratio,
            image_width: self.image_width,
            image_height: self.image_height,
            width: viewport_width,
            height: viewport_height,
            focal_length,
            viewport_u,
            viewport_v,
            pixel_du,
            pixel_dv,
            pixel_00,
            origin,
        };
    }

    pub fn render(&mut self, objects: &HittableCollection) -> Vec<Vec3> {
        let mut pixels = Vec::new();

        for j in 0..self.viewport.image_height {
            for i in 0..self.viewport.image_width {
                let mut color = DVec3::ZERO;
                for _ in 0..Self::SAMPLE_COUNT {
                    let ray = self.get_ray(i as f64, j as f64);
                    color += self.ray_color(ray, objects, 10);
                }

                // Average color
                let color = color / Self::SAMPLE_COUNT as f64;
                // Go from linear to gamma space
                let color = linear_to_gamma(color);

                pixels.push(color.as_vec3());
            }
        }

        pixels
    }

    pub fn ray_color(&self, ray: Ray, objects: &HittableCollection, max_depth: i32) -> DVec3 {
        if max_depth <= 0 {
            return DVec3::ZERO;
        }

        if let Some(hit) = objects.hit(&ray, Interval::new(0.00001, f64::INFINITY)) {
            if let Some(scatter) = hit.material.scatter(&ray, &hit) {
                return scatter.attenuation
                    * self.ray_color(scatter.scattered, objects, max_depth - 1);
            } else {
                return DVec3::ZERO;
            }
        }

        let t = 0.5 * (ray.direction.y + 1.0);
        let start = DVec3::new(1.0, 1.0, 1.0);
        let end = DVec3::new(0.5, 0.7, 1.0);
        start.lerp(end, t)
    }

    fn get_ray(&mut self, u: f64, v: f64) -> Ray {
        let pixel_center =
            self.viewport.pixel_00 + self.viewport.pixel_du * u + self.viewport.pixel_dv * v;
        let pixel_sample = pixel_center + self.sample_square();

        Ray::new(
            self.transform.translation,
            pixel_sample - self.transform.translation,
        )
    }

    fn sample_square(&mut self) -> DVec3 {
        let ru = self.rng.gen::<f64>() - 0.5;
        let rv = self.rng.gen::<f64>() - 0.5;

        ru * self.viewport.pixel_du + rv * self.viewport.pixel_dv
    }
}

#[derive(Debug)]
pub struct Viewport {
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub image_height: i32,
    pub width: f64,
    pub height: f64,
    pub focal_length: f64,
    pub viewport_u: DVec3,
    pub viewport_v: DVec3,
    pub pixel_du: DVec3,
    pub pixel_dv: DVec3,
    pub pixel_00: DVec3,
    pub origin: DVec3,
}

impl Viewport {
    pub fn new(aspect_ratio: f64, image_width: i32) -> Self {
        let image_height = (image_width as f64 / aspect_ratio) as i32;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        let viewport_u = DVec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = DVec3::new(0.0, -viewport_height, 0.0);
        let pixel_du = viewport_u / image_width as f64;
        let pixel_dv = viewport_v / image_height as f64;
        let origin = DVec3::new(-viewport_width / 2.0, viewport_height / 2.0, 0.0);
        let pixel_00 = origin + (pixel_du + pixel_dv) / 2.0;

        Self {
            aspect_ratio,
            image_width,
            image_height,
            width: viewport_width,
            height: viewport_height,
            focal_length,
            viewport_u,
            viewport_v,
            pixel_du,
            pixel_dv,
            pixel_00,
            origin,
        }
    }
}

fn linear_to_gamma(color: DVec3) -> DVec3 {
    DVec3::new(color.x.sqrt(), color.y.sqrt(), color.z.sqrt())
}
