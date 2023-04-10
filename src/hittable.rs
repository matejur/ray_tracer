use crate::{material::Material, ray::Ray, vec3::Vec3};

pub enum HitRecord<'a> {
    Hit {
        point: Vec3,
        t: f64,
        normal: Vec3,
        front_face: bool,
        material: &'a dyn Material,
    },
    Miss,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord;
}

pub struct HittableList<'a> {
    list: Vec<&'a dyn Hittable>,
}

impl<'a> HittableList<'a> {
    pub fn new() -> Self {
        HittableList { list: Vec::new() }
    }

    pub fn add(&mut self, obj: &'a impl Hittable) {
        self.list.push(obj);
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

impl<'a> Hittable for HittableList<'a> {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let mut closest_so_far = t_max;
        let mut rec = HitRecord::Miss;

        for obj in &self.list {
            let record = obj.hit(ray, t_min, closest_so_far);

            match record {
                HitRecord::Hit {
                    point: _,
                    t,
                    normal: _,
                    front_face: _,
                    material: _,
                } => {
                    rec = record;
                    closest_so_far = t;
                }
                HitRecord::Miss => (),
            }
        }

        rec
    }
}
