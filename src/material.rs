use crate::{ray::Ray, hittable::HitRecord, vec3::Vec3};

pub enum MaterialData {
    Scatter {
        attenuation: Vec3,
        scattered: Ray
    },
    Not
}
pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> MaterialData;
}

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> MaterialData {
        if let HitRecord::Hit { point, t, normal, front_face, material } = record {
            let mut scatter_dir = *normal + Vec3::random_unit_vector();

            if scatter_dir.near_zero() {
                scatter_dir = *normal;
            }

            return MaterialData::Scatter { attenuation: self.albedo, scattered: Ray::new(*point, scatter_dir) };
        }

        MaterialData::Not
    }
}

pub struct Metal {
    albedo: Vec3
}

impl Metal {
    pub fn new(albedo: Vec3) -> Self {
        Metal { albedo }
    }
}


impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> MaterialData {
        if let HitRecord::Hit { point, t, normal, front_face, material } = record {
            let reflected = Vec3::reflect(ray.dir().unit_vector(), *normal);
            let scattered = Ray::new(*point, reflected);

            if Vec3::dot(scattered.dir(), *normal) > 0.0 {
                return MaterialData::Scatter { attenuation: self.albedo, scattered};
            } else {
                return MaterialData::Not
            }
        }

        MaterialData::Not
    }
} 