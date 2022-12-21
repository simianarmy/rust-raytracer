use crate::bounds::Bounds;
use crate::group::Group;
use crate::intersection::*;
use crate::materials::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    pub props: Shape3D,
}

// constructor utilities
pub fn sphere_with_id(id: Option<String>) -> Sphere {
    Sphere {
        props: Shape3D::new(id),
    }
}

pub fn sphere() -> Sphere {
    sphere_with_id(None)
}

impl Shape for Sphere {
    fn get_id(&self) -> String {
        format!("sphere_{}", self.props.id)
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
        let sphere_to_ray = ray.origin - point_zero();
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            crate::intersections!(
                Intersection::new(Box::new(self.clone()), t1),
                Intersection::new(Box::new(self.clone()), t2)
            )
        }
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        point - point_zero()
    }

    fn bounds(&self) -> Bounds {
        Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0))
    }
}

// test helper
pub fn glass_sphere() -> Sphere {
    let mut s = sphere();
    s.props.material.transparency = 1.0;
    s.props.material.refractive_index = 1.5;
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_eps;
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
        let s = sphere_with_id(Some(sid));
        let xs = s.intersect(&r);
        assert_eq!(xs[0].group.get_id(), String::from("g_sphere_itme"));
        assert_eq!(xs[1].group.get_id(), String::from("g_sphere_itme"));
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
        let val = 3_f64.sqrt() / 3.0;
        let n = s.normal_at(point(val, val, val));
        assert_eq_eps!(&n, &vector(val, val, val));
    }

    #[test]
    fn normal_at_is_normalized() {
        let s = sphere();
        let val = 3_f64.sqrt() / 3.0;
        let n = s.normal_at(point(val, val, val));
        assert_eq_eps!(&n, &glm::normalize(&n));
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
        let m = make_scaling(1.0, 0.5, 1.0) * make_rotation_z(glm::pi::<crate::math::F3D>() / 5.0);
        s.props.transform = m;
        let n = s.normal_at(point(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));
        assert_eq_eps!(&n, &vector(0.0, 0.97014, -0.24254));
    }
}
