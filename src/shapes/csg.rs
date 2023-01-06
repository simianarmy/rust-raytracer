use crate::bounds::*;
use crate::math;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::shape::*;
use crate::tuple::*;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum CsgOp {
    Intersection,
    Union,
    Difference,
}

#[derive(Clone)]
pub struct Csg {
    pub op: CsgOp,
    pub left: Box<Object>,
    pub right: Box<Object>,
}

// constructor utilities
pub fn csg_with_id(id: Option<String>, op: CsgOp, s1: &Object, s2: &Object) -> Object {
    Object::new(id).with_shape(Shape::Csg(Csg {
        op,
        left: Box::new(s1.clone()),
        right: Box::new(s2.clone()),
    }))
}

pub fn csg(op: CsgOp, s1: &Object, s2: &Object) -> Object {
    csg_with_id(None, op, s1, s2)
}

impl Csg {
    pub fn local_normal_at(&self, point: &Point) -> Vector {
        match point.abs().max() {
            x if x == point.x.abs() => vector(point.x, 0.0, 0.0),
            y if y == point.y.abs() => vector(0.0, point.y, 0.0),
            _ => vector(0.0, 0.0, point.z),
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<math::F3D> {
        vec![]
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0))
    }
}

impl PartialEq for Csg {
    fn eq(&self, other: &Self) -> bool {
        self.op == other.op && self.left == other.left && self.right == other.right
    }
}

impl fmt::Debug for Csg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Csg",)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::cube::*;
    use crate::shapes::sphere::*;

    #[test]
    fn csg_created_with_op_and_two_shapes() {
        let s1 = sphere();
        let s2 = cube();
        let o = csg(CsgOp::Union, &s1, &s2);
        match o.shape() {
            Shape::Csg(c) => {
                assert_eq!(c.op, CsgOp::Union);
                assert_eq!(*c.left, s1);
                assert_eq!(*c.right, s2);
            }
            _ => panic!(),
        }
    }
}
