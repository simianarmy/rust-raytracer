use crate::bounds::*;
use crate::intersection::*;
use crate::math;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::shape::*;
use crate::tuple::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {}

// constructor utilities
pub fn plane_with_id(id: Option<String>) -> Object {
    let mut o = Object::new(id);
    o.shape = Shape::Plane();
    o
}

pub fn plane() -> Object {
    plane_with_id(None)
}

impl Plane {
    pub fn local_intersect(ray: &Ray) -> Vec<math::F3D> {
        if math::f_equals(ray.direction.y, 0.0) {
            vec![]
        } else {
            let t = -ray.origin.y / ray.direction.y;
            vec![t]
        }
    }

    pub fn local_normal_at(_point: Point) -> Vector {
        point_y()
    }

    pub fn bounds() -> Bounds {
        Bounds {
            min: point(-math::INFINITY, 0.0, -math::INFINITY),
            max: point(math::INFINITY, 0.0, math::INFINITY),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_is_constant_everywhere() {
        let n1 = Plane::local_normal_at(point_zero());
        let n2 = Plane::local_normal_at(point(10.0, 0.0, -10.0));
        assert_eq!(n1, point_y());
        assert_eq!(n2, point_y());
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let r = Ray::new(point(0.0, 10.0, 0.0), vector_z());
        let xs = Plane::local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let r = Ray::new(point_zero(), vector_z());
        let xs = Plane::local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_intersect_from_above() {
        let r = Ray::new(point_y(), vector(0.0, -1.0, 0.0));
        let xs = Plane::local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0], 1.0);
        //assert_eq!(xs[0].object.get_id(), format!("g_{}", p.get_id()));
    }
}
