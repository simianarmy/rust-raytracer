/**
 * Low level math routines
 */
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
            f_equals($cond, $expected),
            "left {} != right {}",
            $cond,
            $expected
        );
    };
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
}
