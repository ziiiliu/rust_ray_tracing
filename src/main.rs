pub mod vector3;
pub mod ray;
pub mod hittable;
pub mod sphere;
pub mod util;
pub mod camera;
pub mod material;

use hittable::HitRecord;
use rand::Rng;
use util::{INF, random_in_unit_sphere, random_unit_vector};
use vector3::{Color, write_color, unit_vector};
use ray::Ray;
use camera::Camera;

use crate::{vector3::{Point3}, hittable::HittableList, sphere::Sphere};

fn ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    let hit_record = &mut HitRecord::default();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0)
    }
    if world.hit(ray, 0.001, INF, hit_record) {
        let target = hit_record.p.clone() + hit_record.normal.clone() + random_unit_vector();
        return ray_color(&Ray::new(&hit_record.p, &(target-hit_record.p.clone())), world, depth-1) * 0.5
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
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: i32 = 50;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let cam = Camera::default();

    // Render
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

    let mut rng  = rand::thread_rng();

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH{
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH as f64 - 1.0);
                let v = (j as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT as f64 - 1.0);
                let ray = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world, MAX_DEPTH);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
}
