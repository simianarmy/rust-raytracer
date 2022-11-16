use crate::intersection::*;
use crate::materials::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;
use glm::*;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    props: Shape3D,
}

// constructor utilities
pub fn sphere_with_id(id: Option<String>) -> Sphere {
    Sphere {
        props: Shape3D {
            id: id.unwrap_or("sphere_1".to_string()),
            transform: glm::identity(),
            material: Material::new(),
        },
    }
}

pub fn sphere() -> Sphere {
    sphere_with_id(None)
}

impl NormalAt for Sphere {
    fn get_transform(&self) -> &Matrix4 {
        &self.props.transform
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: Ray) -> Vec<Intersection<Sphere>> {
        let t_ray = ray.transform(inverse(&self.get_transform()));
        let sphere_to_ray = t_ray.origin - point(0.0, 0.0, 0.0);
        let a = t_ray.direction.dot(&t_ray.direction);
        let b = 2.0 * t_ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0_f32 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            crate::intersections!(
                Intersection {
                    t: t1,
                    object: self,
                },
                Intersection {
                    t: t2,
                    object: self,
                }
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_eps;
    use crate::matrix::Matrix4;
    use crate::transformation::*;
    use glm::identity;

    #[test]
    fn instance_has_unique_id() {}

    #[test]
    fn ray_intersects_at_tangent() {
        let r = Ray::new(point(0.0, 1.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn ray_misses_sphere() {
        let r = Ray::new(point(0.0, 2.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn ray_originates_inside() {
        let r = Ray::new(point(0.0, 0.0, 0.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn ray_in_front() {
        let r = Ray::new(point(0.0, 0.0, 5.0), vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_intersection() {
        let sid = String::from("itme");
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let s = sphere_with_id(Some(sid));
        let xs = s.intersect(r);
        assert_eq!(xs[0].object.props.id, String::from("itme"));
        assert_eq!(xs[1].object.props.id, String::from("itme"));
    }

    #[test]
    fn test_default_transform_is_identity() {
        let s = sphere();
        let ident: Matrix4 = identity();
        assert_eq!(s.props.transform, ident);
    }

    #[test]
    fn test_changing_transform() {
        let mut s = sphere();
        let t = make_translation(2.0, 3.0, 4.0);
        s.props.transform = t;
        assert_eq!(s.props.transform, t);
    }

    #[test]
    fn intersect_scaled_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere();
        s.props.transform = make_scaling(2.0, 2.0, 2.0);
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersect_translated_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = sphere();
        s.props.transform = make_translation(5.0, 0.0, 0.0);
        let xs = s.intersect(r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn normal_at_point_on_x_axis() {
        let s = sphere();
        let n = s.normal_at(point(1.0, 0.0, 0.0));
        assert_eq!(n, vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normal_at_nonaxial_point() {
        let s = sphere();
        let val = 3_f32.sqrt() / 3.0;
        let n = s.normal_at(point(val, val, val));
        assert_eq_eps!(&n, &vector(val, val, val));
    }

    #[test]
    fn normal_at_is_normalized() {
        let s = sphere();
        let val = 3_f32.sqrt() / 3.0;
        let n = s.normal_at(point(val, val, val));
        assert_eq_eps!(&n, &normalize(&n));
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut s = sphere();
        s.props.transform = make_translation(0.0, 1.0, 0.0);
        let n = s.normal_at(point(0.0, 1.70711, -0.70711));
        assert_eq_eps!(&n, &vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = sphere();
        let m = make_scaling(1.0, 0.5, 1.0) * make_rotation_z(pi::<f32>() / 5.0);
        s.props.transform = m;
        let n = s.normal_at(point(0.0, 2_f32.sqrt() / 2.0, -2_f32.sqrt() / 2.0));
        assert_eq_eps!(&n, &vector(0.0, 0.97014, -0.24254));
    }
}