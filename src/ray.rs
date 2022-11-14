use crate::tuple::Tuple;

use crate::math::F3D;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: F3D) -> Tuple {
        self.origin + self.direction * t
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::{point, vector};

    #[test]
    fn constructor_assigns_args() {
        let r = Ray::new(point(1.0, 0.0, 0.0), vector(-1.0, 1.0, 0.0));
        assert_eq!(r.origin, point(1.0, 0.0, 0.0));
        assert_eq!(r.direction, vector(-1.0, 1.0, 0.0));
    }

    #[test]
    fn computing_point_from_distance() {
        let r = Ray::new(point(2.0, 3.0, 4.0), vector(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), point(4.5, 3.0, 4.0));
    }
}
