use crate::{vector3::{Point3, dot}, hittable::{Hittable, HitRecord}};

pub struct Sphere {
    centre: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Point3, r: f64) -> Self {
        Self { centre, radius: r }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, mut hit_rec: &mut HitRecord) -> bool {
        let oc = r.origin() - &self.centre;
        let a = r.direction().length_squared();
        let half_b = dot(&oc, r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false
        }
        let sqrt_d = discriminant.sqrt();

        // Find the appropriate t value that's within the acceptable range
        let mut root = (-half_b -sqrt_d) / a;
        if root < t_min || root > t_max {
            root = (-half_b +sqrt_d) / a;
            if root < t_min || root > t_max {
                return false
            }
        }

        hit_rec.t = root;
        hit_rec.p = r.at(hit_rec.t);
        let outward_normal = (hit_rec.p.clone() - self.centre.clone()) / self.radius;
        hit_rec.set_face_normal(r, outward_normal);

        true
    }
}