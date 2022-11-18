/**
 * Point|Vector data type
 * Maybe use an Enum here?
 */
use crate::math::F3D;
use glm::{vec4, Vec4};

pub type Tuple = Vec4;
pub type Point = Tuple;
pub type Vector = Tuple;

/*
 * Legacy Tuple implementation superceded by glm types
 */

/*
use std::cmp::{Eq, PartialEq};
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Debug)]
pub struct Tuple {
    pub x: F3D,
    pub y: F3D,
    pub z: F3D,
    pub w: F3D,
}

impl Tuple {
    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    fn magnitude(&self) -> F3D {
        let exp_sum = self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2);
        exp_sum.sqrt()
    }

    fn normalize(&self) -> Self {
        let mag = self.magnitude();
        self.clone() / mag
    }

    fn dot_product(&self, other: &Self) -> F3D {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    fn cross_product(&self, other: &Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0, // only dealing with vectors
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        f_equals(self.x, other.x)
            && f_equals(self.y, other.y)
            && f_equals(self.z, other.z)
            && f_equals(self.w, other.w)
    }
}

impl Eq for Tuple {}

impl Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        tuple(-self.x, -self.y, -self.z, -self.w)
    }
}

impl Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        assert!(self.w + other.w <= 1.0);
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        assert!(self.w - other.w >= 0.0);
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<F3D> for Tuple {
    type Output = Self;

    fn mul(self, scalar: F3D) -> Self::Output {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }
}

impl Div<F3D> for Tuple {
    type Output = Self;

    fn div(self, scalar: F3D) -> Self::Output {
        assert!(scalar != 0.0);
        self * (1.0 / scalar)
    }
}

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "x: {}, y: {}, z: {}, w: {}",
            self.x, self.y, self.z, self.w
        )
    }
}

// Might be useful later
enum GFX {
    Point(Tuple),
    Vector(Tuple),
    Color(Tuple),
}
*/

// common constructors

pub fn tuple(x: F3D, y: F3D, z: F3D, w: F3D) -> Tuple {
    vec4(x, y, z, w)
}

pub fn point(x: F3D, y: F3D, z: F3D) -> Point {
    tuple(x, y, z, 1.0)
}

pub fn vector(x: F3D, y: F3D, z: F3D) -> Vector {
    tuple(x, y, z, 0.0)
}

pub fn is_point(t: Tuple) -> bool {
    t.w == 1.0
}

pub fn is_vector(t: Tuple) -> bool {
    t.w == 0.0
}

pub fn point_x() -> Point {
    point(1.0, 0.0, 0.0)
}

pub fn point_y() -> Point {
    point(0.0, 1.0, 0.0)
}

pub fn point_z() -> Point {
    point(0.0, 0.0, 1.0)
}

pub fn point_zero() -> Point {
    point(0.0, 0.0, 0.0)
}

pub fn vector_x() -> Vector {
    vector(1.0, 0.0, 0.0)
}

pub fn vector_y() -> Vector {
    vector(0.0, 1.0, 0.0)
}

pub fn vector_z() -> Vector {
    vector(0.0, 0.0, 1.0)
}

pub fn reflect(in_v: Vector, normal: Vector) -> Vector {
    glm::reflect_vec(&in_v, &normal)
}

// test assertion for comparing tuples with epsilon
#[macro_export]
macro_rules! assert_eq_eps {
    ($cond:expr, $expected:expr) => {
        assert_eq!(
            glm::vec4(true, true, true, true),
            glm::equal_eps(&$cond, &$expected, crate::math::EPSILON * 100.0),
            "left {} != right {}",
            $cond,
            $expected
        );
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{f_equals, EPSILON};
    pub(crate) use assert_eq_eps;
    use glm::equal_eps;

    #[test]
    fn tuple_constructor() {
        let t = tuple(4.3, -2.1, 0.0, 1.0);
        assert_eq!(t.x, 4.3);
        assert_eq!(t.y, -2.1);
        assert_eq!(t.z, 0.0);
        assert_eq!(t.w, 1.0);
    }

    #[test]
    fn tuple_with_w_1_is_point() {
        let p = tuple(0.0, 0.0, 0.0, 1.0);
        assert!(is_point(p));
        assert!(!is_vector(p));
    }

    #[test]
    fn tuple_with_w_0_is_vector() {
        let p = tuple(0.0, 0.0, 0.0, 0.0);
        assert!(!is_point(p));
        assert!(is_vector(p));
    }

    #[test]
    fn point_constructor_assigns_values() {
        let p = point(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
        assert_eq!(p.w, 1.0);
    }

    #[test]
    fn vector_constructor_assigns_values() {
        let p = vector(1.0, 2.0, 3.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
        assert_eq!(p.w, 0.0);
    }

    #[test]
    fn tuple_is_equal_when_equal() {
        let t1 = tuple(0.0, 1.0, 2.0, 0.0);
        let t2 = tuple(0.0, 1.0, 2.0, 0.0);
        assert!(t1 == t2);
    }

    #[test]
    fn tuple_is_equal_using_equal_epsilon_fn() {
        let t1 = tuple(0.0, 1.0, -2.0, 0.3);
        let t2 = tuple(EPSILON * 0.5, 1.0, -2.0, 0.3);
        assert!(t1 != t2);
        let teq = equal_eps(&t1, &t2, EPSILON);
        assert!(teq.x, "equal_eps: {}", teq);
    }

    #[test]
    fn assert_eq_eps_macro() {
        let t1 = tuple(0.0, 1.0, -2.0, 0.3);
        let t2 = tuple(EPSILON * 0.5, 1.0, -2.0, 0.3);
        assert_eq_eps!(t1, t2);
    }

    #[test]
    fn tuple_not_equal_when_diff() {
        let t1 = tuple(0.0, 1.1, 2.0, 0.0);
        let t2 = tuple(0.0, 1.0, 2.0, 0.0);
        assert!(t1 != t2);
    }

    #[test]
    fn add_tuples() {
        let t1 = tuple(0.0, 1.1, 2.0, 0.0);
        let t2 = tuple(0.0, 1.0, 2.0, 0.0);
        assert_eq!((t1 + t2), tuple(0.0, 2.1, 4.0, 0.0));
    }

    /*
    #[test]
    #[should_panic]
    fn add_points_is_illegal() {
        let t1 = tuple(0.0, 1.1, 2.0, 1.0);
        let t2 = tuple(0.0, 1.0, 2.0, 1.0);
        let _ = t1 + t2;
    }
    */

    #[test]
    fn subtract_2_points() {
        let p1 = point(1.0, 2.0, 3.0);
        let p2 = point(0.0, 1.0, 2.0);
        assert_eq!((p1 - p2), vector(1.0, 1.0, 1.0));
    }

    #[test]
    fn subtract_vector_from_point() {
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = vector(5.0, 6.0, 7.0);
        assert_eq!((p1 - p2), point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtract_2_vectors() {
        let p1 = vector(3.0, 2.0, 1.0);
        let p2 = vector(5.0, 6.0, 7.0);
        assert_eq!((p1 - p2), vector(-2.0, -4.0, -6.0));
    }

    /*
    #[test]
    #[should_panic]
    fn subtract_point_from_vector_is_illegal() {
        let p1 = vector(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let _ = p1 - p2;
    }
    */

    #[test]
    fn negation_operator_negates_tuple() {
        let t1 = tuple(-2.0, 1.1, 2.0, 1.0);
        let t2 = -t1;
        assert_eq!(t2, tuple(2.0, -1.1, -2.0, -1.0));
    }

    #[test]
    fn multiply_by_scalar() {
        let t1 = tuple(1.0, -2.0, 3.0, -4.0) * 0.5;
        assert_eq!(t1, tuple(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn divide_by_scalar() {
        let t1 = tuple(1.0, -2.0, 3.0, -4.0) / 2.0;
        assert_eq!(t1, tuple(0.5, -1.0, 1.5, -2.0));
    }

    /*
    #[test]
    #[should_panic]
    fn divide_by_zero_should_panic() {
        let _ = tuple(1.0, -2.0, 3.0, -4.0) / 0.0;
    }
    */

    #[test]
    fn magnitude_unit() {
        let t1 = vector(1.0, 0.0, 0.0);
        assert_eq!(t1.magnitude(), 1.0);
    }

    #[test]
    fn magnitude() {
        let t1 = vector(1.0, 2.0, 3.0);
        assert_eq!(t1.magnitude(), (14.0_f32).sqrt());
    }

    #[test]
    fn magnitude_zero_vector() {
        let t1 = vector(0.0, 0.0, 0.0);
        assert_eq!(t1.magnitude(), 0.0);
    }

    #[test]
    fn normalize() {
        let t1 = vector(4.0, 0.0, 0.0);
        assert_eq!(t1.normalize(), vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalize_2() {
        let t1 = vector(1.0, 2.0, 3.0);
        let sqrt14 = (14_f32).sqrt();
        assert_eq!(
            t1.normalize(),
            vector(1.0 / sqrt14, 2.0 / sqrt14, 3.0 / sqrt14)
        );
    }

    /*
    #[test]
    #[should_panic]
    fn normalize_zero_vector_should_panic() {
        let t1 = vector(0.0, 0.0, 0.0);
        let _ = t1.normalize();
    }
    */

    #[test]
    fn magnitude_of_normalized_vector() {
        let t1 = vector(1.0, 2.0, 3.0).normalize();
        assert!(f_equals(t1.magnitude(), 1.0));
    }

    #[test]
    fn dot_product() {
        let t1 = vector(1.0, 2.0, 3.0);
        let t2 = vector(2.0, 3.0, 4.0);
        assert_eq!(t1.dot(&t2), 20.0);
    }

    #[test]
    fn cross_product() {
        let t1 = vector(1.0, 2.0, 3.0).xyz();
        let t2 = vector(2.0, 3.0, 4.0).xyz();
        assert_eq!(t1.cross(&t2), vector(-1.0, 2.0, -1.0).xyz());
        assert_eq!(t2.cross(&t1), vector(1.0, -2.0, 1.0).xyz());
    }

    #[test]
    fn reflect_vector_approaching_45() {
        let v = vector(1.0, -1.0, 0.0);
        let n = vector_y();
        let r = reflect(v, n);
        assert_eq!(r, vector(1.0, 1.0, 0.0));
    }
}
