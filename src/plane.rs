use crate::group::Group;
use crate::intersection::*;
use crate::materials::Material;
use crate::math;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Plane {
    pub props: Shape3D,
}

// constructor utilities
pub fn plane_with_id(id: Option<String>) -> Plane {
    Plane {
        props: Shape3D::new(id),
    }
}

pub fn plane() -> Plane {
    plane_with_id(None)
}

impl Shape for Plane {
    fn get_id(&self) -> String {
        format!("plane_{}", self.props.id)
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
        if math::f_equals(ray.direction.y, 0.0) {
            vec![]
        } else {
            let t = -ray.origin.y / ray.direction.y;
            vec![self.intersection(t)]
        }
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        point_y()
    }

    fn get_parent(&self) -> Option<Box<Group>> {
        self.props.parent
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_is_constant_everywhere() {
        let p = plane();
        let n1 = p.local_normal_at(point_zero());
        let n2 = p.local_normal_at(point(10.0, 0.0, -10.0));
        assert_eq!(n1, point_y());
        assert_eq!(n2, point_y());
    }

    #[test]
    fn intersect_with_ray_parallel_to_plane() {
        let p = plane();
        let r = Ray::new(point(0.0, 10.0, 0.0), vector_z());
        let xs = p.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersect_with_coplanar_ray() {
        let p = plane();
        let r = Ray::new(point_zero(), vector_z());
        let xs = p.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_intersect_from_above() {
        let p = plane();
        let r = Ray::new(point_y(), vector(0.0, -1.0, 0.0));
        let xs = p.local_intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object.get_id(), p.get_id());
    }
}
