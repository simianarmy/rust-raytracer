use crate::intersection::Intersection;
use crate::materials::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;
use std::{cell::RefCell, rc::Rc, sync::Weak};

type GroupRef<T> = Rc<T>;
type Parent<T> = RefCell<Weak<T>>;
type Children<T> = RefCell<Vec<Rc<T>>>;

#[derive(Clone, Debug)]
pub struct Group<T> {
    pub props: T,
    pub parent: Parent<T>,
    pub shapes: Children<T>,
}

impl<T> Group<T> {
    fn from_shape(shape: Shape3D) -> Group<T> {
        Group {
            props: shape,
            parent: RefCell::new(Weak::new()),
            shapes: RefCell::new(Vec::new()),
        }
    }

    fn add_child(&self, shape: Shape3D) {
        let g = Group::from_shape(shape);
        // `child_node.parent` is set to weak reference to `parent_node`.
        *g.parent.borrow_mut() = Rc::downgrade(&self);
        self.shapes.borrow_mut().push(g.clone())
    }
}

impl Shape for Group {
    fn get_id(&self) -> String {
        format!("group_{}", self.props.id)
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
    fn local_intersect(&self, _ray: &Ray) -> Vec<Intersection> {
        //self.saved_ray = ray;
        vec![]
    }
    fn local_normal_at(&self, _point: Point) -> Vector {
        point_zero()
    }
}

// default constructor
impl Default for Group {
    fn default() -> Group {
        Group::from_shape(Shape3D::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::test_shape;

    #[test]
    fn transform_is_identity() {
        let g = Group::default();
        assert_eq!(*g.get_transform(), Matrix4::identity());
    }

    #[test]
    fn default_parent_is_empty() {
        let g = Group::default();
        assert!(g.parent.borrow().upgrade().is_none());
        assert!(g.shapes.borrow().is_empty())
    }

    #[test]
    fn adding_child() {
        let g = Group::default();
        let s = test_shape();
        g.add_child(&s);
    }
}
