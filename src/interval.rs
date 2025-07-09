use std::f64::INFINITY;

pub struct Interval{
    pub min : f64,
    pub max : f64,
}

impl Interval{
    pub const fn new(min: f64, max: f64) -> Interval{
        Interval{
            min: min,
            max: max,
        }
    }

    pub const EMPTY: Interval = Interval::new(INFINITY, -1.0 * INFINITY);
    pub const _UNIVERSE: Self = Self::new(-1.0 * INFINITY, INFINITY);

    pub fn _size( x: Interval ) -> f64 {
        x.max - x.min
    }

    pub fn _contains(x: Interval ,y: f64 ) -> bool {
        x.min <= y && y <= x.max
    }

    pub fn surrounds ( x: &Interval , y: f64 ) -> bool {
        x.min < y && y < x.max
    }

    pub fn clamp (x: &Interval , y: f64) -> f64{
        if y < x.min {
            return x.min
        }
        if y > x.max{
            return x.max
        }
        return y
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self::EMPTY
    }
}