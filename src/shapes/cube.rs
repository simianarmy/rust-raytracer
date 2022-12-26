use crate::bounds::*;
use crate::intersection::*;
use crate::math;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::shape::*;
use crate::tuple::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Cube {}

// constructor utilities
pub fn cube_with_id(id: Option<String>) -> Object {
    let mut o = Object::new(id);
    o.shape = Shape::Cube();
    o
}

pub fn cube() -> Object {
    cube_with_id(None)
}

impl Cube {
    pub fn check_axis(
        origin: math::F3D,
        direction: math::F3D,
        min: math::F3D,
        max: math::F3D,
    ) -> (math::F3D, math::F3D) {
        let tmin_numerator = min - origin;
        let tmax_numerator = max - origin;

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

    pub fn local_normal_at(point: Point) -> Vector {
        match point.abs().max() {
            x if x == point.x.abs() => vector(point.x, 0.0, 0.0),
            y if y == point.y.abs() => vector(0.0, point.y, 0.0),
            _ => vector(0.0, 0.0, point.z),
        }
    }

    pub fn local_intersect(ray: &Ray) -> Vec<math::F3D> {
        let (xtmin, xtmax) = Cube::check_axis(ray.origin.x, ray.direction.x, -1.0, 1.0);
        let (ytmin, ytmax) = Cube::check_axis(ray.origin.y, ray.direction.y, -1.0, 1.0);
        let (ztmin, ztmax) = Cube::check_axis(ray.origin.z, ray.direction.z, -1.0, 1.0);

        let tmin = glm::max3_scalar(xtmin, ytmin, ztmin);
        let tmax = glm::min3_scalar(xtmax, ytmax, ztmax);

        if tmin > tmax {
            vec![]
        } else {
            vec![tmin, tmax]
        }
    }

    pub fn bounds() -> Bounds {
        Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_intersects_cube() {
        let c = cube();
        let tests = vec![
            (point(5.0, 0.5, 0.0), vector(-1.0, 0.0, 0.0), 4.0, 6.0),
            (point(-5.0, 0.5, 0.0), vector(1.0, 0.0, 0.0), 4.0, 6.0),
            (point(0.5, 5.0, 0.0), vector(0.0, -1.0, 0.0), 4.0, 6.0),
            (point(0.5, -5.0, 0.0), vector(0.0, 1.0, 0.0), 4.0, 6.0),
            (point(0.5, 0.0, 5.0), vector(0.0, 0.0, -1.0), 4.0, 6.0),
            (point(0.5, 0.0, -5.0), vector(0.0, 0.0, 1.0), 4.0, 6.0),
            (point(0.0, 0.5, 0.0), vector(0.0, 0.0, 1.0), -1.0, 1.0),
        ];
        for t in tests {
            let r = Ray::new(t.0, t.1);
            let xs = Cube::local_intersect(&r);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0], t.2);
            assert_eq!(xs[1], t.3);
        }
    }

    #[test]
    fn ray_misses_cube() {
        let c = cube();
        let tests = vec![
            (point(-2.0, 0.0, 0.0), vector(0.2673, 0.5345, 0.8018)),
            (point(2.0, 0.0, 2.0), vector(0.0, 0.0, -1.0)),
        ];
        for t in tests {
            let r = Ray::new(t.0, t.1);
            let xs = Cube::local_intersect(&r);
            assert!(xs.is_empty());
        }
    }

    #[test]
    fn normal_on_cube_surface() {
        let c = cube();
        let tests = vec![
            (point(1.0, 0.5, -0.8), vector_x()),
            (point(-1.0, -0.2, 0.9), vector(-1.0, 0.0, 0.0)),
            (point(1.0, 1.0, 1.0), vector_x()),
            (point(-1.0, -1.0, -1.0), vector(-1.0, 0.0, 0.0)),
        ];
        for t in tests {
            let normal = Cube::local_normal_at(t.0);
            assert_eq!(normal, t.1);
        }
    }
}
