use std::fs::File;
use std::io::{BufWriter, Write};

use crate::vec3::Vec3;
use crate::ray::Ray;

pub mod vec3;
pub mod ray;

fn write_color(f: &mut BufWriter<File>, color: Vec3) {
    let r = (255.99 * color.x()) as i32;
    let g = (255.99 * color.y()) as i32;
    let b = (255.99 * color.z()) as i32;

    write!(f, "{r} {g} {b}\n").expect("Can't write to file");
}

fn hit_sphere(center: Vec3, radius: f64, ray: &Ray) -> bool {
    let offset = ray.origin() - center;

    let a = Vec3::dot(ray.dir(), ray.dir());
    let b = 2.0 * Vec3::dot(ray.dir(), offset);
    let c = Vec3::dot(offset, offset) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}

fn ray_color(ray: &Ray) -> Vec3 {
    if hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Vec3::new(1.0, 0.0, 0.0)
    }

    let unit_dir = ray.dir().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);

    (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 600;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let image = File::create("image.ppm").expect("Unable to create file!");
    let mut image = BufWriter::new(image);

    write!(image, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").expect("Unable to write");

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let color = ray_color(&ray);
            write_color(&mut image, color);
        }
    }
    println!("Done");
}
