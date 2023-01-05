use crate::bounds::Bounds;
use crate::math;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::shape::*;
use crate::tuple::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {}

// constructor utilities
pub fn sphere_with_id(id: Option<String>) -> Object {
    Object::new(id).with_shape(Shape::Sphere())
}

pub fn sphere() -> Object {
    sphere_with_id(None)
}

impl Sphere {
    pub fn local_intersect(ray: &Ray) -> Vec<math::F3D> {
        let sphere_to_ray = ray.origin - point_zero();
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let ds = discriminant.sqrt();
            let t1 = (-b - ds) / (2.0 * a);
            let t2 = (-b + ds) / (2.0 * a);
            vec![t1, t2]
        }
    }

    pub fn local_normal_at(point: &Point) -> Vector {
        point - point_zero()
    }

    pub fn bounds() -> Bounds {
        Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0))
    }
}

// test helper
pub fn glass_sphere() -> Object {
    let mut s = sphere();
    s.material.transparency = 1.0;
    s.material.refractive_index = 1.5;
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_eps;
    use crate::intersection::Intersection;
    use crate::transformation::*;

    #[test]
    fn ray_intersects_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn ray_in_front() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_intersection() {
        let sid = String::from("itme");
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere_with_id(Some(sid.clone()));
        let xs = s.intersect(&r);
        assert_eq!(xs[0].object.get_id(), String::from("sphere_itme"));
        assert_eq!(xs[1].object.get_id(), String::from("sphere_itme"));
    }

    #[test]
    fn normal_at_point_on_x_axis() {
        let s = sphere();
        let n = s.normal_at(point(1.0, 0.0, 0.0), None);
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_at_nonaxial_point() {
        let s = sphere();
        let val = 3_f64.sqrt() / 3.0;
        let n = Sphere::local_normal_at(&point(val, val, val));
        assert_eq_eps!(&n, &vector(val, val, val));
    }

    #[test]
    fn normal_at_is_normalized() {
        let s = sphere();
        let val = 3_f64.sqrt() / 3.0;
        let n = s.normal_at(point(val, val, val), None);
        assert_eq_eps!(&n, &glm::normalize(&n));
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut s = sphere();
        s.set_transform(&make_translation(0.0, 1.0, 0.0));
        let n = s.normal_at(point(0.0, 1.70711, -0.70711), None);
        assert_eq_eps!(&n, &vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = sphere();
        let m = make_scaling(1.0, 0.5, 1.0) * make_rotation_z(glm::pi::<crate::math::F3D>() / 5.0);
        s.set_transform(&m);
        let n = s.normal_at(point(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0), None);
        assert_eq_eps!(&n, &vector(0.0, 0.97014, -0.24254));
    }
}
