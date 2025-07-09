mod vec3;
mod color;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod interval;
mod camera;
mod material;

use std::{f64::consts::PI, sync::Arc};
use rand::Rng;
use crate::{camera::Camera, hittable_list::HittableList, material::{Dielectric, EmptyMaterial, Lambertian, Metal}, sphere::Sphere, vec3::Vec3};

fn degrees_to_radians (degrees : f64) -> f64 {
    (degrees * PI) / 180.0
}

fn random_double() -> f64 {
    let mut rng = rand::thread_rng();
    let random_number: f64 = rng.r#gen();
    random_number
}

fn random_double_x(min: f64,max: f64) -> f64 {
    min + (max - min) * random_double()
}

fn main(){

    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Vec3::new(0.5,0.5,0.5)));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0,-1000.0,0.0), 1000.0, ground_material)));

    for a in -11..11{
        for b in -11..11{
            let choose_mat = random_double();
            let center = Vec3::new(a as f64 + 0.9 * random_double(),0.2,b as f64 + 0.9 * random_double());

            if Vec3::sub(center, Vec3::new(4.0,0.2,0.0)).length() > 0.9 {
                let mut sphere_material: Arc<dyn material::Material> = Arc::new(EmptyMaterial);

                if choose_mat < 0.8 {
                    let albedo = Vec3::mul(Vec3::random_vector1(), Vec3::random_vector1());
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    world.add(Arc::new(Sphere::new(center,0.2,sphere_material)));   
                } else if choose_mat <0.95 {
                    let albedo = Vec3::mul(Vec3::random_vector1(), Vec3::random_vector1());
                    let fuzz = random_double();
                    sphere_material = Arc::new(Metal::new(albedo,fuzz));
                    world.add(Arc::new(Sphere::new(center,0.2,sphere_material)));   
                } else{
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center,0.2,sphere_material)));   
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(Vec3::new(0.0,1.0,0.0),1.0,material1)));   

    let material2 = Arc::new(Lambertian::new(Vec3::new(0.4,0.2,0.1)));
    world.add(Arc::new(Sphere::new(Vec3::new(-4.0,1.0,0.0),1.0,material2))); 

    let material3 = Arc::new(Metal::new(Vec3::new(0.7,0.6,0.5),0.0));
    world.add(Arc::new(Sphere::new(Vec3::new(4.0,1.0,0.0),1.0,material3))); 

    
    let aspect_ratio = 16.0/9.0;
    let image_width = 1200;
    let samples_per_pixel: i64 = 500;
    let max_depth = 50;
    let vfov: i64 = 20;
    let lookform = Vec3::new(13.0,2.0,3.0);
    let lookat = Vec3::new(0.0,0.0,0.0);
    let vup = Vec3::new(0.0,1.0,0.0);
    let defocus_angle = 0.6;
    let focus_dist = 10.0;
    let mut camera = Camera::new(aspect_ratio, image_width,samples_per_pixel as i64,max_depth,vfov,lookform,lookat,vup,defocus_angle,focus_dist);

    camera.render(&world);
}