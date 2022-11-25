use crate::intersection::Intersection;
use crate::intersections;
use crate::materials::Material;
use crate::math;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Cylinder {
    pub props: Shape3D,
}

// constructor utilities
pub fn cylinder_with_id(id: Option<String>) -> Cylinder {
    Cylinder {
        props: Shape3D::new(id),
    }
}

pub fn cylinder() -> Cylinder {
    cylinder_with_id(None)
}

fn check_axis(origin: math::F3D, direction: math::F3D) -> (math::F3D, math::F3D) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let mut tmin = 0.0;
    let mut tmax = 0.0;

    if direction.abs() >= math::EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * math::INFINITY;
        tmax = tmax_numerator * math::INFINITY;
    }
    if tmin > tmax {
        let tmp = tmin;
        tmin = tmax;
        tmax = tmp;
    }
    (tmin, tmax)
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
        let a = ray.direction.x.powi(2) + ray.direction.z.powi(2);

        if math::f_equals(a, 0.0) {
            return vec![];
        }
        let b = 2.0 * ray.origin.x * ray.direction.x + 2.0 * ray.origin.z * ray.direction.z;
        let c = ray.origin.x.powi(2) + ray.origin.z.powi(2) - 1.0;
        let disc = b * b - 4.0 * a * c;

        if disc < 0.0 {
            vec![]
        } else {
            intersections!(Intersection {
                t: 1.0,
                object: Box::new(self.clone())
            })
        }
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        match point.abs().max() {
            x if x == point.x.abs() => vector(point.x, 0.0, 0.0),
            y if y == point.y.abs() => vector(0.0, point.y, 0.0),
            _ => vector(0.0, 0.0, point.z),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
