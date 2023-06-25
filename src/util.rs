use rand::{Rng};

pub const INF: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    let mut rng  = rand::thread_rng();
    min + rng.gen::<f64>() * (max-min)
}