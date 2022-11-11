use rand::{thread_rng, Rng};

pub fn deg_to_rad(deg: f64) -> f64 {
    deg * std::f64::consts::PI / 180.
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

pub fn rand_range(min: f64, max: f64) -> f64 {
    thread_rng().gen_range(min..max)
}
pub fn rand() -> f64 {
    rand_range(0., 1.)
}
