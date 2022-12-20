use crate::bounds::*;
use crate::intersection::sort_intersections;
use crate::intersection::Intersection;
use crate::materials::Material;
use crate::math;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::{
    cell::RefCell,
    sync::{Arc, Weak},
};

pub type GroupRef = Arc<Group>;
type Parent = RefCell<Weak<Group>>;
type Children = RefCell<Vec<GroupRef>>;

pub static NUM_BOUNDING_OPTS: AtomicUsize = AtomicUsize::new(0);

fn bump_num_bounding_opts() {
    NUM_BOUNDING_OPTS.fetch_add(1, Ordering::SeqCst);
}

fn check_axis(
    origin: math::F3D,
    direction: math::F3D,
    min: math::F3D,
    max: math::F3D,
) -> (math::F3D, math::F3D) {
    let tmin_numerator = min - origin;
    let tmax_numerator = max - origin;

    let mut tmin = 0.0;
    let mut tmax = 0.0;

    if direction.abs() >= math::EPSILON {
        tmin = tmin_numerator / direction;
        tmax = tmax_numerator / direction;
    } else {
        tmin = tmin_numerator * math::INFINITY;
        tmax = tmax_numerator * math::INFINITY;
    }
    if tmin > tmax {
        let tmp = tmin;
        tmin = tmax;
        tmax = tmp;
    }
    (tmin, tmax)
}

#[derive(Clone, Debug)]
pub struct Group {
    pub props: Shape3D,
    pub val: Option<ShapeBox>,
    pub parent: Parent,
    pub shapes: Children,
    bounds: Bounds,
}

impl Group {
    pub fn from_shape(shape: ShapeBox) -> GroupRef {
        let g = Group {
            props: Shape3D::default(),
            val: Some(shape.clone()),
            parent: RefCell::new(Weak::new()),
            shapes: RefCell::new(Vec::new()),
            bounds: Bounds::default(),
        };
        let g_ref = Arc::new(g);
        g_ref
    }

    pub fn new() -> GroupRef {
        let g = Group {
            props: Shape3D::default(),
            val: None,
            parent: RefCell::new(Weak::new()),
            shapes: RefCell::new(Vec::new()),
            bounds: Bounds::default(),
        };
        let g_ref = Arc::new(g);
        g_ref
    }

    pub fn is_shape(&self) -> bool {
        self.val.is_some()
    }

    pub fn calculate_bounds(&self) -> Bounds {
        if self.shapes.borrow().is_empty() {
            return Bounds::default();
        }
        let mut xs: Vec<math::F3D> = vec![];
        let mut ys: Vec<math::F3D> = vec![];
        let mut zs: Vec<math::F3D> = vec![];

        for s in self.shapes.borrow().iter() {
            let sb = s.bounds();
            let group_min = s.get_transform() * sb.min;
            let group_max = s.get_transform() * sb.max;
            xs.push(group_min.x);
            xs.push(group_max.x);
            ys.push(group_min.y);
            ys.push(group_max.y);
            zs.push(group_min.z);
            zs.push(group_max.z);
        }
        let min_x = *xs.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        let max_x = *xs.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        let min_y = *ys.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        let max_y = *ys.iter().max_by(|a, b| a.total_cmp(b)).unwrap();
        let min_z = *zs.iter().min_by(|a, b| a.total_cmp(b)).unwrap();
        let max_z = *zs.iter().max_by(|a, b| a.total_cmp(b)).unwrap();

        Bounds {
            min: point(min_x, min_y, min_z),
            max: point(max_x, max_y, max_z),
        }
    }
}

impl Shape for Group {
    fn get_id(&self) -> String {
        if let Some(sbox) = &self.val {
            format!("g_{}", sbox.get_id())
        } else {
            format!("group_{}", self.props.id)
        }
    }
    fn get_transform(&self) -> &Matrix4 {
        if let Some(sbox) = &self.val {
            sbox.get_transform()
        } else {
            &self.props.transform
        }
    }
    fn set_transform(&mut self, t: &Matrix4) {
        self.props.transform = *t;
        self.calculate_bounds();
    }
    fn get_material(&self) -> &Material {
        if let Some(sbox) = &self.val {
            sbox.get_material()
        } else {
            &self.props.material
        }
    }
    fn set_material(&mut self, m: Material) {
        self.props.material = m;
    }

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        // Test group's bounding box first
        let bounds = self.bounds();
        let (xmin, xmax) = check_axis(ray.origin.x, ray.direction.x, bounds.min.x, bounds.max.x);
        let (ymin, ymax) = check_axis(ray.origin.y, ray.direction.y, bounds.min.y, bounds.max.y);
        let (zmin, zmax) = check_axis(ray.origin.z, ray.direction.z, bounds.min.z, bounds.max.z);

        let tmin = *vec![xmin, ymin, zmin]
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap();
        let tmax = *vec![xmax, ymax, zmax]
            .iter()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap();

        if tmin > tmax {
            println!("Bounding box optimization!");
            bump_num_bounding_opts();
            return vec![];
        }
        let mut res = vec![];
        for s in self.shapes.borrow().iter() {
            let xs = s.intersect(ray);
            res.extend(xs);
        }
        sort_intersections(&mut res);
        res
    }

    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if let Some(sbox) = &self.val {
            sbox.intersect(ray)
        } else {
            let t_ray = ray.transform(glm::inverse(&self.get_transform()));
            self.local_intersect(&t_ray)
            /*
            let mut res = vec![];
            for s in self.shapes.borrow().iter() {
                let xs = s.intersect(&t_ray);
                res.extend(xs);
            }
            sort_intersections(&mut res);
            res
            */
        }
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        panic!("local_normal_at should never be called on a group!");
    }

    fn bounds(&self) -> Bounds {
        self.calculate_bounds()
    }
}

impl fmt::Display for Group {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut sid = String::from("");
        if let Some(sbox) = &self.val {
            sid = sbox.get_id();
        }
        write!(f, "Group id {} Shape: {}", self.get_id(), sid)
    }
}

// default constructor
pub fn default_group() -> GroupRef {
    Group::new()
}

fn set_parent(child: &GroupRef, parent: &GroupRef) {
    // `child_node.parent` is set to weak reference to `parent_node`.
    //println!("adding {} to {}", child.get_id(), parent.get_id());
    *child.parent.borrow_mut() = Arc::downgrade(&parent);
}

pub fn add_child_shape(parent: &GroupRef, shape: ShapeBox) {
    // Make a GroupRef
    let g = Group::from_shape(shape);
    add_child_group(parent, &g);
}

pub fn add_child_group(parent: &GroupRef, child: &GroupRef) {
    set_parent(child, parent);
    parent.shapes.borrow_mut().push(child.clone());
}

/**
 * Helpers to limit syntax explosions
 */

pub fn set_transform(group: &mut GroupRef, transform: &Matrix4) {
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
        p = world_to_object(&get_parent(group).unwrap(), point);
    }
    if let Some(sbox) = group.val.as_ref() {
        glm::inverse(sbox.get_transform()) * p
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

/**
 * Important function here
 */
pub fn normal_at(group: &GroupRef, world_point: &Point) -> Vector {
    let local_point = world_to_object(group, world_point);
    let mut local_normal = vector_zero(); // is this the right default value?
    if let Some(shape_box) = &group.val {
        local_normal = shape_box.local_normal_at(local_point)
    }
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
        assert_eq!(s.get_id(), child.val.as_ref().unwrap().get_id());
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
        assert_eq!(
            xs[0].group.val.as_ref().unwrap().get_id(),
            String::from("sphere_s2")
        );
        assert_eq!(
            xs[1].group.val.as_ref().unwrap().get_id(),
            String::from("sphere_s2")
        );
        assert_eq!(
            xs[2].group.val.as_ref().unwrap().get_id(),
            String::from("sphere_s1")
        );
        assert_eq!(
            xs[3].group.val.as_ref().unwrap().get_id(),
            String::from("sphere_s1")
        );
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
        let g3 = Group::from_shape(Box::new(s));
        add_child_group(&g2, &g3);
        //let g3 = Group::new();
        //add_child_shape(&g3, Box::new(s));
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
        let g3 = Group::new();
        add_child_shape(&g3, Box::new(s));
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
        let g3 = Group::from_shape(Box::new(s));
        add_child_group(&g2, &g3);
        let n = normal_at(&g3, &point(1.7321, 1.1547, -5.5774));
        assert_eq_eps!(n, vector(0.2857, 0.4286, -0.8571));
    }

    #[test]
    #[should_panic]
    fn local_normal_at_illegal() {
        default_group().local_normal_at(point_zero());
    }

    //#[test]
    fn bounding_box_fits_children() {
        let g = default_group();
        let mut s1 = sphere();
        let mut s2 = sphere();
        s1.set_transform(&make_translation(-2.0, 0.0, -2.0));
        s2.set_transform(&make_translation(2.0, -2.0, 0.5));
        add_child_shape(&g, Box::new(s1));
        add_child_shape(&g, Box::new(s2));
        let bounds = g.bounds();
        assert_eq!(bounds.min.x, -3.0);
        assert_eq!(bounds.min.y, -3.0);
        assert_eq!(bounds.min.z, -3.0);
        assert_eq!(bounds.max.x, 3.0);
        assert_eq!(bounds.max.y, 1.0);
        assert_eq!(bounds.max.z, 1.5);
    }
}
