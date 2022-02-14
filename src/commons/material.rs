use crate::commons::{HitRecord, Ray, Vec3};

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian,
}
// pub struct Material {}
