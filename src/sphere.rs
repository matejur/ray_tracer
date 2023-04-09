use crate::{hittable::*, material::Material, ray::Ray, vec3::Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: impl Material + 'static) -> Self {
        Self {
            center,
            radius,
            material: Box::new(material),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let offset = ray.origin() - self.center;

        let a = ray.dir().length_squared();
        let half_b = Vec3::dot(ray.dir(), offset);
        let c = offset.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return HitRecord::Miss;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;

            if root < t_min || t_max < root {
                return HitRecord::Miss;
            }
        }

        let point = ray.at(root);
        let normal = (point - self.center) / self.radius;
        let front_face = Vec3::dot(ray.dir(), normal) < 0.0;
        let material = &self.material;

        let normal = if front_face { normal } else { -normal };

        HitRecord::Hit {
            point,
            normal,
            t: root,
            front_face,
            material,
        }
    }
}
