/**
 * Code from https://github.com/ahamez/ray-tracer
 *
 * After spending way too much time trying to implement a bidirectional tree myself with Arc,
 * Refcell, etc., this looked like a nice clean solution
 */
use crate::{
    bounds::Bounds,
    intersection::Intersections,
    matrix::Matrix4,
    object::Object,
    ray::Ray,
    shapes::shape::Shape,
    tuple::{Point, Vector},
};
//use serde::{Deserialize, Serialize};

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug, PartialEq)]
pub struct Group {
    bounds: Bounds,
    children: Vec<Object>,
}

/* ---------------------------------------------------------------------------------------------- */

impl Group {
    pub fn new(children: Vec<Object>) -> Self {
        let bounds = Group::mk_bounding_box(&children);

        Self { children, bounds }
    }

    pub fn intersects<'a>(&'a self, ray: &Ray) -> Intersections<'a> {
        let mut xs = Intersections::new();
        if self.bounds().intersects(ray) {
            for child in self.children() {
                xs.extend(&child.intersect(ray));
            }
        }
        // sort results here
        xs.sort_intersections()
    }

    pub fn normal_at(&self, _object_point: &Point) -> Vector {
        unreachable!()
    }

    pub fn children(&self) -> &Vec<Object> {
        &self.children
    }

    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    fn partition(self) -> Self {
        let mut left_children = Vec::with_capacity(self.children.len());
        let mut right_children = Vec::with_capacity(self.children.len());
        let mut children = Vec::with_capacity(self.children.len());

        let (left_bbox, right_bbox) = self.bounds.split();
        for child in self.children {
            if left_bbox.contains_bounds(&child.bounds) {
                left_children.push(child);
            } else if right_bbox.contains_bounds(&child.bounds) {
                right_children.push(child);
            } else {
                // All children that are neither contained in the left nor right
                // sub bounding box stay at this level.
                children.push(child);
            }
        }

        if !left_children.is_empty() {
            let left_child =
                Object::new_dummy().with_shape(Shape::Group(Group::new(left_children)));
            children.push(left_child);
        }

        if !right_children.is_empty() {
            let right_child =
                Object::new_dummy().with_shape(Shape::Group(Group::new(right_children)));
            children.push(right_child);
        }

        Self { children, ..self }
    }

    pub fn divide(self, threshold: usize) -> Self {
        let g = if self.children.len() <= threshold {
            self
        } else {
            self.partition()
        };

        let children = g
            .children
            .into_iter()
            .map(|child| child.divide(threshold))
            .collect();

        Self { children, ..g }
    }

    fn mk_bounding_box(children: &[Object]) -> Bounds {
        let mut bbox = Bounds::default();
        for child in children {
            bbox.add_bounds(&child.bounds);
        }

        bbox
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[derive(Clone, Debug)]
pub enum GroupBuilder {
    Leaf(Object),
    Node(Object, Vec<GroupBuilder>),
}

impl GroupBuilder {
    pub fn build(self) -> Object {
        GroupBuilder::rec(self, &glm::identity())
    }

    fn rec(gb: Self, transform: &Matrix4) -> Object {
        match gb {
            GroupBuilder::Leaf(o) => o.transform(transform),
            GroupBuilder::Node(group, children) => {
                let child_transform = transform * group.get_transform();
                let new_children = children
                    .into_iter()
                    .map(|child| GroupBuilder::rec(child, &child_transform))
                    .collect();

                group
                    .with_shape(Shape::Group(Group::new(new_children)))
                    // The group transformation has been applied to all children.
                    // To make sure it's not propagated again in future usages of this
                    // newly created group, we set it to an Id transformation which is
                    // "neutral".
                    .with_transformation(glm::identity())
            }
        }
    }

    pub fn from_object(object: &Object) -> Self {
        match object.shape() {
            Shape::Group(g) => GroupBuilder::Node(
                object.clone(),
                g.children()
                    .iter()
                    .filter_map(|child| match child.shape() {
                        Shape::Group(g) => {
                            if g.children().is_empty() {
                                None
                            } else {
                                Some(GroupBuilder::from_object(child))
                            }
                        }
                        _ => Some(GroupBuilder::from_object(child)),
                    })
                    .collect(),
            ),
            _other => GroupBuilder::Leaf(object.clone()),
        }
    }
}

pub fn from_shape(s: &Shape) -> Option<&Group> {
    match s {
        Shape::Group(g) => Some(g),
        _ => None,
    }
}

/* ---------------------------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        shapes::cylinder::*, shapes::shape, shapes::sphere::*, transformation::*, tuple::*,
    };

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let group = Object::new_group(vec![]);
        let ray = Ray::new(point_zero(), vector_z());

        let xs = group.intersect(&ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_a_ray_with_an_non_empty_group() {
        let s1 = sphere();
        let mut s2 = sphere();
        s2.set_transform(&make_translation(0.0, 0.0, -3.0));
        let mut s3 = sphere();
        s3.set_transform(&make_translation(5.0, 0.0, 0.0));

        let group = Object::new_group(vec![s1.clone(), s2.clone(), s3]);
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector_z());

        let xs = group.intersect(&ray);

        assert_eq!(xs.len(), 4);
        assert_eq!(*xs[0].object, s2);
        assert_eq!(*xs[1].object, s2);
        assert_eq!(*xs[2].object, s1);
        assert_eq!(*xs[3].object, s1);
    }

    #[test]
    fn intersecting_a_ray_with_a_nested_non_empty_group() {
        {
            let s1 = Object::new_sphere();
            let mut s2 = Object::new_sphere();
            s2.set_transform(&make_translation(0.0, 0.0, -3.0));
            let mut s3 = Object::new_sphere();
            s3.set_transform(&make_translation(5.0, 0.0, 0.0));

            let group_1 = Object::new_group(vec![s1.clone(), s2.clone(), s3.clone()]);
            let group_2 = Object::new_group(vec![group_1]);

            let ray = Ray::new(point(0.0, 0.0, -5.0), vector_z());

            let xs = group_2.intersect(&ray);

            assert_eq!(xs.len(), 4);
            assert_eq!(*xs[0].object, s2);
            assert_eq!(*xs[1].object, s2);
            assert_eq!(*xs[2].object, s1);
            assert_eq!(*xs[3].object, s1);
        }
        {
            let s1 = Object::new_sphere();
            let mut s2 = Object::new_sphere();
            s2.set_transform(&make_translation(0.0, 0.0, -3.0));
            let mut s3 = Object::new_sphere();
            s3.set_transform(&make_translation(5.0, 0.0, 0.0));

            let group_1 = Object::new_group(vec![s1.clone(), s3]);
            let group_2 = Object::new_group(vec![group_1, s2.clone()]);

            let ray = Ray::new(point(0.0, 0.0, -5.0), vector_z());

            let xs = group_2.intersect(&ray);

            assert_eq!(xs.len(), 4);
            assert_eq!(*xs[0].object, s2);
            assert_eq!(*xs[1].object, s2);
            assert_eq!(*xs[2].object, s1);
            assert_eq!(*xs[3].object, s1);
        }
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut s = Object::new_sphere();
        s.set_transform(&make_translation(5.0, 0.0, 0.0));

        let group = Object::new_group(vec![s]).transform(&make_scaling(2.0, 2.0, 2.0));

        let ray = Ray::new(point(10.0, 0.0, -10.0), vector_z());

        let xs = group.intersect(&ray);

        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn intersecting_a_nested_transformed_group() {
        {
            let mut s = Object::new_sphere();
            s.set_transform(&make_translation(5.0, 0.0, 0.0));

            let group_1 = Object::new_group(vec![s]);
            let group_2 = Object::new_group(vec![group_1]).transform(&make_scaling(2.0, 2.0, 2.0));

            let ray = Ray {
                origin: point(10.0, 0.0, -10.0),
                direction: vector_z(),
            };

            let xs = group_2.intersect(&ray);

            assert_eq!(xs.len(), 2);
        }
        {
            let mut s = Object::new_sphere();
            s.set_transform(&make_translation(5.0, 0.0, 0.0));

            let mut group_1 = Object::new_group(vec![s]);
            group_1.set_transform(&make_scaling(2.0, 2.0, 2.0));
            let group_2 = Object::new_group(vec![group_1]);

            let ray = Ray {
                origin: point(10.0, 0.0, -10.0),
                direction: vector_z(),
            };

            let xs = group_2.intersect(&ray);

            assert_eq!(xs.len(), 2);
        }
    }

    #[test]
    fn transformations_are_propagated() {
        let mut s = Object::new_sphere();
        s.set_transform(
            &(make_rotation_y(glm::half_pi())
                * make_scaling(2.0, 2.0, 2.0)
                * make_translation(5.0, 0.0, 0.0)),
        );

        let expected_transformation = s.get_transform();

        // With one group
        {
            let mut s = Object::new_sphere();
            s.set_transform(&make_translation(5.0, 0.0, 0.0));
            let t = make_scaling(2.0, 2.0, 2.0) * make_rotation_y(glm::half_pi());
            let g2t = Object::new_group(vec![s]).transform(&t);

            // Retrieve the s with the baked-in group transform.
            let group_s = from_shape(g2t.shape()).unwrap().children[0].clone();

            assert_eq!(group_s.get_transform(), expected_transformation);
        }
        {
            let mut s = Object::new_sphere();
            s.set_transform(&make_translation(5.0, 0.0, 0.0));
            let g2 = Object::new_group(vec![s])
                .transform(&(make_scaling(2.0, 2.0, 2.0) * make_rotation_y(glm::half_pi())));
            let g1 = Object::new_group(vec![g2]);

            // Retrieve the s with the baked-in group transform.
            let group_g2 = from_shape(g1.shape()).unwrap().children[0].clone();
            let group_s = from_shape(group_g2.shape()).unwrap().children[0].clone();

            assert_eq!(group_s.get_transform(), expected_transformation);
        }
        // With three nested groups, only one being transformed
        {
            let mut s = Object::new_sphere();
            s.set_transform(&make_translation(5.0, 0.0, 0.0));
            let mut g2 = Object::new_group(vec![s]);
            g2.set_transform(
                &(make_rotation_y(std::f64::consts::PI / 2.0) * make_scaling(2.0, 2.0, 2.0)),
            );
            let g1 = Object::new_group(vec![g2]);
            let g0 = Object::new_group(vec![g1]);

            // Retrieve the s with the baked-in group transform.
            let group_g1 = from_shape(g0.shape()).unwrap().children[0].clone();
            let group_g2 = from_shape(group_g1.shape()).unwrap().children[0].clone();
            let group_s = from_shape(group_g2.shape()).unwrap().children[0].clone();

            assert_eq!(group_s.get_transform(), expected_transformation);
        }
        // With two nested groups with transformations in both
        {
            let mut s = Object::new_sphere();
            s.set_transform(&make_translation(5.0, 0.0, 0.0));
            let g2 = Object::new_group(vec![s])
                .transform(&(make_rotation_y(std::f64::consts::PI / 2.0)));
            let g1 = Object::new_group(vec![g2]).transform(&make_scaling(2.0, 2.0, 2.0));

            // Retrieve the s with the baked-in group transform.
            let group_g2 = from_shape(g1.shape()).unwrap().children[0].clone();
            let group_s = from_shape(group_g2.shape()).unwrap().children[0].clone();

            assert_eq!(group_s.get_transform(), expected_transformation);
        }
    }

    #[test]
    fn a_group_has_a_bounding_box_that_contains_its_children() {
        let mut s = Object::new_sphere();
        s.set_transform(&(make_translation(2.0, 5.0, -3.0) * make_scaling(2.0, 2.0, 2.0)));
        let mut c = cylinder(-2.0, 2.0, true);
        c.set_transform(&(make_translation(-4.0, -1.0, 4.0) * make_scaling(0.5, 1.0, 0.5)));

        let g = Object::new_group(vec![s, c]);

        assert_eq!(g.bounds().min, point(-4.5, -3.0, -5.0));
        assert_eq!(g.bounds().max, point(4.0, 7.0, 4.5));
    }

    #[test]
    fn intersecting_a_ray_with_doesnt_test_children_if_bbox_is_missed() {
        let ts = shape::test_shape();

        let g = Object::new_group(vec![ts]);

        let ray = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector_y(),
        };

        g.intersect(&ray);

        let s = from_shape(g.shape()).unwrap().children()[0].clone();
        let ts = match s.shape() {
            Shape::TestShape(testshape) => Some(testshape),
            _ => None,
        };

        assert!(ts.unwrap().ray().is_none());
    }

    #[test]
    fn intersecting_a_ray_with_tests_children_if_bbox_is_hit() {
        let ts = shape::test_shape();

        let g = Object::new_group(vec![ts]);

        let ray = Ray {
            origin: point(0.0, 0.0, -5.0),
            direction: vector_z(),
        };

        g.intersect(&ray);

        let s = from_shape(g.shape()).unwrap().children()[0].clone();
        let ts = match s.shape() {
            Shape::TestShape(testshape) => Some(testshape),
            _ => None,
        };

        assert!(ts.unwrap().ray().is_some());
    }

    #[test]
    fn partitioning_a_group_s_children() {
        let s1 = Object::new_sphere().with_transformation(make_translation(-2.0, 0.0, 0.0));
        let s2 = Object::new_sphere().with_transformation(make_translation(2.0, 0.0, 0.0));
        let s3 = Object::new_sphere();

        let g = Object::new_group(vec![s1.clone(), s2.clone(), s3.clone()]);

        let g = from_shape(g.shape()).unwrap().clone().partition();
        let g_children = g.children();

        assert_eq!(g_children[0], s3);
        // left child
        assert_eq!(from_shape(g_children[1].shape()).unwrap().children()[0], s1);
        // right child
        assert_eq!(from_shape(g_children[2].shape()).unwrap().children()[0], s2);
    }
}
