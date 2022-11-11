use crate::{Collision, Facing, Material, Point, Ray, WORLD};
use std::sync::Arc;

pub trait Collidable {
    fn collide(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision>;
}

pub type DynCollidable = Box<dyn Collidable + Send + Sync>;
pub type CollidableVec = Vec<DynCollidable>;

impl Collidable for CollidableVec {
    fn collide(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let mut closest_col = None;
        let mut closest = t_max;

        for item in self.iter() {
            if let Some(collision) = item.collide(ray, t_min, closest) {
                closest_col = Some(collision.clone());
                closest = collision.dist;
            }
        }

        closest_col
    }
}

pub fn add_to_world(object: DynCollidable) {
    WORLD.write().unwrap().push(object);
}

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Arc<dyn Material>,
}

unsafe impl Send for Sphere {}
unsafe impl Sync for Sphere {}

impl Sphere {
    #[must_use]
    pub fn new(center: Point, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    #[must_use]
    pub fn boxed(center: Point, radius: f64, material: Arc<dyn Material>) -> Box<Self> {
        Box::new(Self::new(center, radius, material))
    }
}

impl Collidable for Sphere {
    fn collide(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Collision> {
        let oc = ray.origin - self.center;
        let a = ray.direction.len_sq();
        let b = oc.dot_product(ray.direction);
        let c = oc.len_sq() - self.radius.powf(2.);
        let discriminant = b * b - a * c;
        if discriminant < 0. {
            None
        } else {
            let disc_sq = discriminant.sqrt();
            let mut root = (-b - disc_sq) / a;
            if root < t_min || t_max < root {
                root = (-b + disc_sq) / a;
                if root < t_min || t_max < root {
                    return None;
                }
            }

            let p = ray.at(root);
            let outward_normal = (p - self.center) / self.radius;
            let mut collision = Collision::new(
                p,
                outward_normal,
                root,
                Facing::Front,
                self.material.clone(),
            );
            collision.set_face_normal(ray, outward_normal);
            Some(collision)
        }
    }
}
