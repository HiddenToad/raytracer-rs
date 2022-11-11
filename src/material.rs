use std::sync::Arc;

use crate::{rand, Collision, Color, Facing, Ray, Vec3};

#[derive(Default)]
pub enum ScatterOutcome {
    Scattered,

    #[default]
    Absorbed,
}

#[derive(Default)]
pub struct ScatterResult {
    pub outcome: ScatterOutcome,
    pub attenuation: Color,
    pub scattered_ray: Ray,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, collision: &Collision) -> ScatterResult;
}

pub struct Lambertian {
    albedo: Color,
}
impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
    pub fn new_arc(albedo: Color) -> Arc<Self> {
        Arc::new(Self::new(albedo))
    }
}
impl Material for Lambertian {
    fn scatter(&self, _: &Ray, collision: &Collision) -> ScatterResult {
        let mut result = ScatterResult::default();
        let mut direction = collision.normal + Vec3::random_unit_vector();
        if direction.is_near_zero() {
            direction = collision.normal;
        }
        result.scattered_ray = Ray::new(collision.point, direction);
        result.attenuation = self.albedo.clone();
        result.outcome = ScatterOutcome::Scattered;
        result
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1. { fuzz } else { 1. },
        }
    }
    pub fn new_arc(albedo: Color, fuzz: f64) -> Arc<Self> {
        Arc::new(Self::new(albedo, fuzz))
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, collision: &Collision) -> ScatterResult {
        let mut result = ScatterResult::default();
        let reflected = ray_in.direction.unit().reflect(collision.normal);
        result.scattered_ray = Ray::new(
            collision.point,
            reflected + self.fuzz * Vec3::random_unit_vector(),
        );
        result.attenuation = self.albedo.clone();
        result.outcome = if result.scattered_ray.direction.dot_product(collision.normal) > 0. {
            ScatterOutcome::Scattered
        } else {
            ScatterOutcome::Absorbed
        };
        result
    }
}

pub struct Dielectric {
    ir: f64, //index of refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
    pub fn new_arc(ir: f64) -> Arc<Self> {
        Arc::new(Self::new(ir))
    }

    fn schlick_approximation(cosine: f64, idx: f64) -> f64 {
        let r0 = (1. - idx) / (1. + idx);
        let r0 = r0.powf(2.);
        r0 + (1. - r0) * (1. - cosine).powf(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, collision: &Collision) -> ScatterResult {
        let mut result = ScatterResult::default();
        let rr = match collision.facing {
            Facing::Front => 1. / self.ir,
            Facing::Back => self.ir,
        };
        let unit_direction = ray_in.direction.unit();
        let cos_theta = f64::min((-unit_direction).dot_product(collision.normal), 1.);
        let sin_theta = (1. - cos_theta.powf(2.)).sqrt();
        let cannot_refract = rr * sin_theta > 1.;
        let direction: Vec3 =
            if cannot_refract || Dielectric::schlick_approximation(cos_theta, rr) > rand() {
                unit_direction.reflect(collision.normal)
            } else {
                unit_direction.refract(collision.normal, rr)
            };

        result.scattered_ray = Ray::new(collision.point, direction);
        result.attenuation = Color::all(1.);
        result.outcome = ScatterOutcome::Scattered;
        result
    }
}
