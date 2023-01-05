/**
 * Triangle shape
 */
use crate::bounds::*;
use crate::intersection::*;
use crate::math;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::shape::*;
use crate::tuple::*;

#[derive(Clone, Debug, PartialEq)]
pub struct SmoothTriangle {
    p1: Point,
    p2: Point,
    p3: Point,
    n1: Vector,
    n2: Vector,
    n3: Vector,
    e1: Vector,
    e2: Vector,
    normal: Vector,
}

// constructor utilities
pub fn smooth_triangle_with_id(
    id: Option<String>,
    p1: Point,
    p2: Point,
    p3: Point,
    n1: Vector,
    n2: Vector,
    n3: Vector,
) -> Object {
    let e1 = p2 - p1;
    let e2 = p3 - p1;
    let norm = (e2.xyz()).cross(&e1.xyz()).normalize();

    Object::new(id).with_shape(Shape::SmoothTriangle(SmoothTriangle {
        p1,
        p2,
        p3,
        n1,
        n2,
        n3,
        e1,
        e2,
        normal: vector(norm.x, norm.y, norm.z),
    }))
}

pub fn smooth_triangle(
    p1: Point,
    p2: Point,
    p3: Point,
    n1: Vector,
    n2: Vector,
    n3: Vector,
) -> Object {
    smooth_triangle_with_id(None, p1, p2, p3, n1, n2, n3)
}

impl SmoothTriangle {
    pub fn local_normal_at(&self, point: &Point, maybe_hit: Option<&Intersection>) -> Vector {
        if let Some(hit) = maybe_hit {
            self.n2 * hit.u + self.n3 * hit.v + self.n1 * (1.0 - hit.u - hit.v)
        } else {
            panic!("local_normal_at without intersection arg")
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<(math::F3D, math::F3D, math::F3D)> {
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
                    vec![(t, u, v)]
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
    use crate::assert_eq_eps;
    use crate::assert_eq_feps;
    use crate::computations::*;

    fn setup() -> (Object) {
        (smooth_triangle(
            point_y(),
            point_x() * -1.0,
            point_x(),
            vector_y(),
            vector_x() * -1.0,
            vector_x(),
        ))
    }

    #[test]
    fn intersection_saves_uv() {
        let tri = setup();
        let ray = Ray::new(point(-0.2, 0.3, -2.0), vector_z());
        let xs = tri.intersect(&ray);
        assert_eq_feps!(xs[0].u, 0.45);
        assert_eq_feps!(xs[0].v, 0.25);
    }

    #[test]
    fn uses_uv_to_interpolate_normal() {
        let tri = setup();
        let i = Intersection::with_uv(&tri, 1.0, 0.45, 0.25);
        let n = tri.normal_at(point_zero(), Some(&i));
        assert_eq_eps!(n, vector(-0.5547, 0.83205, 0.0));
    }

    #[test]
    fn preparing_the_normal() {
        let tri = setup();
        let i = Intersection::with_uv(&tri, 1.0, 0.45, 0.25);
        let ray = Ray::new(point(-0.2, 0.3, -2.0), vector_z());
        let xs = Intersections::from_intersections(vec![i.clone()]);
        let comps = prepare_computations(&i, &ray, &xs);
        assert_eq_eps!(comps.normalv, vector(-0.5547, 0.83205, 0.0));
    }
}
