use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shape::Intersectable;
use crate::tuple::*;

#[derive(Debug, PartialEq)]
pub struct Sphere {
    id: String,
}

pub fn id_sphere(id: Option<String>) -> Sphere {
    Sphere {
        id: id.unwrap_or("sphere_1".to_string()),
    }
}

pub fn sphere() -> Sphere {
    id_sphere(None)
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: Ray) -> Vec<Intersection<Sphere>> {
        let sphere_to_ray = ray.origin - point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * ray.direction.dot(&sphere_to_ray);
        let c = sphere_to_ray.dot(&sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0_f32 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            vec![
                Intersection {
                    t: t1,
                    object: self,
                },
                Intersection {
                    t: t2,
                    object: self,
                },
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let s = id_sphere(Some(sid));
        let xs = s.intersect(r);
        assert_eq!(xs[0].object.id, String::from("itme"));
        assert_eq!(xs[1].object.id, String::from("itme"));
    }
}
