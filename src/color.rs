use std::io::Write;
use crate::{interval::Interval, vec3::Vec3};

fn linear_to_gamma(linear_component: f64) -> f64{
    if linear_component > 0.0 {
        return linear_component.sqrt()
    }
    0.0
}

pub fn write_color<W: Write>(out: &mut W, pixel_color: Vec3) {
    let r = linear_to_gamma(pixel_color.x());
    let g = linear_to_gamma(pixel_color.y());
    let b = linear_to_gamma(pixel_color.z());

    let intensity = Interval::new(0.000, 0.999);

    let rbyte = (256.0 * Interval::clamp(&intensity, r)) as i64;
    let gbyte = (256.0 * Interval::clamp(&intensity, g)) as i64;
    let bbyte = (256.0 * Interval::clamp(&intensity, b)) as i64;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}