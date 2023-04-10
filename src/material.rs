use crate::{ray::Ray, hittable::HitRecord, vec3::Vec3, utility::random};

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
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
    }
}


impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> MaterialData {
        if let HitRecord::Hit { point, t, normal, front_face, material } = record {
            let reflected = Vec3::reflect(ray.dir().unit_vector(), *normal);
            let scattered = Ray::new(*point,  reflected + self.fuzz * Vec3::random_in_unit_sphere());

            if Vec3::dot(scattered.dir(), *normal) > 0.0 {
                return MaterialData::Scatter { attenuation: self.albedo, scattered};
            } else {
                return MaterialData::Not
            }
        }

        MaterialData::Not
    }
}

pub struct Dielectric {
    refraction_index: f64
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}


impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> MaterialData {
        if let HitRecord::Hit { point, t, normal, front_face, material } = record {
            let attenuation = Vec3::new(1.0, 1.0, 1.0);
            let refraction_ratio = if *front_face { 1.0 / self.refraction_index } else { self.refraction_index };

            let unit_dir = ray.dir().unit_vector();
            let cos_theta = Vec3::dot(-unit_dir, *normal).min(1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

            let cannot_refract = (refraction_ratio * sin_theta) > 1.0;

            let dir = if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > random() {
                Vec3::reflect(unit_dir, *normal)
            } else {
                Vec3::refract(unit_dir, *normal, refraction_ratio)
            };

            let scattered = Ray::new(*point, dir);

            return MaterialData::Scatter { attenuation, scattered };
        }

        MaterialData::Not
    }
}