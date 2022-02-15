use crate::prelude::{reflectance, HitRecord, LinAlgOp, LinAlgRandGen, Ray, Vec3};
use rand::{thread_rng, Rng};

pub trait Scatter {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

#[derive(Clone, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3 },
    Metal { albedo: Vec3, fuzz: f64 },
    Dielectric { index_of_refraction: f64 },
}

impl Scatter for Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

                if scatter_direction.near_zero() {
                    scatter_direction = hit_record.normal
                };

                Some((*albedo, Ray::new(&hit_record.point, &scatter_direction)))
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = ray.direction.unit_vector().reflect(hit_record.normal);

                let scattered = Ray::new(
                    &hit_record.point,
                    &(reflected + *fuzz * Vec3::random_in_unit_sphere()),
                );

                match scattered.direction.dot(hit_record.normal) > 0. {
                    true => Some((*albedo, scattered)),
                    false => None,
                }
            }
            Material::Dielectric {
                index_of_refraction,
            } => {
                let refraction_ratio: f64 = if hit_record.is_front_facing {
                    1. / index_of_refraction
                } else {
                    *index_of_refraction
                };

                let unit_direction = ray.direction.unit_vector();

                // Total internal reflection.
                let cos_theta = (-unit_direction).dot(hit_record.normal).min(1.);
                let sin_theta = (1. - cos_theta * cos_theta).sqrt();

                let total_internal_reflection_occurs = refraction_ratio * sin_theta > 1.;

                // Polynomial approximation for reflectivity of glass based on angle.
                // By Christophe Schlick.
                let reflectivitiy_of_glass: f64 = reflectance(cos_theta, refraction_ratio);

                let mut rng = thread_rng();
                let scatter_direction =
                    match total_internal_reflection_occurs || reflectivitiy_of_glass > rng.gen() {
                        true => unit_direction.reflect(hit_record.normal),
                        false => unit_direction.refract(hit_record.normal, refraction_ratio),
                    };

                // let refraction_direction = unit_direction.refract(hit_record.normal, refraction_ratio);
                let scattered = Ray::new(&hit_record.point, &scatter_direction);
                Some((Vec3::new(1., 1., 1.), scattered))
            }
        }
    }
}
