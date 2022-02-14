use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Sub, AddAssign, Mul, MulAssign, SubAssign, DivAssign, Div, Neg, Index};
use rand::distributions::{Distribution, Standard};
use rand::Rng;


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

unsafe impl Send for Vec3 {}
unsafe impl Sync for Vec3 {}

pub type Point = Vec3;

impl Default for Vec3 {
    fn default() -> Self {
        Self(0., 0., 0.)
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0.add_assign(rhs.0);
        self.1.add_assign(rhs.1);
        self.2.add_assign(rhs.2);
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0-rhs.0, self.1-rhs.1, self.2-rhs.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1, -self.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self * rhs.0, self * rhs.1, self * rhs.2)
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}


impl Div<Vec3> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Self(
            self.0 / rhs.0,
            self.1 / rhs.1,
            self.2 / rhs.2,
        )
    }
}

impl Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        Vec3(self / rhs.0, self / rhs.1, self / rhs.2)
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self(
            self.0 / rhs,
            self.1 / rhs,
            self.2 / rhs
        )
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
        self.2 /= rhs;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
        self.2 *= rhs;
    }
}


impl Vec3 {
    pub fn norm_squared(self) -> f64 {
        self.0.powi(2) + self.1.powi(2) + self.2.powi(2)
    }

    pub fn norm(self) -> f64 {
        self.norm_squared().sqrt()
    }

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn near_zero(self) -> bool {
        let tolerance: f64 = 1e-18;
        self.0.abs() <= tolerance &&
        self.1.abs() <= tolerance &&
        self.2.abs() <= tolerance
    }

}

impl Vec3 {

    pub fn rand_uniform(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max)
        )
    }
}

// impl<I: Iterator<Item=f64>> TryFrom<I> for Vec3 {
//     type Error = &'static str;
//
//     fn try_from(value: I) -> Result<Self, Self::Error> {
//
//         let candidates: Vec<f64> =
//             value
//                 // take 1 more than 3 (which should be None),
//                 // just to ensure there's exactly three entries.
//                 .take(4)
//                 .take_while(|val| val.is_some())
//                 .collect::<Vec<f64>>();
//
//         if candidates.len() != 3 {
//             Err("Exactly three f64's must be passed to create a Vec3.")
//         } else {
//             Ok(
//                 Self(
//                     candidates.get(0).unwrap(),
//                     candidates.get(1).unwrap(),
//                     candidates.get(2).unwrap(),
//                 )
//             )
//         }
//     }
// }

// impl<I: Into<[f64; 3]>> From<I> for Vec3 {
//     fn from(collection: I) -> Self {
//         let arr = collection.into();
//         Self(
//             *arr[0].unwrap(),
//             *arr[1].unwrap(),
//             *arr[2].unwrap(),
//         )
//     }
// }

impl From<[f64; 3]> for Vec3 {
    fn from(arr: [f64; 3]) -> Self {
        Self(arr[0], arr[1], arr[2])
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(arr: [f32; 3]) -> Self {
        Self(arr[0] as f64, arr[1] as f64, arr[2] as f64)
    }
}

    impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        let (x, y, z) = rng.gen();
        Vec3(x, y, z)
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "⟨{}, {}, {}⟩", self.0, self.1, self.2)
    }
}

// impl<I: IntoIterator<Item=f64>> PartialEq<I> for Vec3 {
//     fn eq(&self, other: &I) -> bool {
//
//         let candidates: Vec<f64> =
//             other
//                 .clone()
//                 .into_iter()
//                 // take 1 more than 3 (which should be None),
//                 // just to ensure there's exactly three entries.
//                 .take(4)
//                 .collect::<Vec<f64>>();
//
//         candidates.len() == 3
//             && *candidates.get(0).unwrap() == self.0
//             && *candidates.get(1).unwrap() == self.1
//             && *candidates.get(2).unwrap() == self.2
//     }
// }

impl Index<u32> for Vec3 {
    type Output = f64;

    fn index(&self, index: u32) -> &Self::Output {
        match index {
            0 => &self.0,
            1 => &self.1,
            2 => &self.2,
            _ => {
                panic!("Index {} is out of bounds. Must be one of: (0, 1, 2).", index);
            }
        }
    }
}

pub trait LinAlgRandGen {
    fn random_in_unit_disk() -> Self;
    fn random_in_unit_sphere() -> Self;
    fn random_unit_vector() -> Self;
    fn random_in_hemisphere(normal: Self) -> Self;
}

impl LinAlgRandGen for Vec3 {
    fn random_in_unit_disk() -> Self {
        loop {
            let mut generated = Self::rand_uniform(-1., 1.);
            generated.2 = 0.;

            if generated.norm_squared() < 1. {
                return generated
            }
        }
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            let generated = Self::rand_uniform(-1., 1.);
            if generated.norm_squared() < 1. {
                return generated;
            }
        }
    }

    fn random_unit_vector() -> Self {
        Self::random_in_unit_sphere().unit_vector()
    }

    fn random_in_hemisphere(normal: Self) -> Self {
        let random_in_same_sphere = Self::random_in_unit_sphere();
        if random_in_same_sphere.dot(normal) > 0. {
            random_in_same_sphere
        } else {
            -random_in_same_sphere
        }
    }
}


pub trait LinAlgOp {
    fn dot(self, rhs: Self) -> f64;
    fn cross(self, rhs: Self) -> Self;
    fn unit_vector(self) -> Self;
    fn reflect(self, rhs: Self) -> Self;
    fn refract(self, n: Self, etai_over_etat: f64) -> Self;
}

impl LinAlgOp for Vec3 {
    fn dot(self, rhs: Self) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2
    }

    fn cross(self, rhs: Self) -> Self {
        Self(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0
        )
    }

    fn unit_vector(self) -> Self {
        self / self.norm()
    }

    fn reflect(self, rhs: Self) -> Self {
        self - rhs * 2. * self.dot(rhs)
    }

    fn refract(self, n: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (1.0_f64).min((-self).dot(n));

        let r_out_perpendicular = (self + n * cos_theta) * etai_over_etat;
        let r_out_parallel = n * -(f64::abs(1. - r_out_perpendicular.norm_squared())).sqrt();
        r_out_perpendicular + r_out_parallel
    }
}


#[cfg(test)]
mod tests {

    use crate::commons::{LinAlgOp, LinAlgRandGen, Vec3};

    use rand::Rng;

    const TOLERANCE_LEVEL: f64 = 1e-12;

    fn close_enough(v1: Vec3, v2: Vec3) -> bool {
        (v1.0 - v2.0).abs() <= TOLERANCE_LEVEL &&
            (v1.1 - v2.1).abs() <= TOLERANCE_LEVEL &&
            (v1.2 - v2.2).abs() <= TOLERANCE_LEVEL
    }

    #[test]
    fn test_create_default() {
        let v: Vec3 = Vec3::default();
        assert_eq!(vec![v.0, v.1, v.2], vec![0., 0., 0.]);
    }

    #[test]
    fn test_mul_add() {
        let v = Vec3::new(3.0, 4.0, -2.0);
        let v_times_3 = Vec3::new(9.0, 12.0, -6.0);
        assert_eq!(v * 3., v_times_3);
    }

    #[test]
    fn test_f64_mul_vec3() {
        let v = Vec3::new(3.0, 4.0, -2.0);
        let v_times_3 = Vec3::new(9.0, 12.0, -6.0);
        assert_eq!(3. * v, v_times_3);
    }

    #[test]
    fn test_f64_div_vec3() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let three_div_v = Vec3::new(3.0, 1.5, 1.0);
        assert_eq!(3. / v, three_div_v);
    }
    #[test]
    fn test_from_slice() {

        let temp: [f64; 3] = [3.0, 2.0, 1.0];
        let v: Vec3 = temp.into();
    }

    #[test]
    fn test_norm() {
        let v: Vec3 = [3.0, 2.0, 1.0].into();
        assert_eq!(v.norm_squared(), 14.0);
        assert_eq!(v.norm(), 14.0_f64.sqrt());
    }

    #[test]
    fn test_near_zero() {
        let v: Vec3 = [1e-21, 1e-21, 1e-24].into();
        assert!(v.near_zero());
    }

    #[test]
    fn test_random_generated() {
        let mut rng = rand::thread_rng();
        let v: Vec3 = rng.gen();
    }

    #[test]
    fn test_random_uniform() {
        let vector_from_0_to_5 = Vec3::rand_uniform(3., 5.);

        assert!(3. <= vector_from_0_to_5.0 && vector_from_0_to_5.0 < 5.);
        assert!(3. <= vector_from_0_to_5.1 && vector_from_0_to_5.1 < 5.);
        assert!(3. <= vector_from_0_to_5.2 && vector_from_0_to_5.2 < 5.);
    }

    #[test]
    fn test_indexing() {
        let v1: Vec3 = [1., 2., 3.].into();
        assert_eq!(v1[0], v1.0, "Index at 0 does not equal to the x coordinate.");
        assert_eq!(v1[1], v1.1, "Index at 1 does not equal to the y coordinate.");
        assert_eq!(v1[2], v1.2, "Index at 2 does not equal to the z coordinate.");
    }

    #[test]
    fn test_linalgop_dot() {
        let v1: Vec3 = [2., 2., 2.].into();
        let v2: Vec3 = [3., 3.2, 5.].into();

        let observed: f64 = v1.dot(v2);
        let expected: f64 = 2. * 3. + 2. * 3.2 + 2. * 5.;

        assert_eq!(observed, expected, "Dot product of {} and {} must be {}.", v1, v2, expected);
    }

    #[test]
    fn test_linalgop_cross() {
        let v1: Vec3 = [2., 2., 2.].into();
        let v2: Vec3 = [3., 3.2, 5.].into();

        let observed: Vec3 = v1.cross(v2);
        let expected: Vec3 = [3.6, -4.0, 0.4].into();

        assert!(close_enough(observed, expected), "Cross product of {} and {} must be {}.", v1, v2, expected);
    }

    #[test]
    fn test_linalgop_unit_vector() {
        let v1: Vec3 = [1., 1., 1.].into();
        let expected: Vec3 = [1./f64::sqrt(3.); 3].into();

        let observed = v1.unit_vector();
        assert!(close_enough(observed, expected), "Unit vector of {} must be {}.", v1, expected);
    }

    #[test]
    fn test_linalgop_reflect() {
        let inbound: Vec3 = [1.; 3].into();
        let normal: Vec3 = [2.; 3].into();

        let observed = inbound.reflect(normal);
        let expected: Vec3 = [-23.; 3].into();
        assert!(
            close_enough(observed, expected),
            "Reflection of inbound vector {} across normal {} must be {}.\nFound {}", inbound, normal, expected, observed
        );
    }

    #[test]
    fn test_linalgop_refract() {
        let inbound: Vec3 = [1.; 3].into();
        let normal: Vec3 = [2.; 3].into();
        let ratio: f64 = 0.5;

        let observed = inbound.refract(normal, ratio);
        let expected: Vec3 = [-24.447295321496416; 3].into();

        assert!(
            close_enough(observed, expected),
            "Refraction of inbound vector {} across normal {} and eta {} must be {}.\nFound {}", inbound, normal, ratio, expected, observed
        );
    }

    #[test]
    fn random_in_unit_disk() {
        let v = Vec3::random_in_unit_disk();
        assert_eq!(v.2, 0.);
        assert!(v.norm_squared() <= 1.);
        assert!(v.norm() <= 1.);
    }

    #[test]
    fn random_in_unit_sphere() {
        let v = Vec3::random_in_unit_sphere();
        assert!(v.norm() <= 1.);
        assert!(v.norm_squared() <= 1.);
        assert!(-1. <= v.0 && v.0 <= 1.);
        assert!(-1. <= v.1 && v.1 <= 1.);
        assert!(-1. <= v.2 && v.2 <= 1.);
    }
}