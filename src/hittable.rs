use crate::ray::*;
use crate::vec3::{Point3,Vec3};
use crate::interval::Interval;
use std::sync::Arc;
use crate::material::{EmptyMaterial, Material};

#[derive(Clone)]
pub struct HitRecord{
    pub p : Point3,
    pub normal : Vec3,
    pub mat: Arc<dyn Material>,
    pub t : f64,
    pub front_face : bool,
}

impl HitRecord {
    pub fn new() -> HitRecord{
         HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat : Arc::new(EmptyMaterial),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot_product(ray.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal.scalar_mul(-1.0)
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Hittable : Send + Sync{
    fn hit(&self,r:&Ray, ray_t : Interval ,rec:&mut HitRecord) -> bool;
}