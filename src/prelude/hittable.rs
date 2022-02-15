use crate::prelude::{LinAlgOp, Material, Point, Ray, Vec3};
use std::sync::Arc;

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Arc<Material>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

#[derive(Clone, Debug)]
pub struct HitRecord {
    /// The point of intersection.
    pub point: Point,
    /// The canonical vector that is normal to the object's surface at the point of intersection.
    pub normal: Vec3,
    /// The moment in time that the ray hits the object.
    pub time: f64,
    /// true if the ray hits from outside the object, false otherwise.
    pub is_front_facing: bool,
    /// The kind of material is hit.
    pub material: Arc<Material>,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn metadata(&self) -> String {
        String::from("Unknown")
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin - self.center;
        let a = ray.direction.norm_squared();
        let half_b = oc.dot(ray.direction);
        let c = oc.norm_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0. {
            return None;
        }

        let hit_time = {
            vec![
                (-half_b - discriminant.sqrt()) / a,
                (-half_b + discriminant.sqrt()) / a,
            ]
            .into_iter()
            .filter(|&time| t_min <= time && time <= t_max)
            .min_by(|t1, t2| t1.partial_cmp(t2).unwrap())
        };
        hit_time?; // If None, return already.

        let hit_time = hit_time.unwrap();

        let outward_facing_normal = (ray.at(hit_time) - self.center) / self.radius;
        let is_front_facing = ray.direction.dot(outward_facing_normal) < 0.;

        Some(HitRecord {
            point: ray.at(hit_time),
            normal: if is_front_facing {
                outward_facing_normal
            } else {
                -outward_facing_normal
            },
            time: hit_time,
            is_front_facing,
            material: self.material.clone(),
        })
    }

    fn metadata(&self) -> String {
        format!(
            "Sphere {{ center: {}, radius: {} }}",
            self.center, self.radius
        )
    }
}
