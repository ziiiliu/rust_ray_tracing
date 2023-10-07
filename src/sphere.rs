use crate::{vector3::{Point3, dot}, hittable::{Hittable, HitRecord}, material::Material, util::Interval};

pub struct Sphere {
    centre: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(centre: Point3, r: f64, material: Box<dyn Material>) -> Self {
        Self { centre, radius: r, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &crate::ray::Ray, ray_t: Interval, mut hit_rec: &mut HitRecord) -> bool {
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
        if !ray_t.surrounds(root) {
            root = (-half_b +sqrt_d) / a;
            if !ray_t.surrounds(root) {
                return false
            }
        }

        hit_rec.t = root;
        hit_rec.p = r.at(hit_rec.t);
        let outward_normal = (hit_rec.p.clone() - self.centre.clone()) / self.radius;
        hit_rec.set_face_normal(r, outward_normal);
        hit_rec.material = Some(self.material);

        true
    }
}