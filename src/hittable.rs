use crate::{vector::{Point3, Vec3}, ray::Ray, world::material::Material};


pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

pub fn create_new_hittable_list() -> HittableList {
    HittableList { objects: Vec::new() }
}

impl HittableList {
    pub fn add_new_hittable<T: Hittable + 'static>(&mut self, x: T) {
        self.objects.push(Box::new(x))
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record = None;
        let mut closest_so_far = t_max;
        for object in &self.objects {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                hit_record = Some(hit);
            }
        }
        hit_record
    }
}
