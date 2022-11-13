/**
 * Low level math routines
 */

pub type F3D = f32;
const EPSILON: F3D = f32::EPSILON;

pub fn f_equals(a: F3D, b: F3D) -> bool {
    if (a - b).abs() <= EPSILON {
        true
    } else {
        false
    }
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
