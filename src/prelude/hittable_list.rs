use crate::prelude::hittable::Hittable;
use crate::prelude::{HitRecord, Ray};
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl Debug for dyn Hittable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Hittable {{ metadata: {}}}", self.metadata())
    }
}

impl std::fmt::Display for dyn Hittable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn push(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object)
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.objects
            .iter()
            .filter_map(|object| object.hit(ray, t_min, t_max))
            .min_by(|hit_record_1, hit_record_2| {
                hit_record_1
                    .time
                    .partial_cmp(&hit_record_2.time)
                    .unwrap_or(Ordering::Equal)
            })
    }
}
