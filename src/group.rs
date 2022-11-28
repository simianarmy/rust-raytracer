use crate::intersection::Intersection;
use crate::materials::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;
use std::{
    cell::RefCell,
    rc::Rc,
    sync::{Arc, Mutex, RwLock, Weak},
};

#[derive(Clone, Debug)]
pub struct Group {
    pub props: Shape3D,
    pub parent: RefCell<Weak<ShapeBox>>,
    pub shapes: RefCell<Vec<Rc<ShapeBox>>>,
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
    fn get_parent(&self) -> Option<Box<Self>> {
        self.props.parent
    }
}

// default constructor
pub fn group() -> Group {
    Group {
        props: Shape3D::default(),
        parent: RefCell::new(Weak::new()),
        shapes: RefCell::new(Vec::new()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_parent_is_empty() {
        let g = group();
        assert!(g.props.parent.is_none());
    }
}
