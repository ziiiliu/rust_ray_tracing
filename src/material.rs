use crate::{ray::Ray, hittable::HitRecord, vector3::{Color, reflect, unit_vector, dot, refract}, util::{random_unit_vector, random_f64}};
use dyn_clone::DynClone;

dyn_clone::clone_trait_object!(Material);
pub trait Material: DynClone  {
    fn scatter(&self, ray_in: Ray, hit_record: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {albedo}
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: Ray, hit_record: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction = hit_record.normal.clone() + random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }
        scattered.update_as(Ray::new(&hit_record.p, &scatter_direction));
        attenuation.update_as(self.albedo.clone());
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 {fuzz} else {1.0};
        Self {albedo, fuzz}
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: Ray, hit_record: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let reflected = reflect(&unit_vector(ray_in.direction()), &hit_record.normal);
        scattered.update_as(Ray::new(&hit_record.p, &(reflected + random_unit_vector() * self.fuzz)));
        attenuation.update_as(self.albedo.clone());
        dot(scattered.direction(), &hit_record.normal) > 0.0
    }
}

#[derive(Clone)]
pub struct Dielectric {
    ir: f64, // refraction index
}

impl Dielectric {
    pub fn new (ir: f64) -> Self {
        Self {ir}
    }

    fn reflectance(&self, cos_theta: f64, refraction_ratio: f64) -> f64 {
        // Schlick's approximation for reflectance.
        let r0 = (1.0-refraction_ratio) / (1.0+refraction_ratio);
        return r0.powi(2) + (1.0 - r0.powi(2)) * (1.0 - cos_theta).powi(5);
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: Ray, hit_record: HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        let eta_ratio = if hit_record.front_face {1.0/self.ir} else {self.ir};
        attenuation.update_as(Color::new(1.0, 1.0, 1.0));

        let unit_direction = unit_vector(ray_in.direction());
        let cos_theta = f64::min(dot(&-unit_direction.clone(), &hit_record.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = eta_ratio * sin_theta > 1.0; // total reflection

        let direction = match cannot_refract || self.reflectance(cos_theta, eta_ratio) > rand::random::<f64>() {
            true => reflect(&unit_direction, &hit_record.normal),
            false => refract(&unit_direction, &hit_record.normal, eta_ratio),
        };
        scattered.update_as(Ray::new(&hit_record.p, &direction));
        true
    }
}