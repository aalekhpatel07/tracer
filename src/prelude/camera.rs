use crate::prelude::{LinAlgOp, LinAlgRandGen, Point, Ray, Vec3};

#[derive(Clone, Debug)]
pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

pub struct Degrees(pub f64);
pub struct Radians(pub f64);

impl From<Degrees> for Radians {
    fn from(degrees: Degrees) -> Self {
        Self(degrees.0 * std::f64::consts::PI / 180.)
    }
}

impl From<Radians> for Degrees {
    fn from(radians: Radians) -> Self {
        Self(radians.0 * 180. / std::f64::consts::PI)
    }
}

pub enum Angle {
    Degrees(Degrees),
    Radians(Radians),
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        view_up: Vec3,
        vertical_field_of_view: Angle,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = match vertical_field_of_view {
            Angle::Radians(radians) => radians.0,
            Angle::Degrees(degrees) => Radians::from(degrees).0,
        };

        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = view_up.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2. - vertical / 2. - focus_distance * w;
        let lens_radius = aperture / 2.;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * rd.0 + self.v * rd.1;

        Ray::new(
            &(self.origin + offset),
            &(self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset),
        )
    }
}

#[derive(Clone, Debug, Default)]
pub struct Image {
    pub height: usize,
    pub width: usize,
    pub aspect_ratio: f64,
}

impl Image {
    pub fn new(width: usize, aspect_ratio: f64) -> Self {
        Self {
            height: (width as f64 / aspect_ratio).round() as usize,
            width,
            aspect_ratio,
        }
    }
}
