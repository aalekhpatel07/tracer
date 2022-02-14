use crate::commons::{Point, Ray, Vec3};

#[derive(Clone, Debug)]
pub struct Camera {
    pub center: Point,
    pub lower_left_corner: Point,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(center: Point, aspect_ratio: f64, viewport_height: f64, focal_length: f64) -> Self {
        let horizontal = Vec3::new(viewport_height * aspect_ratio, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);

        Self {
            center,
            horizontal,
            vertical,
            lower_left_corner: center
                - horizontal / 2.
                - vertical / 2.
                - Vec3::new(0., 0., focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &(self.center),
            &(self.lower_left_corner + u * self.horizontal + v * self.vertical - self.center),
        )
    }
}
