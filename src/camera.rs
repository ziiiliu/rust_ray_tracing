use crate::{
    hittable::{HitRecord, HittableList},
    ray::Ray,
    util::{degrees_to_radians, Interval, INF},
    vector3::{cross, unit_vector, write_color, Color, Point3, Vec3},
};
use rand::Rng;

const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: i32 = 225;
const IMAGE_WIDTH: i32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 50;

pub struct Camera {
    origin: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Point3,
    vfov: f64, // vertical view angle (field of view)
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let vfov = 90.0;
        let lookfrom = Point3::new(0.0, 0.0, -1.0);
        let lookat = Point3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let origin = lookfrom.clone();
        let focal_length = (lookfrom.clone() - lookat.clone()).length();

        let w = unit_vector(&(lookfrom - lookat));
        let u = unit_vector(&cross(&vup, &w));
        let v = cross(&w, &u);

        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = aspect_ratio * viewport_height;

        let viewport_u = u * viewport_width; // Vector across viewport horizontal edge
        let viewport_v = -v * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u.clone() / IMAGE_WIDTH as f64;
        let pixel_delta_v = viewport_v.clone() / IMAGE_HEIGHT as f64;

        let viewport_upper_left = origin.clone()
            - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc =
            viewport_upper_left + (pixel_delta_u.clone() + pixel_delta_v.clone()) * 0.5;

        Self {
            origin: origin.clone(),
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            vfov,
        }
    }
}

impl Camera {
    pub fn get_ray(&self, i: f64, j: f64) -> Ray {
        let pixel_center = self.pixel00_loc.clone()
            + (self.pixel_delta_u.clone() * i)
            + (self.pixel_delta_v.clone() * j);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.origin.clone();
        let ray_direction = pixel_sample - ray_origin.clone();
        Ray::new(&ray_origin, &ray_direction)
    }

    pub fn ray_color(&self, ray: &Ray, world: &HittableList, depth: i32) -> Color {
        let hit_record = &mut HitRecord::default();
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        if world.hit(ray, Interval::new(0.001, INF), hit_record) {
            let scattered = &mut Ray::default();
            let attenuation = &mut Color::default();
            if hit_record.material.clone().unwrap().scatter(
                ray.clone(),
                hit_record.clone(),
                attenuation,
                scattered,
            ) {
                return self.ray_color(&scattered, world, depth - 1) * attenuation;
            }
            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = unit_vector(ray.direction());
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
    }

    pub fn render(&self, world: &HittableList) {
        println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");

        for j in (0..IMAGE_HEIGHT).rev() {
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _s in 0..SAMPLES_PER_PIXEL {
                    let ray = self.get_ray(i as f64, j as f64);
                    pixel_color = pixel_color + self.ray_color(&ray, &world, MAX_DEPTH);
                }
                write_color(pixel_color, SAMPLES_PER_PIXEL);
            }
        }
    }
    pub fn pixel_sample_square(&self) -> Vec3 {
        let mut rng = rand::thread_rng();
        // Returns a random point in the square surrounding a pixel at the origin.
        let px = -0.5 + rng.gen::<f64>();
        let py = -0.5 + rng.gen::<f64>();
        return (self.pixel_delta_u.clone() * px) + (self.pixel_delta_v.clone() * py);
    }
}
