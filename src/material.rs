use crate::{hittable::HitRecord, random_double, ray::Ray, vec3::Vec3};

pub struct EmptyMaterial;

impl Material for EmptyMaterial {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
}
pub trait Material: Send + Sync {
    fn scatter(&self,r_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3,scattered: &mut Ray) -> bool {
        false
    }
}

pub struct Lambertian{
    pub albedo: Vec3,
}

pub struct Metal{
    pub albedo: Vec3,
    pub fuzz: f64
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self { 
            albedo: albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Lambertian{
    fn scatter(&self,r_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = Vec3::add(rec.normal,Vec3::random_unit_vector());

        if scatter_direction.near_zero(){
            scatter_direction = rec.normal
        }

        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        return true
    }
}

impl Material for Metal{
    fn scatter(&self,r_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut reflected = Vec3::reflect(r_in.direction(), rec.normal);
        reflected = Vec3::add(reflected, Vec3::random_unit_vector().scalar_mul(self.fuzz));
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        return Vec3::dot_product(scattered.direction(), rec.normal) > 0.0
    }
}

pub struct Dielectric{
    pub refraction_index : f64,
}

impl Dielectric{
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { 
            refraction_index, 
        }
    }

    pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index)/(1.0 + refraction_index);
        r0 = r0 * r0;
        return r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
    }
}

impl Material for Dielectric{
    fn scatter(&self,r_in: &Ray,rec: &HitRecord,attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::new(1.0,1.0,1.0);
        let ri = if rec.front_face {1.0/self.refraction_index} else { self.refraction_index };
        let unit_direction = (r_in.direction()).unit_vector();
        let cos_theta = Vec3::dot_product(unit_direction.scalar_mul(-1.0), rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract  = ri * sin_theta > 1.0;
        let mut direction = Vec3::new(0.0, 0.0, 0.0);
        if cannot_refract || (Dielectric::reflectance(cos_theta, ri) > random_double()){
            direction = Vec3::reflect(unit_direction, rec.normal);
        }else{
            direction = Vec3::refract(unit_direction, rec.normal, ri);
        }
        *scattered = Ray::new(rec.p, direction);
        return true
    }
}