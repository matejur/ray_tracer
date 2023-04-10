use std::f64::INFINITY;
use std::fs::File;
use std::io::{BufWriter, Write};

use camera::Camera;
use hittable::Hittable;
use hittable::HittableList;
use material::Lambertian;
use ray::Ray;
use sphere::Sphere;
use utility::clamp;
use utility::random;
use vec3::Vec3;

use crate::material::{Dielectric, Metal};

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod utility;
mod vec3;

fn write_color(f: &mut BufWriter<File>, color: Vec3, samples: i32) {
    let r = color.x() / samples as f64;
    let g = color.y() / samples as f64;
    let b = color.z() / samples as f64;

    let r = (256.0 * clamp(r.sqrt(), 0.0, 0.999)) as i32;
    let g = (256.0 * clamp(g.sqrt(), 0.0, 0.999)) as i32;
    let b = (256.0 * clamp(b.sqrt(), 0.0, 0.999)) as i32;

    write!(f, "{r} {g} {b}\n").expect("Can't write to file");
}

fn ray_color(ray: &Ray, world: &impl Hittable, depth: i32) -> Vec3 {
    let record = world.hit(ray, 0.001, INFINITY);

    if depth < 0 {
        return Vec3::new(0.0, 0.0, 0.0);
    }

    match record {
        hittable::HitRecord::Hit {
            point,
            t: _,
            normal,
            front_face: _,
            material,
        } => {
            let scattered = material.scatter(ray, &record);

            match scattered {
                material::MaterialData::Scatter {
                    attenuation,
                    scattered,
                } => {
                    return attenuation * ray_color(&scattered, world, depth - 1);
                }
                material::MaterialData::Not => return Vec3::new(0.0, 0.0, 0.0),
            }
        }
        hittable::HitRecord::Miss => {
            let unit_dir = ray.dir().unit_vector();
            let t = 0.5 * (unit_dir.y() + 1.0);

            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 600;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 25;
    const MAX_DEPTH: i32 = 20;

    let mut world = HittableList::new();
    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_right = Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0);


    let sphere = Sphere::new(
        Vec3::new(0.0, -100.5, 0.0),
        100.0,
        &material_ground,
    );
    world.add(&sphere);

    let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, &material_center);
    world.add(&sphere);

    let sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, &material_left);
    world.add(&sphere);

    let sphere = Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, &material_left);
    world.add(&sphere);

    let sphere = Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, &material_right);
    world.add(&sphere);

    let camera = Camera::new();

    let image = File::create("image.ppm").expect("Unable to create file!");
    let mut image = BufWriter::new(image);

    write!(image, "P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255\n").expect("Unable to write");

    for j in (0..IMAGE_HEIGHT).rev() {
        println!("Scanlines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut color = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, &world, MAX_DEPTH);
            }

            write_color(&mut image, color, SAMPLES_PER_PIXEL);
        }
    }
    println!("Done");
}
