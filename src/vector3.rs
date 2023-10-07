use std::{ops, fmt::{Display, Formatter}};

use crate::util::{clamp, random_f64};

#[derive(Clone, Default)]
pub struct Vec3 {
    e: Vec<f64>,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn default() -> Self {
        Self { e: vec![0.0, 0.0, 0.0] }
    }

    pub fn new(e1: f64, e2: f64, e3: f64) -> Self {
        Self { e: vec![e1, e2, e3] }
    }

    pub fn random(min: f64, max: f64) -> Self {
        Self::new(random_f64(min, max), random_f64(min, max), random_f64(min, max))
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0].powf(2.0) + self.e[1].powf(2.0) + self.e[2].powf(2.0) 
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z() < s
    }

    pub fn update_as(&mut self, c: Vec3) {
        self.e = c.e
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let x = self.x();
        let y = self.y();
        let z = self.z();
        write!(f, "({x}, {y}, {z})")
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self {
            e: vec![self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z()]
        }
    } 
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Self {
            e: vec![self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()]
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: &'b Vec3) -> Self::Output {
        Vec3 {
            e: vec![self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z()]
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            e: vec![self.x() * rhs, self.y() * rhs, self.z() * rhs]
        }
    }
}

impl ops::Mul<&mut Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: &mut Vec3) -> Self::Output {
        Self {
            e: vec![self.x() * v.x(), self.y() * v.y(), self.z() * v.z()]
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0/rhs)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self{
            e: vec![-self.x(), -self.y(), -self.z()]
        }
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() * v.z()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
    Vec3::new(u.e[1] * v.e[2] - u.e[2] * v.e[1], 
              u.e[2] * v.e[0] - u.e[0] * v.e[2],
              u.e[0] * v.e[1] - u.e[1] * v.e[0])
}

pub fn unit_vector(u: &Vec3) -> Vec3 {
    u.clone() / u.length()
}

pub fn write_color(pixel_color: Color, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f64;

    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b *scale).sqrt();

    let r = (255.999 * clamp(r, 0.0, 0.999)) as i16;
    let g = (255.999 * clamp(g, 0.0, 0.999)) as i16;
    let b = (255.999 * clamp(b,0.0, 0.999)) as i16;
    println!("{r} {g} {b}")
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    v.clone() - n.clone() * dot(v, n) * 2.0
}