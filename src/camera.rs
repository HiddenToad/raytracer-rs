use crate::{deg_to_rad, Point, Ray, Vec3};

#[derive(Clone)]
pub struct Camera {
    aspect_ratio: f64,
    view_h: f64,
    view_w: f64,
    origin: Point,
    lower_left: Point,
    x_axis: Vec3,
    y_axis: Vec3,
    look_from: Point,
    look_at: Point,
    vup: Vec3,
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    focus_dist: f64,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            aspect_ratio: 1.,
            view_h: 2.,
            view_w: 2.,
            origin: Point::new(0., 0., 0.),
            x_axis: Vec3::from_x(2.),
            y_axis: Vec3::from_y(2.),
            lower_left: (Point::new(0., 0., 0.)
                - Point::from_x(2.) / 2.
                - Point::from_y(2.) / 2.
                - Vec3::from_z(1.)),
            look_from: Vec3::all(0.),
            look_at: Vec3::new(1., 0., -1.),
            vup: Vec3::from_y(1.),
            lens_radius: 1.,
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            focus_dist: Vec3::new(-1., 0., 1.).len(), //(lookfrom - lookat).len()
        }
    }
}

impl Camera {
    fn update_dependent_components(self) -> Self {
        let w = (self.look_from - self.look_at).unit();
        let u = self.vup.cross(w).unit();
        let v = w.cross(u);
        let focus_dist = (self.look_from - self.look_at).len();
        let origin = self.look_from;
        let view_w = self.aspect_ratio * self.view_h;
        let x_axis = view_w * u * focus_dist;
        let y_axis = self.view_h * v * focus_dist;
        let lower_left = origin - x_axis / 2. - y_axis / 2. - w * focus_dist;

        Camera {
            view_w,
            lower_left,
            x_axis,
            y_axis,
            origin,
            w,
            u,
            v,
            focus_dist,
            ..self
        }
    }

    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn aspect_ratio(self, aspect_ratio: f64) -> Self {
        Camera {
            aspect_ratio,
            ..self
        }
        .update_dependent_components()
    }

    #[must_use]
    pub fn viewport_height(self, view_h: f64) -> Self {
        Camera { view_h, ..self }.update_dependent_components()
    }

    #[must_use]
    pub fn origin(self, origin: Point) -> Self {
        Camera { origin, ..self }.update_dependent_components()
    }

    #[must_use]
    pub fn vfov(self, value: f64) -> Self {
        let h = (deg_to_rad(value) / 2.).tan();

        self.viewport_height(2. * h).update_dependent_components()
    }

    #[must_use]
    pub fn look_at(self, lookat: Point) -> Self {
        Camera {
            look_at: lookat,
            ..self
        }
        .update_dependent_components()
    }

    #[must_use]
    pub fn look_from(self, lookfrom: Point) -> Self {
        Camera {
            look_from: lookfrom,
            ..self
        }
        .update_dependent_components()
    }

    #[must_use]
    pub fn lens_radius(self, radius: f64) -> Self {
        Camera {
            lens_radius: radius,
            ..self
        }
        .update_dependent_components()
    }

    #[must_use]
    pub fn focus_dist(self, dist: f64) -> Self {
        Camera {
            focus_dist: dist,
            ..self
        }
        .update_dependent_components()
    }

    #[must_use]
    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let in_disk = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * in_disk.x + self.v * in_disk.y;

        Ray::new(
            self.origin + offset,
            self.lower_left + (x * self.x_axis) + (y * self.y_axis) - self.origin - offset,
        )
    }
}
