use crate::intersection::sort_intersections;
use crate::intersection::Intersection;
use crate::materials::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;
use std::{
    cell::RefCell,
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
    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut res = vec![];
        for s in self.shapes.borrow().iter() {
            let xs = s.val.intersect(ray);
            res.extend(xs);
        }
        sort_intersections(&mut res);
        res
    }
    fn local_normal_at(&self, _point: Point) -> Vector {
        panic!("local_normal_at should never be called on a group!");
    }
}

// default constructor
pub fn default_group() -> GroupRef {
    Group::from_shape(Box::new(test_shape()))
}

fn set_parent(child: &GroupRef, parent: &GroupRef) {
    // `child_node.parent` is set to weak reference to `parent_node`.
    println!("adding {} to {}", child.get_id(), parent.get_id());
    *child.parent.borrow_mut() = Arc::downgrade(&parent);
}

fn add_child_shape(parent: &GroupRef, shape: ShapeBox) {
    // Make a GroupRef
    let g = Group::from_shape(shape);
    set_parent(&g, parent);
    parent.shapes.borrow_mut().push(g.clone())
}

pub fn add_child_group(parent: &GroupRef, child: &GroupRef) {
    set_parent(child, parent);
    parent.shapes.borrow_mut().push(child.clone());
}

/**
 * Helpers to limit syntax explosions
 */

fn set_transform(group: &mut GroupRef, transform: &Matrix4) {
    (*Arc::get_mut(group).unwrap()).set_transform(transform);
}

fn get_parent(group: &GroupRef) -> Option<GroupRef> {
    group.parent.borrow().upgrade()
}

fn has_parent(group: &GroupRef) -> bool {
    get_parent(group).is_some()
}

/**
 * Recursive functions operating up parent trees
 */

pub fn world_to_object(group: &GroupRef, point: &Point) -> Point {
    let mut p = point.clone();

    if has_parent(group) {
        // lol Rust
        p = world_to_object(&get_parent(group).unwrap(), point);
    }
    glm::inverse(group.val.get_transform()) * p
}

pub fn normal_to_world(group: &GroupRef, normal: &Vector) -> Vector {
    let mut n = glm::inverse(group.get_transform()).transpose() * normal;
    n.w = 0.0;
    n = n.normalize();

    if has_parent(group) {
        n = normal_to_world(&get_parent(group).unwrap(), &n);
    }
    n
}

pub fn normal_at(group: &GroupRef, world_point: &Point) -> Vector {
    let local_point = world_to_object(group, world_point);
    let local_normal = group.val.local_normal_at(local_point);
    normal_to_world(group, &local_normal)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::test_shape;
    use crate::sphere::*;
    use crate::transformation::*;

    #[test]
    fn transform_is_identity() {
        let g = default_group();
        assert_eq!(*g.get_transform(), Matrix4::identity());
    }

    #[test]
    fn set_transform_on_group() {
        let mut g = default_group();
        // eesh
        (*Arc::get_mut(&mut g).unwrap()).set_transform(&make_translation(1.0, 0.0, 0.0));
        assert_eq!(*g.get_transform(), make_translation(1.0, 0.0, 0.0));
    }

    #[test]
    fn default_parent_is_empty() {
        let g = default_group();
        assert!(g.parent.borrow().upgrade().is_none());
        assert!(g.shapes.borrow().is_empty())
    }

    #[test]
    fn adding_child_shape() {
        let g = default_group();
        let s = test_shape();
        add_child_shape(&g, Box::new(s.clone()));
        assert_eq!(g.shapes.borrow().len(), 1);
        let child = &g.shapes.borrow()[0];
        assert_eq!(s.get_id(), child.val.get_id());
        assert!(g.shapes.borrow()[0].parent.borrow().upgrade().is_some());
    }

    #[test]
    fn adding_child_group() {
        let g1 = default_group();
        let g2 = default_group();
        add_child_group(&g1, &g2);
        assert_eq!(g1.shapes.borrow().len(), 1);
        let child_group = &g1.shapes.borrow()[0];
        assert_eq!(child_group.get_id(), g2.get_id());
        assert!(g1.shapes.borrow()[0].parent.borrow().upgrade().is_some());
    }

    #[test]
    fn intersection_ray_with_empty_group() {
        let g = default_group();
        let r = Ray::new(point_zero(), vector_z());
        let xs = g.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersection_ray_with_nonempty_group() {
        let g = default_group();
        let s1 = sphere_with_id(Some(String::from("s1")));
        let mut s2 = sphere_with_id(Some(String::from("s2")));
        s2.set_transform(&make_translation(0.0, 0.0, -3.0));
        let mut s3 = sphere_with_id(Some(String::from("s3")));
        s3.set_transform(&make_translation(5.0, 0.0, 0.0));
        add_child_shape(&g, Box::new(s1));
        add_child_shape(&g, Box::new(s2));
        add_child_shape(&g, Box::new(s3));
        let r = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let xs = g.local_intersect(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object.get_id(), String::from("sphere_s2"));
        assert_eq!(xs[1].object.get_id(), String::from("sphere_s2"));
        assert_eq!(xs[2].object.get_id(), String::from("sphere_s1"));
        assert_eq!(xs[3].object.get_id(), String::from("sphere_s1"));
    }

    #[test]
    fn intersecting_transformed_group() {
        let mut g = default_group();
        (*Arc::get_mut(&mut g).unwrap()).set_transform(&make_scaling(2.0, 2.0, 2.0));
        let mut s = sphere();
        s.set_transform(&make_translation(5.0, 0.0, 0.0));
        add_child_shape(&g, Box::new(s));
        let r = Ray::new(point(10.0, 0.0, -10.0), vector_z());
        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn converting_point_from_world_to_object_space() {
        let mut g1 = default_group();
        set_transform(&mut g1, &make_rotation_y(glm::half_pi()));
        let mut g2 = default_group();
        set_transform(&mut g2, &make_scaling(2.0, 2.0, 2.0));
        add_child_group(&g1, &g2);
        let mut s = sphere();
        s.set_transform(&make_translation(5.0, 0.0, 0.0));
        let g3 = Group::from_shape(Box::new(s.clone()));
        add_child_group(&g2, &g3);
        let p = world_to_object(&g3, &point(-2.0, 0.0, -10.0));
        assert_eq_eps!(p, point(0.0, 0.0, -1.0));
    }

    #[test]
    fn converting_normal_from_object_to_normal_space() {
        let mut g1 = default_group();
        set_transform(&mut g1, &make_rotation_y(glm::half_pi()));
        let mut g2 = default_group();
        set_transform(&mut g2, &make_scaling(1.0, 2.0, 3.0));
        add_child_group(&g1, &g2);
        let mut s = sphere();
        s.set_transform(&make_translation(5.0, 0.0, 0.0));
        let g3 = Group::from_shape(Box::new(s.clone()));
        add_child_group(&g2, &g3);
        let threes = 3_f64.sqrt() / 3.0;
        let n = normal_to_world(&g3, &vector(threes, threes, threes));
        assert_eq_eps!(n, vector(0.2857, 0.4286, -0.8571));
    }

    #[test]
    fn find_normal_on_child() {
        let mut g1 = default_group();
        set_transform(&mut g1, &make_rotation_y(glm::half_pi()));
        let mut g2 = default_group();
        set_transform(&mut g2, &make_scaling(1.0, 2.0, 3.0));
        add_child_group(&g1, &g2);
        let mut s = sphere();
        s.set_transform(&make_translation(5.0, 0.0, 0.0));
        let g3 = Group::from_shape(Box::new(s.clone()));
        add_child_group(&g2, &g3);
        let n = normal_at(&g3, &point(1.7321, 1.1547, -5.5774));
        assert_eq_eps!(n, vector(0.2857, 0.4286, -0.8571));
    }

    #[test]
    #[should_panic]
    fn local_normal_at_illegal() {
        default_group().local_normal_at(point_zero());
    }
}
