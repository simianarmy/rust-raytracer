use crate::bounds::*;
use crate::intersection::*;
use crate::math;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::*;
use std::sync::{Arc, RwLock};

type ChildNode = Arc<RwLock<CsgNode>>;

#[derive(Clone, Debug, PartialEq)]
pub enum CsgOp {
    Intersection,
    Union,
    Difference,
}

#[derive(Clone, Debug)]
pub enum CsgNode {
    Node(Csg),
    Leaf(Object),
}

impl CsgNode {
    pub fn is_object_in_tree(&self, obj: &Object) -> bool {
        match self {
            CsgNode::Node(n) => {
                let ll = n.left.read().unwrap();
                let lr = n.right.read().unwrap();
                ll.is_object_in_tree(obj) || lr.is_object_in_tree(obj)
            }
            CsgNode::Leaf(o) => o == obj,
        }
    }

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Intersections<'a> {
        match self {
            CsgNode::Node(n) => n.intersect(ray),
            CsgNode::Leaf(o) => o.intersect(ray),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Csg {
    pub op: CsgOp,
    pub left: ChildNode,
    pub right: ChildNode,
}

impl Csg {
    // constructor utilities
    pub fn new(op: CsgOp, s1: &Object, s2: &Object) -> Csg {
        let left = CsgNode::Leaf(s1.clone()); // should be Object but then how is s1 stored?
        let right = CsgNode::Leaf(s2.clone());
        Csg {
            op,
            left: Arc::new(RwLock::new(left)),
            right: Arc::new(RwLock::new(right)),
        }
    }

    pub fn is_intersection_allowed(op: &CsgOp, lhit: bool, inl: bool, inr: bool) -> bool {
        match op {
            CsgOp::Union => (lhit && !inr) || (!lhit && !inl),
            CsgOp::Intersection => (lhit && inr) || (!lhit && inl),
            CsgOp::Difference => (lhit && !inr) || (!lhit && inl),
            _ => false,
        }
    }

    pub fn filter_intersections<'a>(&self, xs: &'a Intersections) -> Intersections<'a> {
        let mut result = Intersections::new();
        let mut inl = false;
        let mut inr = false;
        let l = self.left.read().unwrap();

        for is in xs.iter() {
            let lhit = l.is_object_in_tree(&is.object);

            if Csg::is_intersection_allowed(&self.op, lhit, inl, inr) {
                // Optimization: pass by reference
                result.push(is.clone());
            }

            if lhit {
                inl = !inl;
            } else {
                inr = !inr;
            }
        }
        result
    }

    pub fn local_normal_at(&self, point: &Point) -> Vector {
        match point.abs().max() {
            x if x == point.x.abs() => vector(point.x, 0.0, 0.0),
            y if y == point.y.abs() => vector(0.0, point.y, 0.0),
            _ => vector(0.0, 0.0, point.z),
        }
    }

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Intersections<'a> {
        let l = self.left.read().unwrap();
        let r = self.right.read().unwrap();

        let mut xs = l.intersect(ray);
        xs.extend(&r.intersect(ray));

        // combine and sort
        self.filter_intersections(&xs)
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::{cube, shape, sphere};
    use crate::transformation::*;

    #[test]
    fn csg_created_with_op_and_two_shapes() {
        let s1 = sphere::sphere();
        let s2 = cube::cube();
        let o = Object::new_csg(CsgOp::Union, &s1, &s2);
        match o.shape() {
            shape::Shape::Csg(c) => {
                assert_eq!(c.op, CsgOp::Union);
                let l = c.left.read().unwrap();
                match &*l {
                    CsgNode::Leaf(n) => assert_eq!(n, &s1),
                    _ => panic!(),
                }
                let r = c.right.read().unwrap();
                match &*r {
                    CsgNode::Leaf(n) => assert_eq!(n, &s2),
                    _ => panic!(),
                }
            }
            _ => panic!(),
        }
    }

    #[test]
    fn evaluating_rule_for_csg_op() {
        let table = vec![
            (CsgOp::Union, true, true, true, false),
            (CsgOp::Union, true, true, false, true),
            (CsgOp::Union, true, false, true, false),
            (CsgOp::Union, true, false, false, true),
            (CsgOp::Union, false, false, false, true),
            (CsgOp::Intersection, true, true, true, true),
            (CsgOp::Intersection, true, false, true, true),
            (CsgOp::Intersection, false, true, false, true),
            (CsgOp::Intersection, false, false, false, false),
            (CsgOp::Difference, true, true, true, false),
            (CsgOp::Difference, true, false, true, false),
            (CsgOp::Difference, false, true, true, true),
            (CsgOp::Difference, false, false, false, false),
        ];
        for t in table {
            let res = Csg::is_intersection_allowed(&t.0, t.1, t.2, t.3);
            assert_eq!(res, t.4);
        }
    }

    #[test]
    fn filtering_intersections() {
        let s1 = sphere::sphere();
        let s2 = cube::cube();
        for t in vec![
            (CsgOp::Union, 0, 3),
            (CsgOp::Intersection, 1, 2),
            (CsgOp::Difference, 0, 1),
        ] {
            let o = Object::new_csg(t.0, &s1, &s2);
            let xs = Intersections::from_intersections(vec![
                Intersection::new(&s1, 1.0),
                Intersection::new(&s2, 2.0),
                Intersection::new(&s1, 3.0),
                Intersection::new(&s2, 4.0),
            ]);
            match o.shape() {
                shape::Shape::Csg(c) => {
                    let res = c.filter_intersections(&xs);
                    assert_eq!(res.len(), 2);
                    assert_eq!(res[0].object, xs[t.1].object);
                    assert_eq!(res[1].object, xs[t.2].object);
                }
                _ => panic!(),
            }
        }
    }

    #[test]
    fn ray_misses() {
        let s1 = sphere::sphere();
        let s2 = cube::cube();
        let o = Object::new_csg(CsgOp::Union, &s1, &s2);
        let r = Ray::new(point(0.0, 2.0, -5.0), vector_z());
        let xs = o.intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn ray_hits() {
        let s1 = sphere::sphere();
        let mut s2 = sphere::sphere();
        s2.set_transform(&make_translation(0.0, 0.0, 0.5));
        let c = Object::new_csg(CsgOp::Union, &s1, &s2);
        let r = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let xs = c.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[0].object, &s1);
        assert_eq!(xs[1].t, 6.5);
        assert_eq!(xs[2].object, &s2);
    }
}
