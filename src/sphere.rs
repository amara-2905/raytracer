use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::{Material};
use crate::vec3::*;
use crate::ray::Ray;

pub struct Sphere{
    pub center : Point3,
    pub radius : f64,
    pub mat: Arc<dyn Material>,
}

impl Sphere{
    pub fn new(center : Point3, radius : f64, mat: Arc<dyn Material>) -> Sphere{
        Sphere{
            center,
            radius : radius.max(0.0),
            mat,
        }
    }
}

impl Hittable for Sphere{
    fn hit (&self,r :&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool{
        let oc = Vec3::sub(self.center, r.origin());
        let a = r.direction().length_squared();
        let h = Vec3::dot_product(r.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false
        } 

        let sqrtd = discriminant.sqrt();

        let mut root = ( h - sqrtd )/a;
        if !Interval::surrounds(&ray_t,root) {
            root = (h + sqrtd) / a;
            if  !Interval::surrounds(&ray_t,root){
                return false
            }
        }

        rec.t = root;
        rec.p = r.ray_at(rec.t);
        let outer_normal = (Vec3::sub(rec.p, self.center)).scalar_div(self.radius) ;
        rec.set_face_normal(r, outer_normal);
        rec.mat = self.mat.clone();

        true
    }
}