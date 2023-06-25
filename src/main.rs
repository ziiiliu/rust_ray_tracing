pub mod vector3;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod util;
pub mod camera;

use hittable::HitRecord;
use util::INF;
use vector3::{Color, write_color, unit_vector, dot};
use ray::Ray;

use crate::{vector3::{Point3, Vec3}, hittable::HittableList, sphere::Sphere};

fn ray_color(ray: &Ray, world: &HittableList) -> Color {
    let hit_record = &mut HitRecord::default();
    if world.hit(ray, 0.0, INF, hit_record) {
        return (Color::new(1.0, 1.0, 1.0) + hit_record.normal.clone()) * 0.5
    }

    let unit_direction = unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0-t) + Color::new(0.5, 0.7, 1.0) * t 
}

// fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
//     let oc = ray.origin() - center;
//     let a = dot(ray.direction(), ray.direction());
//     let b = 2.0 * dot(&oc, ray.direction());
//     let c = dot(&oc, &oc) - radius * radius;
//     let discriminant = b*b -a*c*4.0;
//     if discriminant < 0.0 {
//         -1.0
//     }
//     else {
//         (-b - discriminant.sqrt()) / (2.0 * a)
//     }
// }

fn main(){
    
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: i32 = 225;
    const IMAGE_WIDTH: i32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as i32;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let view_port_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0,0.0,0.0);
    let horizontal = Vec3::new(view_port_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin.clone() - horizontal.clone()/2.0 -vertical.clone()/2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH{
            let u = (i as f64) / (IMAGE_WIDTH as f64 - 1.0);
            let v = (j as f64) / (IMAGE_HEIGHT as f64 - 1.0);
            let direction = lower_left_corner.clone() + horizontal.clone() * u + vertical.clone() * v - origin.clone();

            let ray = Ray::new(&origin, &direction);
            let pixel_color = ray_color(&ray, &world);
            write_color(pixel_color);
        }
    }
}
