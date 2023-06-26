use crate::vector3::{Vec3, unit_vector};

pub const INF: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_f64(min: f64, max: f64) -> f64 {
    min + rand::random::<f64>() * (max-min)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let random_p = Vec3::random(-1.0, 1.0);
        if random_p.length_squared() >= 1.0 {
            continue;
        } 
        return random_p
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(&random_in_unit_sphere())
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    }
    else if x > max {
        max
    }
    else {
        x
    }
}