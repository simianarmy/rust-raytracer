/**
 * Represents an object in a World
 */
use crate::bounds::*;
use crate::intersection::*;
use crate::materials::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shapes::group::*;
use crate::shapes::shape::*;
use crate::tuple::*;
use glm::*;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

pub fn get_unique_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone)]
pub struct Object {
    pub id: String,
    pub transform: Matrix4,
    pub material: Material,
    pub bounds: Bounds,
    pub shape: Shape,
}

impl Object {
    pub fn new(id: Option<String>) -> Object {
        Object {
            id: id.unwrap_or(get_unique_id().to_string()),
            ..Object::default()
        }
    }

    pub fn new_dummy() -> Self {
        Object::new(Some(String::from("dummy")))
    }

    pub fn new_group(children: Vec<Object>) -> Self {
        let children_group_builders = children
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
            .collect();
        let group_builder = GroupBuilder::Node(Object::new_dummy(), children_group_builders);
        let object = group_builder.build();

        Object {
            bounds: object.shape.bounds(),
            ..object
        }
    }

    // TODO: Add remaining shape constructors here
    //
    pub fn with_shape(mut self, shape: Shape) -> Self {
        self.shape = shape;
        self.bounds = self.shape.bounds();

        self
    }

    pub fn get_id(&self) -> String {
        format!("{}_{}", self.shape.get_id(), self.id)
    }

    pub fn get_transform(&self) -> &Matrix4 {
        &self.transform
    }
    pub fn set_transform(&mut self, t: &Matrix4) {
        self.transform = *t;
        self.bounds = self.shape.bounds();
    }

    pub fn with_transformation(mut self, transformation: Matrix4) -> Self {
        self.set_transform(&transformation);
        //self.transformation_inverse = self.transformation.invert();
        //self.transformation_inverse_transpose = self.transformation_inverse.transpose();
        //self.bounding_box = self.shape_bounds().transform(&self.transformation);

        self
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }
    pub fn set_material(&mut self, t: Material) {
        self.material = t;
    }

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Intersections<'a> {
        let t_ray = ray.transform(inverse(&self.get_transform()));
        Intersections::from_intersections(
            self.shape
                .intersect(&t_ray)
                .into_iter()
                .map(|t| Intersection::new(self, t))
                .collect(),
        )
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        /* pre-groups
        let t = self.get_transform();
        let local_point = inverse(t) * world_point;
        let local_normal = self.shape.normal_at(local_point);
        let mut world_normal = transpose(&inverse(t)) * local_normal;
        world_normal.w = 0.0;
        world_normal.normalize();
        */

        let local_point = self.world_to_object(world_point);
        let local_normal = self.shape.normal_at(local_point);
        self.normal_to_world(&local_normal)
    }

    pub fn parent_space_bounds(&self) -> Bounds {
        self.bounds.transform(self.get_transform())
    }

    pub fn shape(&self) -> &Shape {
        &self.shape
    }

    pub fn is_shape(&self) -> bool {
        match self.shape() {
            Shape::None => false,
            _ => true,
        }
    }

    pub fn world_to_object(&self, point: Point) -> Point {
        let p = match self.shape() {
            Shape::Group(g) => {
                if let Some(gref) = g.get_parent() {
                    let gp = gref.val.world_to_object(point);
                    gp
                } else {
                    point
                }
            }
            _ => point,
        };
        //if group.is_leaf() {
        //glm::inverse(group.val.get_transform()) * p
        //} else {
        //glm::inverse(group.get_transform()) * p
        //}
        glm::inverse(self.get_transform()) * p
    }

    pub fn normal_to_world(&self, normal: &Vector) -> Vector {
        let mut n = glm::inverse(self.get_transform()).transpose() * normal;
        n.w = 0.0;
        n = n.normalize();

        match &self.shape {
            Shape::Group(g) => {
                if let Some(gr) = g.get_parent() {
                    gr.val.normal_to_world(&n)
                } else {
                    n
                }
            }
            _ => n,
        }
    }

    pub fn divide(self, threshold: usize) -> Self {
        Self {
            shape: self.shape.divide(threshold),
            ..self
        }
    }

    /**
     * For GroupBuilder
     */
    pub fn transform(self, new_transformation: &Matrix4) -> Self {
        match self.shape() {
            Shape::Group(g) => {
                // Each time a Group is transformed, we convert it back to a GroupBuilder,
                // which is easier to manipulate. It's not the most efficient, but as this
                // is only peformed when constructing objects of a world, it has no impact on
                // the rendering itself.
                let children_group_builders =
                    g.children().iter().map(GroupBuilder::from_object).collect();

                // We then create a new top GroupBuilder Node from which the new transformation is
                // applied.
                let group_builder = GroupBuilder::Node(
                    Object::new(Some(String::from("dummy")))
                        .with_transformation(*new_transformation),
                    children_group_builders,
                );

                // Convert back to a Group.
                group_builder.build()
            }
            _other_shape => {
                let new_transformation = *new_transformation * self.transform;
                self.with_transformation(new_transformation)
            }
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Self {
            id: get_unique_id().to_string(),
            transform: glm::identity(),
            material: Material::default(),
            bounds: Bounds::default(),
            shape: Shape::None,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl fmt::Debug for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "shape: {}\nmaterial: {:?}\ntransform: {}",
            self.get_id(),
            self.get_material(),
            self.get_transform()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shapes::sphere::*;
    use crate::transformation::*;

    #[test]
    fn shape_instances_have_unique_ids() {
        let s1 = test_shape();
        let s2 = test_shape();
        assert_ne!(s1.get_id(), s2.get_id());
    }

    #[test]
    fn test_default_transform_is_identity() {
        let s = test_shape();
        let ident: Matrix4 = identity();
        assert_eq!(*s.get_transform(), ident);
    }

    #[test]
    fn test_changing_transform() {
        let mut s = test_shape();
        let t = make_translation(2.0, 3.0, 4.0);
        s.set_transform(&t);
        assert_eq!(*s.get_transform(), t);
    }

    /*
    #[test]
    fn intersect_scaled_shape_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = test_shape();
        s.set_transform(&make_scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(s.saved_ray.origin, point(0.0, 0.0, -2.5));
    }
    */

    #[test]
    fn intersect_translated_shape_with_ray() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector(0.0, 0.0, 1.0));
        let mut s = test_shape();
        s.set_transform(&make_translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn querying_shapes_bounding_box_in_its_parents_space() {
        let mut s = sphere();
        s.set_transform(&(make_translation(1.0, -3.0, 5.0) * make_scaling(0.5, 2.0, 4.0)));
        let b = s.parent_space_bounds();
        assert_eq!(b.min, point(0.5, -5.0, 1.0));
        assert_eq!(b.max, point(1.5, -1.0, 9.0));
    }
}
