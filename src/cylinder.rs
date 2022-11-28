use crate::group::Group;
use crate::intersection::Intersection;
use crate::materials::Material;
use crate::math;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;
use std::mem;

#[derive(Clone, Debug, PartialEq)]
pub struct Cylinder {
    pub props: Shape3D,
    pub minimum: math::F3D,
    pub maximum: math::F3D,
    pub closed: bool,
}

impl Cylinder {
    pub fn set_bounds(&mut self, min: math::F3D, max: math::F3D) {
        self.minimum = min;
        self.maximum = max;
    }

    fn intersect_caps(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs = vec![];

        if !self.closed || math::f_equals(ray.direction.y, 0.0) {
            return xs;
        }
        let t = (self.minimum - ray.origin.y) / ray.direction.y;

        if check_cap(ray, t) {
            xs.push(Intersection {
                t,
                object: Box::new(self.clone()),
            });
        }
        let t = (self.maximum - ray.origin.y) / ray.direction.y;

        if check_cap(ray, t) {
            xs.push(Intersection {
                t,
                object: Box::new(self.clone()),
            });
        }
        xs
    }
}

// helper for intersect_caps
pub fn check_cap(ray: &Ray, t: math::F3D) -> bool {
    let v = ray.origin + t * ray.direction;
    (v.x.powi(2) + v.z.powi(2)) <= 1.0
}

// constructor utilities
pub fn cylinder_with_id(id: Option<String>) -> Cylinder {
    Cylinder {
        props: Shape3D::new(id),
        minimum: -math::INFINITY,
        maximum: math::INFINITY,
        closed: false,
    }
}

pub fn cylinder() -> Cylinder {
    cylinder_with_id(None)
}

impl Shape for Cylinder {
    fn get_id(&self) -> String {
        format!("cylinder_{}", self.props.id)
    }
    fn get_transform(&self) -> &Matrix4 {
        &self.props.transform
    }
    fn set_transform(&mut self, t: &Matrix4) {
        self.props.transform = *t;
    }
    fn get_material(&self) -> &Material {
        &self.props.material
    }
    fn set_material(&mut self, m: Material) {
        self.props.material = m;
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
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
                xs.push(Intersection {
                    t: t0,
                    object: Box::new(self.clone()),
                });
            }
            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection {
                    t: t1,
                    object: Box::new(self.clone()),
                });
            }
            xs.extend(self.intersect_caps(ray));
        }
        xs
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        let dist = point.x.powi(2) + point.z.powi(2);
        if dist < 1.0 && point.y >= self.maximum - math::EPSILON {
            vector_y()
        } else if dist < 1.0 && point.y <= self.maximum + math::EPSILON {
            vector(0.0, -1.0, 0.0)
        } else {
            vector(point.x, 0.0, point.z)
        }
    }

    fn get_parent(&self) -> Option<Box<Group>> {
        self.props.parent
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_feps;

    #[test]
    fn ray_misses_cylinder() {
        let c = cylinder();
        let tests = vec![
            (point_x(), vector_y()),
            (point_zero(), vector_y()),
            (point(0.0, 0.0, -5.0), vector(1.0, 1.0, 1.0)),
        ];
        for t in tests {
            let direction = t.1.normalize();
            let r = Ray::new(t.0, direction);
            let xs = c.local_intersect(&r);
            assert!(xs.is_empty());
        }
    }

    #[test]
    fn ray_strikes_cylinder() {
        let c = cylinder();
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
            let xs = c.local_intersect(&r);
            assert_eq!(xs.len(), 2);
            assert_eq_feps!(xs[0].t, t.2);
            assert_eq_feps!(xs[1].t, t.3);
        }
    }

    #[test]
    fn normal_vector() {
        let c = cylinder();
        for t in vec![
            (point_x(), vector_x()),
            (point(0.0, 5.0, -1.0), vector(0.0, 0.0, -1.0)),
            (point(0.0, -2.0, 1.0), vector_z()),
            (point(-1.0, 1.0, 0.0), vector_x() * -1.0),
        ] {
            let n = c.local_normal_at(t.0);
            assert_eq!(n, t.1);
        }
    }

    #[test]
    fn default_y_values() {
        let c = cylinder();
        assert_eq!(c.minimum, -math::INFINITY);
        assert_eq!(c.maximum, math::INFINITY);
    }

    #[test]
    fn intersecting_constrained_cyclinder() {
        let mut c = cylinder();
        c.set_bounds(1.0, 2.0);
        let tests = vec![
            (point(0.0, 1.5, 0.0), vector(0.1, 1.0, 0.0), 0),
            (point(0.0, 3.0, -5.0), vector_z(), 0),
            (point(0.0, 0.0, -5.0), vector_z(), 0),
            (point(0.0, 2.0, -5.0), vector_z(), 0),
            (point(0.0, 1.5, -2.0), vector_z(), 2),
        ];
        for t in tests {
            let r = Ray::new(t.0, t.1.normalize());
            let xs = c.local_intersect(&r);
            assert_eq!(xs.len(), t.2);
        }
    }

    #[test]
    fn default_closed_value() {
        assert!(!cylinder().closed);
    }

    #[test]
    fn intersecting_caps_of_closed_cylinder() {
        let mut c = cylinder();
        c.set_bounds(1.0, 2.0);
        c.closed = true;
        let tests = vec![
            (point(0.0, 3.0, 0.0), vector(0.0, -1.0, 0.0), 2),
            (point(0.0, 3.0, -2.0), vector(0.0, -1.0, 2.0), 2),
            (point(0.0, 4.0, -2.0), vector(0.0, -1.0, 1.0), 2),
            (point(0.0, -1.0, -2.0), vector(0.0, 1.0, 1.0), 2),
        ];
        for t in tests {
            let r = Ray::new(t.0, t.1.normalize());
            let xs = c.local_intersect(&r);
            assert_eq!(xs.len(), t.2);
        }
    }

    #[test]
    fn normal_at_end_caps() {
        let mut c = cylinder();
        c.set_bounds(1.0, 2.0);
        c.closed = true;
        for t in vec![
            (point_y(), vector(0.0, -1.0, 0.0)),
            (point(0.5, 1.0, 0.0), vector(0.0, -1.0, 0.0)),
            (point(0.0, 2.0, 0.0), vector_y()),
            (point(0.0, 2.0, 0.5), vector_y()),
        ] {
            let n = c.local_normal_at(t.0);
            assert_eq!(n, t.1);
        }
    }
}
