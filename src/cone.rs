use crate::bounds::*;
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
pub struct Cone {
    pub props: Shape3D,
    pub minimum: math::F3D,
    pub maximum: math::F3D,
    pub closed: bool,
}

impl Cone {
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

        if check_cap(ray, t, self.minimum) {
            xs.push(Intersection::new(Box::new(self.clone()), t));
        }
        let t = (self.maximum - ray.origin.y) / ray.direction.y;

        if check_cap(ray, t, self.maximum) {
            xs.push(Intersection::new(Box::new(self.clone()), t));
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
pub fn cone_with_id(id: Option<String>) -> Cone {
    Cone {
        props: Shape3D::new(id),
        minimum: -math::INFINITY,
        maximum: math::INFINITY,
        closed: false,
    }
}

pub fn cone() -> Cone {
    cone_with_id(None)
}

impl Shape for Cone {
    fn get_id(&self) -> String {
        format!("cone_{}", self.props.id)
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
            xs.push(Intersection::new(Box::new(self.clone()), t));
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
                xs.push(Intersection::new(Box::new(self.clone()), t0));
            }
            let y1 = ray.origin.y + t1 * ray.direction.y;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection::new(Box::new(self.clone()), t1));
            }
            xs.extend(self.intersect_caps(ray));
        }
        xs
    }

    fn local_normal_at(&self, point: Point) -> Vector {
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

    fn bounds(&self) -> Bounds {
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
    use std::f64::consts::SQRT_2;

    #[test]
    fn ray_strikes_cone() {
        let c = cone();
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
            let xs = c.local_intersect(&r);
            assert_eq!(xs.len(), 2);
            assert_eq_feps!(xs[0].t, t.2);
            assert_eq_feps!(xs[1].t, t.3);
        }
    }

    #[test]
    fn intersecting_with_ray_parallel_to_a_half() {
        let c = cone();
        let r = Ray::new(point(0.0, 0.0, -1.0), vector(0.0, 1.0, 1.0).normalize());
        let xs = c.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq_feps!(xs[0].t, 0.35355);
    }

    #[test]
    fn intersecting_caps_of_closed_cone() {
        let mut c = cone();
        c.set_bounds(-0.5, 0.5);
        c.closed = true;
        let tests = vec![
            (point(0.0, 0.0, -5.0), vector_y(), 0),
            (point(0.0, 0.0, -0.25), vector(0.0, 1.0, 1.0), 2),
            (point(0.0, 0.0, -0.25), vector_y(), 4),
        ];
        for t in tests {
            println!("test = {:?}", t);
            let r = Ray::new(t.0, (t.1).normalize());
            let xs = c.local_intersect(&r);
            assert_eq!(xs.len(), t.2);
        }
    }

    #[test]
    fn normal_at() {
        let c = cone();
        for t in vec![
            (point_zero(), vector_zero()),
            (point_unit(), vector(1.0, -SQRT_2, 1.0)),
            (point(-1.0, -1.0, 0.0), vector(-1.0, 1.0, 0.0)),
        ] {
            println!("test: {:?}", t);
            let n = c.local_normal_at(t.0);
            assert_eq!(n, t.1);
        }
    }
}
