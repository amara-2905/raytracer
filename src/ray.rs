use crate::vec3::{Point3, Vec3};

pub struct Ray{
    pub origin : Point3,
    pub direction : Vec3,
}

impl Ray{
    pub fn new(a : Point3 , b : Vec3) -> Ray {
        Ray { origin: a, direction: b }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn ray_at(&self, t: f64) -> Vec3 {
        Vec3::add(self.origin, self.direction.scalar_mul(t))
    }
}