use std::f64::consts::PI;

use rand::random as rng;

pub fn random() -> f64 {
    rng()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * rng::<f64>()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
