use crate::point::*;
use crate::Collidable;
use crate::Material;
use crate::{color::*, ScatterOutcome, ScatterResult};
use crate::{MAX_DEPTH, WORLD};
use std::cmp::Ordering::*;
use std::sync::Arc;

#[derive(Clone)]
pub enum Facing {
    Front,
    Back,
}

#[derive(Clone)]
pub struct Collision {
    pub point: Point,
    pub normal: Vec3,
    pub dist: f64,
    pub facing: Facing,
    pub material: Arc<dyn Material>,
}

impl Collision {
    pub fn new(
        point: Point,
        normal: Vec3,
        dist: f64,
        facing: Facing,
        material: Arc<dyn Material>,
    ) -> Self {
        Self {
            point,
            normal,
            dist,
            facing,
            material,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        match ray.direction.dot_product(outward_normal).total_cmp(&0.) {
            Less => {
                self.normal = outward_normal;
                self.facing = Facing::Front;
            }
            Greater | Equal => {
                self.normal = -outward_normal;
                self.facing = Facing::Back;
            }
        };
    }
}

#[derive(Clone, Default)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub const fn new(origin: Point, direction: Vec3) -> Self {
        Ray { origin, direction }
    }
    pub fn at(&self, t: f64) -> Point {
        self.origin + (t * self.direction)
    }

    pub fn collide<T: Collidable>(&self, other: T, t_min: f64, t_max: f64) -> Option<Collision> {
        other.collide(self, t_min, t_max)
    }

    fn do_color(&self, depth: i32) -> Color {
        if depth <= 0 {
            return Color::black();
        }

        let world_guard = WORLD.read().unwrap();

        match world_guard.collide(self, 0.001, f64::INFINITY) {
            Some(collision) => {
                let ScatterResult {
                    outcome,
                    attenuation,
                    scattered_ray,
                } = collision.material.scatter(self, &collision);

                drop(world_guard);

                match outcome {
                    ScatterOutcome::Scattered => attenuation * scattered_ray.do_color(depth - 1),
                    ScatterOutcome::Absorbed => Color::black(),
                }
            }
            None => {
                let t = 0.5 * (self.direction.unit().y + 1.);
                Color::from((1. - t) * Point::all(1.) + t * Point::new(0.5, 0.7, 1.))
            }
        }
    }

    pub fn color(&self) -> Color {
        self.do_color(MAX_DEPTH)
    }
}
