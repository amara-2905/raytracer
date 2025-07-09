use crate::{random_double, random_double_x};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3{
    pub e : [f64 ; 3],
}

pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(e0:f64 ,e1:f64 ,e2:f64) -> Vec3{
        Vec3 { e : [e0 ,e1 ,e2] }
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn add( u:Vec3, v:Vec3 ) -> Vec3{
        Vec3 {
            e : [
                u.e[0] + v.e[0],
                u.e[1] + v.e[1],
                u.e[2] + v.e[2],
            ]
        }
    }

    pub fn sub( u:Vec3, v:Vec3 ) -> Vec3{
        Vec3 {
            e : [
                u.e[0] - v.e[0],
                u.e[1] - v.e[1],
                u.e[2] - v.e[2],
            ]
        }
    }

    pub fn scalar_mul (&self , t : f64) -> Vec3 {
        Vec3 {
            e : [
                t * self.e[0],
                t * self.e[1],
                t * self.e[2],
            ]
        }
    }

    pub fn scalar_div (&self , t : f64) -> Vec3 {
        Vec3 {
            e : [
                self.e[0]/t,
                self.e[1]/t,
                self.e[2]/t,
            ]
        }
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }

    pub fn length(&self) -> f64{
        self.length_squared().sqrt()
    }

    pub fn dot_product( u:Vec3 , v:Vec3 ) -> f64 {
        u.e[0]*v.e[0] + u.e[1]*v.e[1] + u.e[2]*v.e[2]
    }

    pub fn cross_product( u:Vec3 , v:Vec3 ) -> Vec3 {
        Vec3 { 
            e:[
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0]
            ],
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        Vec3 { 
            e:[
                self.x()/self.length(),
                self.y()/self.length(),
                self.z()/self.length()
            ],
        }
    }

    pub fn random_vector1() -> Vec3{
        Vec3::new(random_double(),random_double(),random_double())
    }

    pub fn random_vector2(min: f64, max:f64) -> Vec3{
        Vec3::new(random_double_x(min, max),random_double_x(min, max),random_double_x(min, max))
    }

    pub fn random_unit_vector() -> Vec3{
        loop{
            let p = Vec3::random_vector2(-1 as f64, 1 as f64);
            let lensq = p.length_squared();
            let num = 1e-160;
            if (num) < lensq && lensq <= 1.0{
                return p.scalar_div(lensq.sqrt() as f64)
            }
        }
    }

    pub fn _random_on_hemisphere ( normal: Vec3 ) -> Vec3{
        let on_unit_sphere = Vec3::random_unit_vector();
        if Vec3::dot_product(on_unit_sphere, normal) > 0.0{
            on_unit_sphere
        }else{
            on_unit_sphere.scalar_mul(-1.0)
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.e[0].abs() < s && self.e[1].abs() < s && self.e[2].abs() < s
    }

    pub fn reflect(v: Vec3, n: Vec3 ) -> Vec3 {
        return Vec3::sub(v, n.scalar_mul(2.0 * Vec3::dot_product(v, n)))
    }

    pub fn mul( u:Vec3, v:Vec3 ) -> Vec3{
        Vec3::new(u.x()*v.x(), u.y()*v.y(), u.z()*v.z())
    }

    pub fn refract(uv: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = Vec3::dot_product(uv.scalar_mul(-1.0),n).min(1.0);
        let r_out_perp = (Vec3::add(uv, n.scalar_mul(cos_theta))).scalar_mul(etai_over_etat);
        let r_out_parallel = n.scalar_mul(-1.0 * ((1.0 - r_out_perp.length_squared()).abs()).sqrt());
        Vec3::add(r_out_parallel, r_out_perp)
    }

    pub fn random_in_unit_disk() -> Vec3{
        loop{
            let p = Vec3::new(random_double_x(-1.0, 1.0),random_double_x(-1.0, 1.0),0.0);
            if p.length_squared() < 1.0{
                return p 
            }
        }
    }
}
