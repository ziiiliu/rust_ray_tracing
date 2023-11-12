pub mod camera;
pub mod hittable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vector3;

use std::f64::consts::PI;

use camera::Camera;
use vector3::Color;

use crate::{
    hittable::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
    vector3::Point3,
};

fn main() {
    // World
    let mut world = HittableList::new();

    let r = (PI / 4.0).cos();

    let material_ground = Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_centre = Box::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left = Box::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    // let material_centre = Box::new(Dielectric::new(1.5));
    let material_left = Box::new(Dielectric::new(1.5));
    let material_right = Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    // world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_centre)));
    // world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone())));
    // world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.4, material_left)));
    // world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    world.add(Box::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    // Camera
    let cam = Camera::default();

    cam.render(&world);
}
