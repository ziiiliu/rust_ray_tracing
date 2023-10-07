use crate::{ray::Ray, hittable::HitRecord, vector3::{Color, reflect, unit_vector, dot}, util::random_unit_vector};
use dyn_clone::DynClone;

dyn_clone::clone_trait_object!(Material);
pub trait Material: DynClone  {
    fn scatter(&self, ray_in: Ray, hit_record: HitRecord, attenuation: Color, scattered: Ray) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: Ray, hit_record: HitRecord, mut attenuation: Color, mut scattered: Ray) -> bool {
        let mut scatter_direction = hit_record.normal + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        scattered = Ray::new(&hit_record.p, &scatter_direction);
        attenuation = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit_record: HitRecord, mut attenuation: Color, mut scattered: Ray) -> bool {
        let reflected = reflect(&unit_vector(ray_in.direction()), &hit_record.normal);
        scattered = Ray::new(&hit_record.p, &reflected);
        attenuation = self.albedo;
        dot(scattered.direction(), &hit_record.normal) > 0.0
    }
}