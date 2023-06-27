use crate::{ray::Ray, hittable::HitRecord, vector3::Color};

pub trait Material: {
    fn scatter(&self, ray_in: Ray, hit_record: HitRecord, attenuation: Color, scattered: Ray) -> bool;
}