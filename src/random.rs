pub fn random_vec() -> glam::DVec3 {
    let x = rand::random::<f64>();
    let y = rand::random::<f64>();
    let z = rand::random::<f64>();

    glam::DVec3::new(x, y, z)
}

pub fn random_vec_in_unit_sphere() -> glam::DVec3 {
    loop {
        let vec = random_vec() * 2.0 - glam::DVec3::ONE;
        if vec.length_squared() < 1.0 {
            return vec;
        }
    }
}

pub fn random_vec_unit() -> glam::DVec3 {
    random_vec_in_unit_sphere().normalize()
}

pub fn random_vec_on_hemi(normal: glam::DVec3) -> glam::DVec3 {
    let vec = random_vec_in_unit_sphere();
    if vec.dot(normal) > 0.0 {
        vec
    } else {
        -vec
    }
}
