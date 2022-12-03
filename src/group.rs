use crate::intersection::Intersection;
use crate::materials::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Weak},
};

type GroupRef = Arc<Group>;
type Parent = RefCell<Weak<Group>>;
type Children = RefCell<Vec<GroupRef>>;

#[derive(Clone, Debug)]
pub struct Group {
    pub val: ShapeBox,
    pub parent: Parent,
    pub shapes: Children,
}

impl Group {
    fn from_shape(shape: ShapeBox) -> GroupRef {
        let g = Group {
            val: shape.clone(),
            parent: RefCell::new(Weak::new()),
            shapes: RefCell::new(Vec::new()),
        };
        let g_ref = Arc::new(g);
        g_ref
    }
}

impl Shape for Group {
    fn get_id(&self) -> String {
        format!("group_{}", self.val.get_id())
    }
    fn get_transform(&self) -> &Matrix4 {
        &self.val.get_transform()
    }
    fn set_transform(&mut self, t: &Matrix4) {
        self.val.set_transform(t)
    }
    fn get_material(&self) -> &Material {
        &self.val.get_material()
    }
    fn set_material(&mut self, m: Material) {
        self.val.set_material(m)
    }
    fn local_intersect(&self, _ray: &Ray) -> Vec<Intersection> {
        //self.saved_ray = ray;
        vec![]
    }
    fn local_normal_at(&self, _point: Point) -> Vector {
        point_zero()
    }
}

// default constructor
fn default_group() -> GroupRef {
    Group::from_shape(Box::new(test_shape()))
}

fn add_child(parent: &GroupRef, shape: ShapeBox) {
    // Make a GroupRef
    let g = Group::from_shape(shape);
    // `child_node.parent` is set to weak reference to `parent_node`.
    *g.parent.borrow_mut() = Arc::downgrade(&parent);
    parent.shapes.borrow_mut().push(g.clone())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::test_shape;

    #[test]
    fn transform_is_identity() {
        let g = default_group();
        assert_eq!(*g.get_transform(), Matrix4::identity());
    }

    #[test]
    fn default_parent_is_empty() {
        let g = default_group();
        assert!(g.parent.borrow().upgrade().is_none());
        assert!(g.shapes.borrow().is_empty())
    }

    #[test]
    fn adding_child() {
        let g = default_group();
        let s = test_shape();
        add_child(&g, Box::new(s.clone()));
        assert_eq!(g.shapes.borrow().len(), 1);
        let child = &g.shapes.borrow()[0];
        assert_eq!(s.get_id(), child.val.get_id());
        assert!(g.shapes.borrow()[0].parent.borrow().upgrade().is_some());
    }
}
