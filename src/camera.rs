use std::{f64::INFINITY};
use rayon::prelude::*;
use crate::{color::write_color, degrees_to_radians, hittable::{HitRecord, Hittable}, interval::Interval, random_double, ray::Ray, vec3::Vec3};

pub struct Camera{
    pub aspect_ratio: f64,
    pub image_width: i64,
    pub samples_per_pixel: i64,
    pub max_depth: i64,
    pub vfov: i64,
    pub lookform: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    image_height: i64,
    center: Vec3,
    pixel100_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    pixel_samples_scale: f64,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera{
    pub fn new(aspect_ratio: f64, image_width: i64, samples_per_pixel :i64, max_depth: i64, vfov: i64, lookform: Vec3, lookat: Vec3, vup: Vec3,defocus_angle: f64, focus_dist:f64) -> Self {
        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            vfov,
            lookform,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            image_height: 0,
            center: Vec3::new(0.0, 0.0, 0.0),
            pixel100_loc: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            u: Vec3::new(0.0, 0.0, 0.0),
            v: Vec3::new(0.0, 0.0, 0.0),
            w: Vec3::new(0.0, 0.0, 0.0),
            pixel_samples_scale: 0.0,
            defocus_disk_u: Vec3::new(0.0, 0.0, 0.0),
            defocus_disk_v: Vec3::new(0.0, 0.0, 0.0),
        }
    }

    fn initialize(&mut self){
        self.image_height = (self.image_width as f64 /self.aspect_ratio) as i64;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
        
        self.center = self.lookform;

        let theta = degrees_to_radians(self.vfov as f64);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width : f64 = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = Vec3::sub(self.lookform, self.lookat).unit_vector();
        self.u = Vec3::cross_product(self.vup, self.w).unit_vector();
        self.v = Vec3::cross_product(self.w, self.u);

        let viewport_u = self.u.scalar_mul(viewport_width);
        let viewport_v = self.v.scalar_mul(-1.0 * viewport_height);

        self.pixel_delta_u = viewport_u.scalar_div(self.image_width as f64);
        self.pixel_delta_v = viewport_v.scalar_div(self.image_height as f64);
        
        let t = viewport_u.scalar_div(2.0);
        let o = viewport_v.scalar_div(2.0);
        let p = Vec3::add(t,o);
        let j = self.w.scalar_mul(self.focus_dist);
        let q = Vec3::add(p,j);
        let viewport_upper_left = Vec3::sub(self.center,q);

        let sum1 = Vec3::add(self.pixel_delta_u,self.pixel_delta_v);
        let q = sum1.scalar_mul(0.5);
        self.pixel100_loc = Vec3::add(q, viewport_upper_left);
        
        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle/2.0)).tan();
        self.defocus_disk_u = self.u.scalar_mul(defocus_radius);
        self.defocus_disk_v = self.v.scalar_mul(defocus_radius);

    }

    fn ray_color<T: Hittable>(r: &Ray,depth: i64, world: &T) -> Vec3{
        if depth <= 0 {
            return Vec3::new(0.0,0.0,0.0);
        }
        let mut rec = HitRecord::new();
        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec){
            let mut scattered = Ray::new(Vec3::new(0.0,0.0,0.0), Vec3::new(0.0,0.0,0.0));
            let mut attenuation = Vec3::new(0.0,0.0,0.0);

            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered){
                return Vec3::mul(attenuation, Camera::ray_color(&scattered, depth -1 , world))
            }
            return Vec3::new(0.0, 0.0, 0.0)
        }

        let unit_direction = (r.direction()).unit_vector();
        let a = (unit_direction.y() + 1.0 ) * (0.5);
        return Vec3::add(Vec3::new(1.0,1.0,1.0).scalar_mul(1.0-a), Vec3::new(0.5,0.7,1.0).scalar_mul(a))

    }
    
    fn sample_square() -> Vec3{
        Vec3::new(random_double()-0.5,random_double()-0.5,0.0)
    }

    fn defocus_disk_sample(&self) -> Vec3{
        let p = Vec3::random_in_unit_disk();
        return Vec3::add(self.center, Vec3::add(self.defocus_disk_u.scalar_mul(p.x()),self.defocus_disk_v.scalar_mul(p.y())));
    }

    fn get_ray(&self,i: i64, j: i64) -> Ray {
        let offset = Camera::sample_square();
        let g = self.pixel_delta_u.scalar_mul(i as f64 + offset.x());
        let h = self.pixel_delta_v.scalar_mul(j as f64 + offset.y());
        let k = Vec3::add(g,h);
        let pixel_sample = Vec3::add(self.pixel100_loc,k);
        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else{ self.defocus_disk_sample() };
        let ray_direction = Vec3::sub(pixel_sample, ray_origin);
        return Ray::new(ray_origin, ray_direction)
    }

    pub fn render<T: Hittable + Sync>(&mut self, world: &T){
        self.initialize();

        println!("P3\n{} {} \n255\n",self.image_width,self.image_height);
        let image_pixels: Vec<Vec<Vec3>> = (0..self.image_height)
            .into_par_iter()
            .map(|j| {
                (0..self.image_width)
                    .map(|i| {
                        let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                        for _ in 0..self.samples_per_pixel {
                            let r = self.get_ray(i, j);
                            pixel_color = Vec3::add(pixel_color, Camera::ray_color(&r, self.max_depth, world));
                        }
                        pixel_color.scalar_mul(self.pixel_samples_scale)
                    })
                    .collect() 
            })
            .collect(); 
        
        for row in image_pixels {
            for pixel_color in row {
                write_color(pixel_color);
            }
        }
    }
}