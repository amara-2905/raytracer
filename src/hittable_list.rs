use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use std::sync::Arc;
use std::vec::Vec;
use crate::interval::Interval;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn _new_with(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    pub fn _clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval , rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default(); 
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if object.hit(r, Interval{ min: ray_t.min , max: closest_so_far}, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone(); 
            }
        }

        hit_anything
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}