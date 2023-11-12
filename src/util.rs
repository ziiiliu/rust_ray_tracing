use crate::vector3::{unit_vector, Vec3};

pub const INF: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;
// pub const EMPTY: Interval = Interval::new(INF, -INF);
// pub const UNIVERSE: Interval = Interval::new(-INF, INF);

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    min + rand::random::<f64>() * (max - min)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let random_p = Vec3::random(-1.0, 1.0);
        if random_p.length_squared() >= 1.0 {
            continue;
        }
        return random_p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(_min: f64, _max: f64) -> Self {
        Self {
            min: _min,
            max: _max,
        }
    }

    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: -INF,
            max: INF,
        }
    }
}
