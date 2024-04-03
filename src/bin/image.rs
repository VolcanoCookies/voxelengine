use std::{fs::OpenOptions, io::Write, rc::Rc};

use glam::{DAffine3, DQuat, DVec3};
use voxelengine::{
    camera::Camera,
    graphics::materials::{
        dielectric_material::DielectricMaterial, lambertian_material::LambertianMaterial,
        metal_material::MetalMaterial,
    },
    hittable::HittableCollection,
    shapes::Sphere,
};

fn main() {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("image.ppm")
        .unwrap();

    let look_from = DVec3::new(-2.0, 2.0, 1.0);
    let look_at = DVec3::new(0.0, 0.0, -1.0);
    //let look_from = DVec3::new(0.0, 0.0, 1.0);
    //let look_at = DVec3::new(0.0, 0.0, 0.0);

    let rot = DQuat::from_rotation_arc(DVec3::NEG_Z, (look_at - look_from).normalize());
    let (y, x, _) = rot.to_euler(glam::EulerRot::YXZ);
    let rot = DQuat::from_euler(glam::EulerRot::YXZ, y, x, 0.0);

    let camera_transform = DAffine3::from_rotation_translation(rot, look_from);
    let mut camera = Camera::new(camera_transform, 90.0, 1920, 1080);
    camera.initialize();

    let mut objects = HittableCollection::new();

    let lambertian_material_red = Rc::new(LambertianMaterial::new(DVec3::new(0.9, 0.2, 0.1)));
    let lambertian_material_green = Rc::new(LambertianMaterial::new(DVec3::new(0.2, 0.8, 0.0)));
    let lambertian_material_blue = Rc::new(LambertianMaterial::new(DVec3::new(0.1, 0.2, 0.9)));
    let metal_material = Rc::new(MetalMaterial::new(DVec3::new(0.8, 0.8, 0.8), 0.1));

    let dielectric_material = Rc::new(DielectricMaterial::new(1.5));

    objects.add(Box::new(Sphere::new(
        DVec3::new(0.0, 0.0, -1.0),
        0.5,
        lambertian_material_red.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        DVec3::new(1.0, 2.0, -2.0),
        0.5,
        metal_material.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        DVec3::new(0.3, -0.75, -1.0),
        1.0,
        lambertian_material_blue.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        DVec3::new(1.5, 0.0, -0.6),
        0.5,
        dielectric_material.clone(),
    )));
    objects.add(Box::new(Sphere::new(
        DVec3::new(0.0, -100.5, -1.0),
        100.0,
        lambertian_material_green.clone(),
    )));

    file.write(
        format!(
            "P3\n{} {}\n256\n",
            camera.viewport.image_width, camera.viewport.image_height
        )
        .as_bytes(),
    )
    .unwrap();

    let start = std::time::Instant::now();
    let pixels = camera.render(&objects);
    println!("Elapsed: {:?}", start.elapsed());
    pixels.iter().for_each(|pixel| {
        file.write(
            format!(
                "{} {} {}\n",
                (pixel.x * 255.0) as u8,
                (pixel.y * 255.0) as u8,
                (pixel.z * 255.0) as u8
            )
            .as_bytes(),
        )
        .unwrap();
    });
}
