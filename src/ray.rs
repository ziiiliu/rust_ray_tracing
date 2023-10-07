use crate::vector3::{Point3, Vec3};

#[derive(Clone)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Ray { origin: origin.clone(), direction: direction.clone() }
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
}