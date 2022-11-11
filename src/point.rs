use crate::{rand, rand_range, Color};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone, Default)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    #[must_use]
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    #[must_use]
    pub const fn all(val: f64) -> Self {
        Point::new(val, val, val)
    }

    #[must_use]
    pub const fn origin() -> Self {
        Point::all(0.)
    }

    #[must_use]
    pub const fn from_x(x: f64) -> Self {
        Self::new(x, 0., 0.)
    }

    #[must_use]
    pub const fn from_y(y: f64) -> Self {
        Self::new(0., y, 0.)
    }

    #[must_use]
    pub const fn from_z(z: f64) -> Self {
        Self::new(0., 0., z)
    }

    #[must_use]
    pub fn random() -> Self {
        Self::new(rand(), rand(), rand())
    }

    #[must_use]
    pub fn random_range(min: f64, max: f64) -> Self {
        Self::new(
            rand_range(min, max),
            rand_range(min, max),
            rand_range(min, max),
        )
    }

    #[must_use]
    pub fn random_in_unit_sphere() -> Self {
        loop {
            let v = Vec3::random_range(-1., 1.);
            if v.len_sq() >= 1. {
                continue;
            } else {
                return v;
            }
        }
    }

    #[must_use]
    pub fn random_in_unit_disk() -> Self{
        loop{
            let p = Vec3::new(rand_range(-1., 1.), rand_range(-1., 1.), 0.);
            if p.len_sq() < 1. {
                return p;
            }
        }
    }

    #[must_use]
    pub fn random_unit_vector() -> Self {
        Point::random_in_unit_sphere().unit()
    }

    pub fn dot_product(&self, rhs: Point) -> f64 {
        (self.x * rhs.x) + (self.y * rhs.y) + (self.z * rhs.z)
    }

    pub fn cross(&self, rhs: Point) -> Self {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    pub fn len_sq(&self) -> f64 {
        self.x.powf(2.) + self.y.powf(2.) + self.z.powf(2.)
    }
    pub fn len(&self) -> f64 {
        self.len_sq().sqrt()
    }
    pub fn unit(&self) -> Self {
        *self / self.len()
    }

    pub fn is_near_zero(&self) -> bool {
        const EPSILON: f64 = 0.0000001;
        self.x.abs() < EPSILON && self.y.abs() < EPSILON && self.z.abs() < EPSILON
    }
    pub fn reflect(&self, rhs: Point) -> Self {
        *self - 2. * self.dot_product(rhs) * rhs
    }
    pub fn refract(&self, rhs: Point, refraction_ratio: f64) -> Self {
        let cos_theta = f64::min(self.dot_product(rhs), 1.);
        let out_perp = refraction_ratio * (*self + cos_theta * rhs);
        let out_parallel = -f64::sqrt(f64::abs(1. - out_perp.len_sq())) * rhs;
        out_parallel + out_perp
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y, -self.z)
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl Div for Point {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        self * Point::new(rhs, rhs, rhs)
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        self / Point::new(rhs, rhs, rhs)
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        rhs.mul(self)
    }
}

impl Div<Point> for f64 {
    type Output = Point;
    fn div(self, rhs: Point) -> Self::Output {
        rhs.div(self)
    }
}

impl From<Color> for Point {
    fn from(c: Color) -> Self {
        Self::new(c.r, c.g, c.b)
    }
}

pub type Vec3 = Point;
