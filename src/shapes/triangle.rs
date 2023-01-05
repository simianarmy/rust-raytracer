use crate::bounds::*;
/**
 * Triangle shape
 */
use crate::math;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::shape::*;
use crate::tuple::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Triangle {
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
    e1: Vector,
    e2: Vector,
    normal: Vector,
}

// constructor utilities
pub fn triangle_with_id(id: Option<String>, p1: Point, p2: Point, p3: Point) -> Object {
    let e1 = p2 - p1;
    let e2 = p3 - p1;
    let norm = (e2.xyz()).cross(&e1.xyz()).normalize();

    Object::new(id).with_shape(Shape::Triangle(Triangle {
        p1,
        p2,
        p3,
        e1,
        e2,
        normal: vector(norm.x, norm.y, norm.z),
    }))
}

pub fn triangle(p1: Point, p2: Point, p3: Point) -> Object {
    triangle_with_id(None, p1, p2, p3)
}

impl Triangle {
    pub fn local_normal_at(&self, point: &Point) -> Vector {
        self.normal
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<math::F3D> {
        let dir_cross_e2 = ray.direction.xyz().cross(&self.e2.xyz());
        let det = self.e1.xyz().dot(&dir_cross_e2);
        if math::f_equals(det.abs(), 0.0) {
            vec![]
        } else {
            let f = 1.0 / det;
            let p1_to_origin = ray.origin - self.p1;
            let u = f * p1_to_origin.xyz().dot(&dir_cross_e2);

            if u < 0.0 || u > 1.0 {
                vec![]
            } else {
                let origin_cross_e1 = p1_to_origin.xyz().cross(&self.e1.xyz());
                let v = f * ray.direction.xyz().dot(&origin_cross_e1);

                if v < 0.0 || (u + v) > 1.0 {
                    vec![]
                } else {
                    let t = f * self.e2.xyz().dot(&origin_cross_e1);
                    vec![t]
                }
            }
        }
    }

    pub fn bounds(&self) -> Bounds {
        let minx = glm::min3_scalar(self.p1.x, self.p2.x, self.p3.x);
        let miny = glm::min3_scalar(self.p1.y, self.p2.y, self.p3.y);
        let minz = glm::min3_scalar(self.p1.z, self.p2.z, self.p3.z);
        let maxx = glm::max3_scalar(self.p1.x, self.p2.x, self.p3.x);
        let maxy = glm::max3_scalar(self.p1.y, self.p2.y, self.p3.y);
        let maxz = glm::max3_scalar(self.p1.z, self.p2.z, self.p3.z);

        Bounds::new(point(minx, miny, minz), point(maxx, maxy, maxz))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_calculates_props() {
        let t = triangle(point_y(), point(-1.0, 0.0, 0.0), point_x());
        match t.shape() {
            Shape::Triangle(t) => {
                assert_eq!(t.p1, point_y());
                assert_eq!(t.p2, point(-1.0, 0.0, 0.0));
                assert_eq!(t.p3, point_x());
                assert_eq!(t.e1, vector(-1.0, -1.0, 0.0));
                assert_eq!(t.e2, vector(1.0, -1.0, 0.0));
                assert_eq!(t.normal, vector(0.0, 0.0, -1.0));
            }
            _ => panic!(),
        }
    }

    #[test]
    fn finding_normal() {
        let t = triangle(point_y(), point(-1.0, 0.0, 0.0), point_x());
        match t.shape() {
            Shape::Triangle(tr) => {
                let n1 = t.normal_at(point(0.0, 0.5, 0.0), None);
                let n2 = t.normal_at(point(0.0, 0.0, -666.69), None);
                assert_eq!(n1, tr.normal);
                assert_eq!(n2, tr.normal);
            }
            _ => panic!(),
        }
    }

    #[test]
    fn intersecting_ray_parallel_to_triangle() {
        let t = triangle(point_y(), point(-1.0, 0.0, 0.0), point_x());
        let ray = Ray::new(point(0.0, -1.0, -2.0), vector_y());
        let xs = t.intersect(&ray);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p1_p3_edge() {
        let t = triangle(point_y(), point(-1.0, 0.0, 0.0), point_x());
        let ray = Ray::new(point(1.0, 1.0, -2.0), vector_z());
        let xs = t.intersect(&ray);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p1_p2_edge() {
        let t = triangle(point_y(), point(-1.0, 0.0, 0.0), point_x());
        let ray = Ray::new(point(-1.0, 1.0, -2.0), vector_z());
        let xs = t.intersect(&ray);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_misses_p2_p3_edge() {
        let t = triangle(point_y(), point(-1.0, 0.0, 0.0), point_x());
        let ray = Ray::new(point(0.0, -1.0, -2.0), vector_z());
        let xs = t.intersect(&ray);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_strikes_triangle() {
        let t = triangle(point_y(), point(-1.0, 0.0, 0.0), point_x());
        let ray = Ray::new(point(0.0, 0.5, -2.0), vector_z());
        let xs = t.intersect(&ray);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 2.0);
    }
}
