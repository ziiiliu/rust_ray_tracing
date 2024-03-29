use crate::vector3::{Point3, Vec3};

#[derive(Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Default::default(),
            direction: Default::default(),
        }
    }
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin().clone() + self.direction().clone() * t
    }

    pub fn update_as(&mut self, ray: Ray) {
        self.origin = ray.origin;
        self.direction = ray.direction;
    }
}
