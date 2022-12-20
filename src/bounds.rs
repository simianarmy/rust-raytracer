/**
 * Bounding box / groups
 */
use crate::math;
use crate::matrix::*;
use crate::tuple::*;

#[derive(Clone, Debug)]
pub struct Bounds {
    pub min: Point,
    pub max: Point,
}

impl Bounds {
    pub fn new(min: Point, max: Point) -> Bounds {
        Bounds { min, max }
    }

    pub fn add_point(&mut self, p: &Point) {
        self.min.x = self.min.x.min(p.x);
        self.min.y = self.min.y.min(p.y);
        self.min.z = self.min.z.min(p.z);
        self.max.x = self.max.x.max(p.x);
        self.max.y = self.max.y.max(p.y);
        self.max.z = self.max.z.max(p.z);
    }

    pub fn add_bounds(&mut self, b: &Self) {
        if b.min.x < self.min.x {
            self.min.x = b.min.x;
        }
        if b.min.y < self.min.y {
            self.min.y = b.min.y;
        }
        if b.min.z < self.min.z {
            self.min.z = b.min.z;
        }
        if b.max.x > self.max.x {
            self.max.x = b.max.x;
        }
        if b.max.y > self.max.y {
            self.max.y = b.max.y;
        }
        if b.max.z > self.max.z {
            self.max.z = b.max.z;
        }
    }

    pub fn contains_point(&self, p: &Point) -> bool {
        self.min.x <= p.x
            && p.x <= self.max.x
            && self.min.y <= p.y
            && p.y <= self.max.y
            && self.min.z <= p.z
            && p.z <= self.max.z
    }

    pub fn contains_bounds(&self, b: &Self) -> bool {
        self.contains_point(&b.min) && self.contains_point(&b.max)
    }

    pub fn transform(&self, m: &Matrix4) -> Self {
        // get all 8 corners of our bounding box
        let p1 = self.min;
        let p2 = point(self.min.x, self.min.y, self.max.z);
        let p3 = point(self.min.x, self.max.y, self.min.z);
        let p4 = point(self.min.x, self.max.y, self.max.z);
        let p5 = point(self.max.x, self.min.y, self.min.z);
        let p6 = point(self.max.x, self.min.y, self.max.z);
        let p7 = point(self.max.x, self.max.y, self.min.z);
        let p8 = self.max;

        let mut bb = Bounds::default();

        for p in [p1, p2, p3, p4, p5, p6, p7, p8] {
            bb.add_point(&(m * p));
        }
        bb
    }
}

impl Default for Bounds {
    fn default() -> Self {
        Bounds {
            max: point(-math::INFINITY, -math::INFINITY, -math::INFINITY),
            min: point(math::INFINITY, math::INFINITY, math::INFINITY),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math;
    use crate::shape::test_shape;
    use crate::sphere::*;
    use crate::transformation::*;

    #[test]
    fn default_bounds_infinite() {
        let b = Bounds::default();
        assert_eq!(b.min, point(math::INFINITY, math::INFINITY, math::INFINITY));
        assert_eq!(
            b.max,
            point(-math::INFINITY, -math::INFINITY, -math::INFINITY)
        );
    }

    #[test]
    fn creating_bounding_box_with_volume() {
        let b = Bounds::new(point(-1.0, -2.0, -3.0), point(1.0, 2.0, 3.0));
        assert_eq!(b.min, point(-1.0, -2.0, -3.0));
        assert_eq!(b.max, point(1.0, 2.0, 3.0));
    }

    #[test]
    fn adding_points_to_empty_box() {
        let mut b = Bounds::default();
        let p1 = point(-5.0, 2.0, 0.0);
        let p2 = point(7.0, 0.0, -3.0);
        b.add_point(&p1);
        b.add_point(&p2);
        assert_eq!(b.min, point(-5.0, 0.0, -3.0));
        assert_eq!(b.max, point(7.0, 2.0, 0.0));
    }

    #[test]
    fn adding_box() {
        let mut a = Bounds::new(point(-5.0, -2.0, 0.0), point(7.0, 4.0, 4.0));
        let b = Bounds::new(point(8.0, -7.0, -2.0), point(14.0, 2.0, 8.0));
        a.add_bounds(&b);
        assert_eq!(a.min, point(-5.0, -7.0, -2.0));
        assert_eq!(a.max, point(14.0, 4.0, 8.0));
    }

    #[test]
    fn box_contains_point() {
        let a = Bounds::new(point(5.0, -2.0, 0.0), point(11.0, 4.0, 7.0));
        for c in [
            (point(5.0, -2.0, 0.0), true),
            (point(11.0, 4.0, 7.0), true),
            (point(8.0, 1.0, 3.0), true),
            (point(3.0, 0.0, 3.0), false),
            (point(8.0, -4.0, 3.0), false),
            (point(8.0, 1.0, -1.0), false),
            (point(8.0, 1.0, 8.0), false),
        ] {
            assert_eq!(a.contains_point(&c.0), c.1);
        }
    }

    #[test]
    fn box_contains_box() {
        let a = Bounds::new(point(5.0, -2.0, 0.0), point(11.0, 4.0, 7.0));
        for c in [
            (
                Bounds::new(point(5.0, -2.0, 0.0), point(11.0, 4.0, 7.0)),
                true,
            ),
            (
                Bounds::new(point(6.0, -1.0, 1.0), point(10.0, 3.0, 6.0)),
                true,
            ),
            (
                Bounds::new(point(4.0, -3.0, -1.0), point(10.0, 3.0, 6.0)),
                false,
            ),
        ] {
            assert_eq!(a.contains_bounds(&c.0), c.1, "{:?}", c);
        }
    }

    #[test]
    fn transforming_bounds() {
        let b = Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0));
        let m = make_rotation_x(glm::quarter_pi()) * make_rotation_y(glm::quarter_pi());
        let bb = b.transform(&m);
        assert_eq_eps!(bb.min, point(-1.4142, -1.7071, -1.7071));
        assert_eq_eps!(bb.max, point(1.4142, 1.7071, 1.7071));
    }
}