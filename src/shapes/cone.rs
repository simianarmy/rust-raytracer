use crate::bounds::*;
use crate::intersection::Intersection;
use crate::math;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::shape::*;
use crate::tuple::*;
use std::mem;

#[derive(Clone, Debug, PartialEq)]
pub struct Cone {
    pub minimum: math::F3D,
    pub maximum: math::F3D,
    pub closed: bool,
}

impl Cone {
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

        if check_cap(ray, t, self.minimum) {
            xs.push(t);
        }
        let t = (self.maximum - ray.origin.y) / ray.direction.y;

        if check_cap(ray, t, self.maximum) {
            xs.push(t);
        }
        xs
    }
}

// helper for intersect_caps
pub fn check_cap(ray: &Ray, t: math::F3D, y: math::F3D) -> bool {
    let v = ray.origin + t * ray.direction;
    (v.x.powi(2) + v.z.powi(2)) <= y.abs()
}

// constructor utilities
pub fn cone_with_id(id: Option<String>, min: math::F3D, max: math::F3D, closed: bool) -> Object {
    let mut o = Object::new(id);
    o.shape = Shape::Cone(Cone {
        minimum: min,
        maximum: max,
        closed,
    });
    o
}

pub fn cone(min: math::F3D, max: math::F3D, closed: bool) -> Object {
    cone_with_id(None, min, max, closed)
}

pub fn default_cone() -> Object {
    cone(-math::INFINITY, math::INFINITY, false)
}

impl Cone {
    pub fn local_intersect(&self, ray: &Ray) -> Vec<math::F3D> {
        let mut xs = vec![];
        let ro = ray.origin;
        let rd = ray.direction;
        let a = rd.x.powi(2) - rd.y.powi(2) + rd.z.powi(2);
        let c = ro.x.powi(2) - ro.y.powi(2) + ro.z.powi(2);
        let b = 2.0 * ro.x * rd.x - 2.0 * ro.y * rd.y + 2.0 * ro.z * rd.z;

        if math::f_equals(a, 0.0) {
            if math::f_equals(b, 0.0) {
                return vec![];
            }
            let t = -c / (2.0 * b);
            xs.push(t);
        }
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

    pub fn local_normal_at(&self, point: Point) -> Vector {
        let dist = point.x.powi(2) + point.z.powi(2);
        if dist < 1.0 && point.y >= self.maximum - math::EPSILON {
            vector_y()
        } else if dist < 1.0 && point.y <= self.minimum + math::EPSILON {
            vector(0.0, -1.0, 0.0)
        } else {
            let y = (point.x.powi(2) + point.z.powi(2)).sqrt();
            if point.y > 0.0 {
                vector(point.x, -y, point.z)
            } else {
                vector(point.x, y, point.z)
            }
        }
    }

    pub fn bounds(&self) -> Bounds {
        if self.minimum == -math::INFINITY && self.maximum == math::INFINITY {
            Bounds {
                min: point(-math::INFINITY, -math::INFINITY, -math::INFINITY),
                max: point(math::INFINITY, math::INFINITY, math::INFINITY),
            }
        } else {
            let a = self.minimum.abs();
            let b = self.maximum.abs();
            let limit = a.max(b);

            Bounds {
                min: point(-limit, self.minimum, -limit),
                max: point(limit, self.maximum, limit),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_feps;
    use std::f64::consts::SQRT_2;

    #[test]
    fn ray_strikes_cone() {
        let c = default_cone();
        for t in vec![
            (point(0.0, 0.0, -5.0), vector_z(), 5.0, 5.0),
            (
                point(0.0, 0.0, -5.0),
                vector(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                point(1.0, 1.0, -5.0),
                vector(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ] {
            let dir = t.1.normalize();
            let r = Ray::new(t.0, dir);
            let xs = match c.shape {
                Shape::Cone(c) => c.local_intersect(&r),
                _ => vec![],
            };
            assert_eq!(xs.len(), 2);
            assert_eq_feps!(xs[0], t.2);
            assert_eq_feps!(xs[1], t.3);
        }
    }

    #[test]
    fn intersecting_with_ray_parallel_to_a_half() {
        let c = default_cone();
        let r = Ray::new(point(0.0, 0.0, -1.0), vector(0.0, 1.0, 1.0).normalize());
        let xs = match c.shape {
            Shape::Cone(c) => c.local_intersect(&r),
            _ => vec![],
        };
        assert_eq!(xs.len(), 1);
        assert_eq_feps!(xs[0], 0.35355);
    }

    #[test]
    fn intersecting_caps_of_closed_cone() {
        let mut c = cone(-0.5, 0.5, true);
        let tests = vec![
            (point(0.0, 0.0, -5.0), vector_y(), 0),
            (point(0.0, 0.0, -0.25), vector(0.0, 1.0, 1.0), 2),
            (point(0.0, 0.0, -0.25), vector_y(), 4),
        ];
        for t in tests {
            println!("test = {:?}", t);
            let r = Ray::new(t.0, (t.1).normalize());
            let xs = match c.shape {
                Shape::Cone(c) => c.local_intersect(&r),
                _ => vec![],
            };
            assert_eq!(xs.len(), t.2);
        }
    }

    #[test]
    fn normal_at() {
        let c = default_cone();
        for t in vec![
            (point_zero(), vector_zero()),
            (point_unit(), vector(1.0, -SQRT_2, 1.0)),
            (point(-1.0, -1.0, 0.0), vector(-1.0, 1.0, 0.0)),
        ] {
            println!("test: {:?}", t);
            let n = match c.shape {
                Shape::Cone(c) => c.local_normal_at(t.0),
                _ => vector_zero(),
            };
            assert_eq!(n, t.1);
        }
    }
}
