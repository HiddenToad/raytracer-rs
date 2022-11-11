use crate::{clamp, rand, rand_range, Point};
use std::ops::Add;
use std::{fs::File, io::Write};

#[derive(Debug, Clone, Default)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    #[must_use]
    pub const fn new(r: f64, g: f64, b: f64) -> Self {
        Color { r, g, b }
    }

    #[must_use]
    pub const fn all(val: f64) -> Self {
        Self::new(val, val, val)
    }

    #[must_use]
    pub const fn black() -> Self {
        Self::all(0.)
    }

    #[must_use]
    pub const fn white() -> Self {
        Self::all(1.)
    }

    #[must_use]
    pub fn random() -> Self {
        Self::new(rand(), rand(), rand())
    }

    #[must_use]
    pub fn rand_range(min: f64, max: f64) -> Self {
        Self::new(
            rand_range(min, max),
            rand_range(min, max),
            rand_range(min, max),
        )
    }

    #[must_use]
    pub fn as_output(&self, samples: u32) -> Vec<u8> {
        let scale = 1. / f64::from(samples);
        let r = f64::sqrt(self.r * scale);
        let g = f64::sqrt(self.g * scale);
        let b = f64::sqrt(self.b * scale);

        format!(
            "{} {} {}\n",
            (clamp(r, 0., 0.999) * 256.) as u8,
            (clamp(g, 0., 0.999) * 256.) as u8,
            (clamp(b, 0., 0.999) * 256.) as u8
        )
        .as_bytes()
        .to_owned()
    }
    pub fn output_to(&self, f: &mut File, samples: u32) -> Result<(), std::io::Error> {
        f.write_all(&self.as_output(samples))
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl std::ops::Mul for Color {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.r * rhs.r, self.g * rhs.g, self.b * rhs.b)
    }
}

impl From<Point> for Color {
    fn from(p: Point) -> Self {
        Color::new(p.x, p.y, p.z)
    }
}
