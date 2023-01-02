use crate::bounds::*;
use crate::intersection::*;
use crate::materials::Material;
use crate::matrix::Matrix4;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::shape::*;
use crate::tuple::*;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

//pub type GroupRef = Arc<Group>;
type Parent = RefCell<Weak<Object>>;
//type Children = RefCell<Vec<Object>>;
type Children = Vec<Object>;

pub static NUM_BOUNDING_OPTS: AtomicUsize = AtomicUsize::new(0);

fn bump_num_bounding_opts() {
    NUM_BOUNDING_OPTS.fetch_add(1, Ordering::SeqCst);
}

#[derive(Clone, Debug)]
pub struct Group {
    //pub val: Box<Object>,
    //pub parent: Parent,
    pub children: Children,
    bounds: Option<Bounds>,
}

/**
 * Leaf: Group with 1 non-group child
 */
impl Group {
    pub fn from_shape(obj: &Object) -> GroupRef {
        let g = Group {
            //val: Box::new(obj.clone()),
            parent: RefCell::new(Weak::new()),
            shapes: RefCell::new(Vec::new()),
            bounds: None,
        };
        g.add_child_shape(obj);
        let g_ref = Arc::new(g);
        g_ref
    }

    pub fn from_shapes(shapes: &Vec<Object>) -> GroupRef {
        let mut g = Group::new();
        for s in shapes.iter() {
            add_child_shape(&mut g, &s);
        }
        g
    }

    pub fn new() -> GroupRef {
        let g = Group {
            val: Box::new(Object::new(None)),
            parent: RefCell::new(Weak::new()),
            shapes: RefCell::new(Vec::new()),
            bounds: None,
        };
        let g_ref = Arc::new(g);
        g_ref
    }

    pub fn get_transform(&self) -> &Matrix4 {
        self.val.get_transform()
    }

    pub fn is_leaf(&self) -> bool {
        self.num_children() == 1 // TODO: && shape is not group
    }

    pub fn num_children(&self) -> usize {
        self.shapes.borrow().len()
    }

    pub fn add_child_shape(&mut self, shape: &Object) {
        // Make a GroupRef
        self.shapes.borrow_mut().push(*shape)
        //let g = Group::from_shape(shape);
        //add_child_group(parent, &g);
    }

    pub fn calculate_bounds(&self) -> Bounds {
        if self.is_leaf() {
            self.val.parent_space_bounds()
        } else {
            let mut bounds = Bounds::default();
            for s in self.shapes.borrow().iter() {
                bounds.add_bounds(&s.bounds());
            }
            bounds
        }
    }

    pub fn local_intersect<'a>(&'a self, ray: &Ray) -> Intersections<'a> {
        // Test group's bounding box first
        if self.bounds().intersects(ray) {
            // OK I need to be able to iterate over self.shapes without
            // causing a borrow...
            let mut acc = Intersections::new();
            let shit = self.shapes.as_ptr();
            unsafe {
                for i in 0..(*shit).len() {
                    //shit.iter().fold(Intersections::new(), |mut acc, curr| {
                    let xs = (*shit).get(i).unwrap().intersect(ray);
                    for is in xs.intersections {
                        acc.push(is.clone());
                    }
                }
            }
            acc.sort_intersections()
        } else {
            bump_num_bounding_opts();
            Intersections::new()
        }
    }

    // TODO: Move to Shape::intersect
    pub fn intersect<'a>(&'a self, ray: &Ray) -> Intersections<'a> {
        // Need elegant way to differentiate Leaf vs Group nodes
        if self.is_leaf() {
            return self.val.intersect(ray);
        }
        let t_ray = ray.transform(glm::inverse(self.get_transform()));
        self.local_intersect(&t_ray)
    }

    pub fn bounds(&self) -> Bounds {
        self.calculate_bounds()
    }

    pub fn get_parent(&self) -> Option<GroupRef> {
        self.parent.borrow().upgrade()
    }

    pub fn has_parent(&self) -> bool {
        self.get_parent().is_some()
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sid = String::from("");
        if self.is_leaf() {
            sid = self.val.get_id();
            write!(f, "Group leaf (shape: {})", sid)
        } else {
            write!(f, "Group")
        }
    }
}

fn set_bounds(g: &mut GroupRef) {
    //(*Arc::get_mut(g).unwrap()).bounds = Some(g.calculate_bounds());
}

fn set_parent(child: &GroupRef, parent: &mut GroupRef) {
    // `child_node.parent` is set to weak reference to `parent_node`.
    //println!("adding {} to {}", child.get_id(), parent.get_id());
    *child.parent.borrow_mut() = Arc::downgrade(&parent);
}

pub fn add_child_shape(parent: &mut GroupRef, shape: &Object) {
    // Make a GroupRef
    let g = Group::from_shape(shape);
    add_child_group(parent, &g);
}

pub fn add_child_group(parent: &mut GroupRef, child: &GroupRef) {
    set_parent(child, parent);
    parent.shapes.borrow_mut().push(child.clone());
    // We want to recalculate the bounding box for parent
    set_bounds(parent);
}

pub fn partition_children(g: &mut GroupRef) -> (GroupRef, GroupRef) {
    let mut left = Group::new();
    let mut right = Group::new();
    let (lbounds, rbounds) = g.bounds().split();

    for s in g.shapes.borrow().iter() {
        let sbounds = s.bounds();
        if lbounds.contains_bounds(&sbounds) {
            add_child_group(&mut left, s);
        } else if rbounds.contains_bounds(&sbounds) {
            add_child_group(&mut right, s);
        }
    }
    // remove copied shapes from the original list
    g.shapes.borrow_mut().retain(|s| {
        let sbounds = s.bounds();
        !lbounds.contains_bounds(&sbounds) && !rbounds.contains_bounds(&sbounds)
    });
    // Recalculate bounds for g
    set_bounds(g);
    set_bounds(&mut left);
    set_bounds(&mut right);

    (left, right)
}

pub fn make_subgroup(g: &mut GroupRef, shapes: &Vec<Object>) {
    let subgroup = Group::from_shapes(shapes);
    add_child_group(g, &subgroup);
}

pub fn divide(g: &mut GroupRef, threshold: usize) {
    // divide on a shape is a no-op
    if g.is_leaf() {
        return;
    }
    if threshold <= g.num_children() {
        let (left, right) = partition_children(g);
        if left.num_children() > 0 {
            add_child_group(g, &left);
        }
        if right.num_children() > 0 {
            add_child_group(g, &right);
        }
    }
    for child in g.shapes.borrow_mut().iter_mut() {
        divide(child, threshold);
    }
}

/**
 * Helpers to limit syntax explosions
 */

fn get_parent(group: &GroupRef) -> Option<GroupRef> {
    group.parent.borrow().upgrade()
}

fn has_parent(group: &GroupRef) -> bool {
    get_parent(group).is_some()
}

/**
 * Recursive functions operating up parent trees
 */

/*
pub fn world_to_object(group: &GroupRef, point: &Point) -> Point {
    let mut p = point.clone();

    if has_parent(group) {
        p = world_to_object(&get_parent(group).unwrap(), point);
    }
    if group.is_leaf() {
        glm::inverse(group.val.get_transform()) * p
    } else {
        glm::inverse(group.get_transform()) * p
    }
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
*/

/**
 * Important function here
pub fn normal_at(group: &GroupRef, world_point: &Point) -> Vector {
    let local_point = world_to_object(group, world_point);
    let mut local_normal = vector_zero(); // is this the right default value?
    if group.is_leaf() {
        local_normal = group.val.normal_at(local_point)
    }
    normal_to_world(group, &local_normal)
}
 */

#[cfg(test)]
mod tests {
    use super::*;
    //use crate::shapes::cylinder::*;
    use crate::assert_eq_eps;
    use crate::shapes::shape::*;
    use crate::shapes::sphere::*;
    use crate::transformation::*;

    // default constructor
    fn group_object() -> Object {
        Object::group()
    }

    #[test]
    fn transform_is_identity() {
        let g = group_object();
        assert_eq!(g.get_transform(), &Matrix4::identity());
    }

    #[test]
    fn set_transform_on_group() {
        let mut g = group_object();
        // eesh
        g.set_transform(&make_translation(1.0, 0.0, 0.0));
        println!("group: {:?}", g);
        assert_eq!(g.get_transform(), &make_translation(1.0, 0.0, 0.0));
    }

    #[test]
    fn default_parent_is_empty() {
        let g = group_object();
        match g.shape {
            Shape::Group(g) => {
                assert!(g.parent.borrow().upgrade().is_none());
                assert!(g.shapes.borrow().is_empty())
            }
            _ => panic!("shape is not a group"),
        }
    }

    #[test]
    fn adding_child_shape() {
        let g = group_object();

        match g.shape {
            Shape::Group(mut g) => {
                let s = test_shape();
                add_child_shape(&mut g, &s);
                assert_eq!(g.shapes.borrow().len(), 1);
                let child = &g.shapes.borrow()[0];
                assert_eq!(s.get_id(), child.val.as_ref().get_id());
                assert!(g.shapes.borrow()[0].parent.borrow().upgrade().is_some());
            }
            _ => panic!("nope"),
        }
    }

    #[test]
    fn adding_child_group() {
        let g1 = group_object();

        match g1.shape {
            Shape::Group(mut gg1) => {
                let g2 = group_object();
                let g2_id = g2.get_id();
                match g2.shape {
                    Shape::Group(gg2) => {
                        add_child_group(&mut gg1, &gg2);
                        assert_eq!(gg1.shapes.borrow().len(), 1);
                        let child_group = &gg1.shapes.borrow()[0];
                        //assert_eq!(child_group.val.get_id(), g2_id);
                        assert!(gg1.shapes.borrow()[0].parent.borrow().upgrade().is_some());
                    }
                    _ => panic!("g2 fail"),
                }
            }
            _ => panic!("g1 fail"),
        }
    }

    /*
    #[test]
    fn intersection_ray_with_empty_group() {
        let g = group_object();
        let r = Ray::new(point_zero(), vector_z());
        let xs = g.local_intersect(&r);
        assert!(xs.is_empty());
    }

    #[test]
    fn intersection_ray_with_nonempty_group() {
        let mut g = group_object();
        let s1 = sphere_with_id(Some(String::from("s1")));
        let mut s2 = sphere_with_id(Some(String::from("s2")));
        s2.set_transform(&make_translation(0.0, 0.0, -3.0));
        let mut s3 = sphere_with_id(Some(String::from("s3")));
        s3.set_transform(&make_translation(5.0, 0.0, 0.0));
        add_child_shape(&mut g, &s1);
        add_child_shape(&mut g, &s2);
        add_child_shape(&mut g, &s3);
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
        let mut g = group_object();
        set_transform(&mut g, &make_scaling(2.0, 2.0, 2.0));
        let mut s = sphere();
        s.set_transform(&make_translation(5.0, 0.0, 0.0));
        add_child_shape(&mut g, &s);
        let r = Ray::new(point(10.0, 0.0, -10.0), vector_z());
        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn converting_point_from_world_to_object_space() {
        let mut g1 = group_object();
        set_transform(&mut g1, &make_rotation_y(glm::half_pi()));
        let mut g2 = group_object();
        set_transform(&mut g2, &make_scaling(2.0, 2.0, 2.0));
        add_child_group(&mut g1, &g2);
        let mut s = sphere();
        s.set_transform(&make_translation(5.0, 0.0, 0.0));
        let g3 = Group::from_shape(&s);
        add_child_group(&mut g2, &g3);
        //let g3 = Group::new();
        //add_child_shape(&g3, Box::new(s));
        let p = world_to_object(&g3, &point(-2.0, 0.0, -10.0));
        assert_eq_eps!(p, point(0.0, 0.0, -1.0));
    }

    #[test]
    fn converting_normal_from_object_to_normal_space() {
        let mut g1 = group_object();
        set_transform(&mut g1, &make_rotation_y(glm::half_pi()));
        let mut g2 = group_object();
        set_transform(&mut g2, &make_scaling(1.0, 2.0, 3.0));
        add_child_group(&mut g1, &g2);
        let mut s = sphere();
        s.set_transform(&make_translation(5.0, 0.0, 0.0));
        let mut g3 = Group::new();
        add_child_shape(&mut g3, &s);
        add_child_group(&mut g2, &g3);
        let threes = 3_f64.sqrt() / 3.0;
        let n = normal_to_world(&g3, &vector(threes, threes, threes));
        assert_eq_eps!(n, vector(0.2857, 0.4286, -0.8571));
    }

    #[test]
    fn find_normal_on_child() {
        let mut g1 = group_object();
        set_transform(&mut g1, &make_rotation_y(glm::half_pi()));
        let mut g2 = group_object();
        set_transform(&mut g2, &make_scaling(1.0, 2.0, 3.0));
        add_child_group(&mut g1, &g2);
        let mut s = sphere();
        s.set_transform(&make_translation(5.0, 0.0, 0.0));
        //let g3 = Group::from_shape(&s);
        //add_child_group(&mut g2, &g3);
        //let n = normal_at(&g3, &point(1.7321, 1.1547, -5.5774));
        add_child_shape(&mut g2, &s);
        let n = s.normal_at(point(1.7321, 1.1547, -5.5774));
        assert_eq_eps!(n, vector(0.2857, 0.4286, -0.8571));
    }

    #[test]
    #[should_panic]
    fn local_normal_at_illegal() {
        group_object().local_normal_at(point_zero());
    }

    #[test]
    fn bounding_box_fits_children() {
        let mut s = sphere();
        s.set_transform(&(make_translation(2.0, 5.0, -3.0) * make_scaling(2.0, 2.0, 2.0)));
        let mut c = cylinder();
        // TODO: Fix
        c.set_bounds(-2.0, 2.0);
        c.set_transform(&(make_translation(-4.0, -1.0, 4.0) * make_scaling(0.5, 1.0, 0.5)));
        let mut g = group_object();
        add_child_shape(&mut g, &s);
        add_child_shape(&mut g, &c);
        let bounds = g.bounds();
        assert_eq!(bounds.min, point(-4.5, -3.0, -5.0));
        assert_eq!(bounds.max, point(4.0, 7.0, 4.5));
    }

    #[test]
    fn intersecting_ray_group_skips_tests_if_box_missed() {
        NUM_BOUNDING_OPTS.store(0, Ordering::SeqCst);
        let child = test_shape();
        let mut group = group_object();
        add_child_shape(&mut group, &child);
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector_y());
        group.intersect(&ray);
        assert_eq!(NUM_BOUNDING_OPTS.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn intersecting_ray_group_tests_children_if_box_hit() {
        let child = test_shape();
        let mut group = group_object();
        add_child_shape(&mut group, &child);
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let xs = group.intersect(&ray);
        // TODO: Test optimization not made
    }

    #[test]
    fn partition_groups_children() {
        let mut s1 = sphere_with_id(Some(String::from("s1")));
        s1.set_transform(&make_translation(-2.0, 0.0, 0.0));
        let mut s2 = sphere_with_id(Some(String::from("s2")));
        s2.set_transform(&make_translation(2.0, 0.0, 0.0));
        let s3 = sphere_with_id(Some(String::from("s3")));
        let mut g = group_object();
        add_child_shape(&mut g, &s1);
        add_child_shape(&mut g, &s2);
        add_child_shape(&mut g, &s3);
        let (left, right) = partition_children(&mut g);
        // g is a group of [s3]
        assert_eq!(g.num_children(), 1);
        assert_eq!(g.shapes.borrow()[0].get_id(), String::from("g_sphere_s3"));
        assert_eq!(left.num_children(), 1);
        assert_eq!(
            left.shapes.borrow()[0].get_id(),
            String::from("g_sphere_s1")
        );
        assert_eq!(right.num_children(), 1);
        assert_eq!(
            right.shapes.borrow()[0].get_id(),
            String::from("g_sphere_s2")
        );
    }

    #[test]
    fn creating_subgroup_from_children() {
        let s1 = sphere_with_id(Some("s1".to_string()));
        let s2 = sphere_with_id(Some("s2".to_string()));
        let mut g = group_object();
        make_subgroup(&mut g, &vec![Box::new(s1), Box::new(s2)]);
        assert_eq!(g.num_children(), 1);
        let g0 = Arc::clone(g.shapes.borrow().get(0).unwrap());
        assert_eq!(g0.shapes.borrow().len(), 2);
        // g[0] is group [s1, s2]
    }

    #[test]
    fn subdividing_group_partitions_its_children() {
        let mut s1 = sphere_with_id(Some(String::from("s1")));
        s1.set_transform(&make_translation(-2.0, -2.0, 0.0));
        let mut s2 = sphere_with_id(Some(String::from("s2")));
        s2.set_transform(&make_translation(-2.0, 2.0, 0.0));
        let mut s3 = sphere_with_id(Some(String::from("s3")));
        s3.set_transform(&make_scaling(4.0, 4.0, 4.0));
        let mut g = group_object();
        add_child_shape(&mut g, Box::new(s1.clone()));
        add_child_shape(&mut g, Box::new(s2.clone()));
        add_child_shape(&mut g, Box::new(s3.clone()));
        divide(&mut g, 1);
        assert_eq!(g.num_children(), 2);
        assert_eq!(g.shapes.borrow()[0].get_id(), String::from("g_sphere_s3"));
        let g1 = Arc::clone(g.shapes.borrow().get(1).unwrap());
        assert_eq!(g1.num_children(), 2);
        // g1[0] is a subgroup of [s1]
        assert_eq!(
            g1.shapes.borrow()[0].shapes.borrow()[0].get_id(),
            String::from("g_sphere_s1")
        );
        // g1[1] is a subgroup of [s2]
        assert_eq!(
            g1.shapes.borrow()[1].shapes.borrow()[0].get_id(),
            String::from("g_sphere_s2")
        );
    }

    #[test]
    fn subdividing_group_with_too_few_children() {
        let mut s1 = sphere_with_id(Some(String::from("s1")));
        s1.set_transform(&make_translation(-2.0, 0.0, 0.0));
        let mut s2 = sphere_with_id(Some(String::from("s2")));
        s2.set_transform(&make_translation(2.0, 1.0, 0.0));
        let mut s3 = sphere_with_id(Some(String::from("s3")));
        s3.set_transform(&make_translation(2.0, -1.0, 0.0));
        let mut subgroup = Group::new();
        add_child_shape(&mut subgroup, Box::new(s1.clone()));
        add_child_shape(&mut subgroup, Box::new(s2.clone()));
        add_child_shape(&mut subgroup, Box::new(s3.clone()));
        let s4 = sphere_with_id(Some(String::from("s4")));
        let mut g = group_object();
        add_child_group(&mut g, &subgroup);
        add_child_shape(&mut g, Box::new(s4));
        divide(&mut g, 3);
        assert_eq!(g.num_children(), 2);
        let g0 = Arc::clone(g.shapes.borrow().get(0).unwrap());
        assert_eq!(g0.num_children(), 2);
        // g0[0] is a group of [s1]
        assert_eq!(g0.shapes.borrow()[0].num_children(), 1);
        assert_eq!(
            g0.shapes.borrow()[0].shapes.borrow()[0].get_id(),
            String::from("g_sphere_s1")
        );
        // g0[1] is a group of [s2, s3]
        assert_eq!(
            g0.shapes.borrow()[1].shapes.borrow()[0].get_id(),
            String::from("g_sphere_s2")
        );
    }
    */
}
