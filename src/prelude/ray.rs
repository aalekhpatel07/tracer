use crate::prelude::{Point, Vec3};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Point,
}

impl Default for Ray {
    fn default() -> Self {
        Self {
            origin: Vec3::default(),
            direction: Vec3::default(),
        }
    }
}

impl Display for Ray {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.origin.fmt(f).unwrap();
        f.write_str(" + t ").unwrap();
        self.direction.fmt(f)
    }
}

impl Ray {
    pub fn new(origin: &Point, direction: &Vec3) -> Self {
        Self {
            origin: *origin,
            direction: *direction,
        }
    }

    pub fn at(&self, time: f64) -> Point {
        self.origin + self.direction * time
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::{Point, Ray, Vec3};

    #[test]
    fn ray_default() {
        let ray = Ray::default();
        assert_eq!(ray.origin, [0.; 3].into());
        assert_eq!(ray.direction, [0.; 3].into());
    }

    #[test]
    fn ray_at() {
        let ray = Ray::new(&Point::new(0., 0., 0.), &Point::new(1., 1., 1.));
        assert_eq!(ray.at(1.), [1.; 3].into());
        assert_eq!(ray.at(0.), [0.; 3].into());
    }

    #[test]
    fn ray_display() {
        let ray = Ray::new(&Point::new(0., 0., 0.), &Point::new(1., 1., 1.));

        let ray_as_str = format!("{}", ray);
        assert_eq!(ray_as_str, "⟨0, 0, 0⟩ + t ⟨1, 1, 1⟩")
    }
}
