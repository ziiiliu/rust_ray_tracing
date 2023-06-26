use crate::ray::Ray;

pub trait Material {
    fn scatter(&self, ray_in: Ray, ) -> bool {
        
    };
}