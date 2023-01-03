use crate::math::F3D;
use crate::matrix::Matrix4;
use crate::tuple::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: F3D) -> Tuple {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: Matrix4) -> Ray {
        Ray::new(m * self.origin, m * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::transformation::*;

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

    #[test]
    fn translating_a_ray() {
        let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = make_translation(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(point(1.0, 2.0, 3.0), vector(0.0, 1.0, 0.0));
        let m = make_scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(m);
        assert_eq!(r2.origin, point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, vector(0.0, 3.0, 0.0));
    }
}
