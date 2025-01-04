use core::f64;
use rand::prelude::*;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * f64::consts::PI / 180.0;
}

pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    // Returns a random real in [0,1s).
    return rng.gen::<f64>();
}

pub fn random_f64_bounded(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    // Returns a random real in [min,max).
    return min + (max - min) * rng.gen::<f64>();
}
