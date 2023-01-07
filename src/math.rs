/**
 * Low level math routines
 */
use crate::tuple::*;
use std::f64::consts::SQRT_2;

pub type F3D = f64;
// Use suggested raytracer book value to pass unit tests
pub const EPSILON: F3D = 0.0001; // f64::EPSILON; //  * 100.0;
pub const SQRT_2_DIV_2: F3D = SQRT_2 / 2.0;
pub const INFINITY: F3D = std::f64::INFINITY;

pub fn f_equals(a: F3D, b: F3D) -> bool {
    (a - b).abs() <= EPSILON
}

// test assertion for comparing floats with epsilon
#[macro_export]
macro_rules! assert_eq_feps {
    ($cond:expr, $expected:expr) => {
        assert!(
            crate::math::f_equals($cond, $expected),
            "left {} != right {}",
            $cond,
            $expected
        );
    };
}

// map 3d point to 2d
pub fn spherical_map(p: &Point) -> (F3D, F3D) {
    let theta = p.x.atan2(p.z);
    let vec = vector(p.x, p.y, p.z);
    let radius = vec.magnitude();
    let phi = (p.y / radius).acos();
    let raw_u = theta / (glm::pi::<F3D>() * 2.0);
    let u = 1.0 - (raw_u + 0.5);
    let v = 1.0 - phi / glm::pi::<F3D>();

    (u, v)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn f_equals_with_same_number() {
        assert!(f_equals(1.0, 1.0))
    }

    #[test]
    fn f_equals_with_different_numbers() {
        assert!(!f_equals(1.0, 2.0))
    }

    #[test]
    fn f_equals_with_tiny_difference() {
        let a = 1.0;
        let b = 0.99999994;
        assert!(f_equals(a, b), "{} and {} not equal", a, b);
    }

    #[test]
    fn spherical_mapping_on_3d_point() {
        for c in [
            (point(0.0, 0.0, -1.0), 0.0, 0.5),
            (point_x(), 0.25, 0.5),
            (point_z(), 0.5, 0.5),
            (point_x() * -1.0, 0.75, 0.5),
            (point_y(), 0.5, 1.0),
            (point(SQRT_2_DIV_2, SQRT_2_DIV_2, 0.0), 0.25, 0.75),
        ] {
            let (u, v) = spherical_map(&c.0);
            assert_eq!(u, c.1);
            assert_eq!(v, c.2);
        }
    }
}
