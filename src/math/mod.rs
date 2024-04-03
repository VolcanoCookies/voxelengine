use glam::DVec3;

#[derive(Debug, Clone, Copy)]
pub struct Interval {
    pub start: f64,
    pub end: f64,
}

impl Interval {
    pub const EMPTY: Interval = Interval {
        start: 0.0,
        end: 0.0,
    };

    pub const UNIVERSE: Interval = Interval {
        start: f64::NEG_INFINITY,
        end: f64::INFINITY,
    };

    pub fn new(start: f64, end: f64) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, value: f64) -> bool {
        self.start <= value && value <= self.end
    }

    pub fn surrounds(&self, value: f64) -> bool {
        self.start < value && value < self.end
    }
}

pub fn reflect(v: DVec3, n: DVec3) -> DVec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(v: DVec3, n: DVec3, etai_over_etat: f64) -> DVec3 {
    let cos_theta = (-v).dot(n).min(1.0);
    let r_out_perp = etai_over_etat * (v + cos_theta * n);
    let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * n;

    r_out_perp + r_out_parallel
}
