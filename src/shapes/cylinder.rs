use crate::bounds::*;
use crate::math;
use crate::object::*;
use crate::ray::Ray;
use crate::shapes::shape::*;
use crate::tuple::*;
use std::mem;

#[derive(Clone, Debug, PartialEq)]
pub struct Cylinder {
    pub minimum: math::F3D,
    pub maximum: math::F3D,
    pub closed: bool,
}

// helper for intersect_caps
pub fn check_cap(ray: &Ray, t: math::F3D) -> bool {
    let v = ray.origin + t * ray.direction;
    (v.x.powi(2) + v.z.powi(2)) <= 1.0
}

// constructor utilities
pub fn cylinder_with_id(
    id: Option<String>,
    minimum: math::F3D,
    maximum: math::F3D,
    closed: bool,
) -> Object {
    Object::new(id).with_shape(Shape::Cylinder(Cylinder {
        minimum,
        maximum,
        closed,
    }))
}

pub fn cylinder(min: math::F3D, max: math::F3D, closed: bool) -> Object {
    cylinder_with_id(None, min, max, closed)
}

pub fn default_cylinder() -> Object {
    cylinder(-math::INFINITY, math::INFINITY, false)
}

impl Cylinder {
    pub fn set_bounds(&mut self, min: math::F3D, max: math::F3D) {
        self.minimum = min;
        self.maximum = max;
    }

    fn intersect_caps(&self, ray: &Ray) -> Vec<math::F3D> {
        let mut xs = vec![];

        if !self.closed || math::f_equals(ray.direction.y, 0.0) {
            return xs;
        }
        let t = (self.minimum - ray.origin.y) / ray.direction.y;

        if check_cap(ray, t) {
            xs.push(t);
        }
        let t = (self.maximum - ray.origin.y) / ray.direction.y;

        if check_cap(ray, t) {
            xs.push(t);
        }
        xs
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<math::F3D> {
        let mut xs = vec![];
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        if math::f_equals(a, 0.0) {
            return self.intersect_caps(ray);
        }
        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;
        let disc = b * b - 4.0 * a * c;

        if disc >= 0.0 {
            let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
            let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

            if t0 > t1 {
                mem::swap(&mut t0, &mut t1);
            }
            let y0 = ray.origin.y + t0 * ray.direction.y;
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(t0);
            }
            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(t1);
            }
            xs.extend(self.intersect_caps(ray));
        }
        xs
    }

    pub fn local_normal_at(&self, point: &Point) -> Vector {
        let dist = point.x.powi(2) + point.z.powi(2);
        if dist < 1.0 && point.y >= (self.maximum - math::EPSILON) {
            vector_y()
        } else if dist < 1.0 && point.y <= (self.minimum + math::EPSILON) {
            vector(0.0, -1.0, 0.0)
        } else {
            vector(point.x, 0.0, point.z)
        }
    }

    pub fn bounds(&self) -> Bounds {
        Bounds {
            min: point(-1.0, self.minimum, -1.0),
            max: point(1.0, self.maximum, 1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_feps;

    #[test]
    fn ray_misses_cylinder() {
        let c = default_cylinder();
        let tests = vec![
            (point_x(), vector_y()),
            (point_zero(), vector_y()),
            (point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0)),
        ];
        for t in tests {
            let direction = t.1.normalize();
            let r = Ray::new(t.0, direction);
            let xs = c.intersect(&r);
            assert!(xs.is_empty());
        }
    }

    #[test]
    fn ray_strikes_cylinder() {
        let c = default_cylinder();
        for t in vec![
            (point(1.0, 0.0, -5.0), vector_z(), 5.0, 5.0),
            (point(0.0, 0.0, -5.0), vector_z(), 4.0, 6.0),
            (
                point(0.5, 0.0, -5.0),
                vector(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ] {
            let dir = t.1.normalize();
            let r = Ray::new(t.0, dir);
            let xs = c.intersect(&r);
            assert_eq!(xs.len(), 2);
            assert_eq_feps!(xs[0].t, t.2);
            assert_eq_feps!(xs[1].t, t.3);
        }
    }

    #[test]
    fn normal_vector() {
        let c = default_cylinder();
        for t in vec![
            (point_x(), vector_x()),
            (point(0.0, 5.0, -1.0), vector(0.0, 0.0, -1.0)),
            (point(0.0, -2.0, 1.0), vector_z()),
            (point(-1.0, 1.0, 0.0), vector_x() * -1.0),
        ] {
            let n = c.normal_at(t.0);
            assert_eq!(n, t.1);
        }
    }

    #[test]
    fn default_y_values() {
        if let Shape::Cylinder(c) = default_cylinder().shape {
            assert_eq!(c.minimum, -math::INFINITY);
            assert_eq!(c.maximum, math::INFINITY);
        } else {
            panic!("no cylinder shape");
        }
    }

    #[test]
    fn intersecting_constrained_cyclinder() {
        let c = cylinder(1.0, 2.0, false);
        let tests = vec![
            (point(0.0, 1.5, 0.0), vector(0.1, 1.0, 0.0), 0),
            (point(0.0, 3.0, -5.0), vector_z(), 0),
            (point(0.0, 0.0, -5.0), vector_z(), 0),
            (point(0.0, 2.0, -5.0), vector_z(), 0),
            (point(0.0, 1.5, -2.0), vector_z(), 2),
        ];
        for t in tests {
            let r = Ray::new(t.0, t.1.normalize());
            let xs = c.intersect(&r);
            assert_eq!(xs.len(), t.2);
        }
    }

    #[test]
    fn default_closed_value() {
        if let Shape::Cylinder(c) = default_cylinder().shape {
            assert!(!c.closed);
        }
    }

    #[test]
    fn intersecting_caps_of_closed_cylinder() {
        let c = cylinder(1.0, 2.0, true);
        let tests = vec![
            (point(0.0, 3.0, 0.0), vector(0.0, -1.0, 0.0), 2),
            (point(0.0, 3.0, -2.0), vector(0.0, -1.0, 2.0), 2),
            (point(0.0, 4.0, -2.0), vector(0.0, -1.0, 1.0), 2),
            (point(0.0, -1.0, -2.0), vector(0.0, 1.0, 1.0), 2),
        ];
        for t in tests {
            let r = Ray::new(t.0, t.1.normalize());
            let xs = c.intersect(&r);
            assert_eq!(xs.len(), t.2);
        }
    }

    #[test]
    fn normal_at_end_caps() {
        let c = cylinder(1.0, 2.0, true);
        for t in vec![
            (point_y(), vector(0.0, -1.0, 0.0)),
            (point(0.5, 1.0, 0.0), vector(0.0, -1.0, 0.0)),
            (point(0.0, 2.0, 0.0), vector_y()),
            (point(0.0, 2.0, 0.5), vector_y()),
        ] {
            let n = c.normal_at(t.0);
            assert_eq!(n, t.1);
        }
    }
}
