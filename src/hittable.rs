use crate::material::Material;
use crate::vector3::{Point3, Vec3, dot};
use crate::ray::Ray;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        let front_face = dot(ray.direction(), &outward_normal) < 0.0;
        self.normal = if front_face {outward_normal} else {-outward_normal};
    }

    pub fn update_as(&mut self, new_record: &HitRecord) {
        self.p = new_record.p.clone();
        self.normal = new_record.normal.clone();
        self.t = new_record.t;
        self.front_face = new_record.front_face;
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool;
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self{
        Self {
            objects: vec![]
        }
    }
    
    pub fn add(&mut self, hittable_object: Box<dyn Hittable>) {
        self.objects.push(hittable_object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut temp_rec = HitRecord::default();
        
        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                hit_rec.update_as(&temp_rec);
            }
        }
        hit_anything
    }
}