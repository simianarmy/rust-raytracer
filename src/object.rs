/**
 * Represents an object in a World
 */
use crate::bounds::*;
use crate::intersection::*;
use crate::materials::Material;
use crate::math;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::shapes::{csg::*, cylinder::*, group::*, shape::*, sphere::*};
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
    pub transformation_inverse: Matrix4,
    pub transformation_inverse_transpose: Matrix4,
    pub material: Material,
    pub bounds: Bounds,
    pub has_shadow: bool,
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

    // TODO: Add remaining shape constructors here
    pub fn new_sphere() -> Self {
        Object {
            shape: Shape::Sphere(),
            bounds: Sphere::bounds(),
            ..Object::default()
        }
    }

    pub fn new_cylinder(min: math::F3D, max: math::F3D, closed: bool) -> Object {
        let mut o = Object {
            shape: Shape::Cylinder(Cylinder {
                minimum: min,
                maximum: max,
                closed,
            }),
            ..Object::default()
        };
        o.bounds = o.shape.bounds();
        o
    }

    pub fn new_group(children: Vec<Object>) -> Self {
        let mut o = Object {
            shape: Shape::Group(Group::new(children)),
            ..Object::default()
        };
        o.bounds = o.shape.bounds();
        o
    }

    pub fn new_csg(csg_op: CsgOp, left: &Object, right: &Object) -> Object {
        let mut o = Object {
            shape: Shape::Csg(Csg::new(csg_op, left, right)),
            ..Object::default()
        };
        o.bounds = o.shape.bounds();
        o
    }

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
    pub fn get_transformation_inverse(&self) -> &Matrix4 {
        &self.transformation_inverse
    }

    pub fn set_transform(&mut self, t: &Matrix4) {
        self.transform = *t;
        self.transformation_inverse = glm::inverse(&self.transform);
        self.transformation_inverse_transpose = glm::transpose(&self.transformation_inverse);
        self.bounds = self.shape.bounds().transform(&self.transform);
    }

    pub fn with_transformation(mut self, transformation: Matrix4) -> Self {
        self.set_transform(&transformation);

        self
    }

    pub fn with_material(mut self, material: Material) -> Self {
        self.set_material(material);

        self
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, t: Material) {
        // If I am a group, use set_group_material for now
        self.material = t;
    }

    pub fn intersect(&self, ray: &Ray) -> Intersections {
        let t_ray = ray.transform(inverse(&self.get_transform()));
        match self.shape() {
            Shape::Group(g) => g.intersects(&t_ray),
            Shape::Csg(c) => c.intersect(&t_ray),
            _ => Intersections::from_intersections(
                self.shape
                    .intersect(&t_ray)
                    .into_iter()
                    .map(|t| Intersection::with_uv(self, t.0, t.1, t.2))
                    .collect(),
            ),
        }
    }

    pub fn normal_at(&self, world_point: Point, is: Option<&Intersection>) -> Vector {
        let local_point = self.world_to_object(&world_point);
        let local_normal = self.shape().normal_at(&local_point, is);
        self.normal_to_world(&local_normal)
    }

    pub fn world_to_object(&self, world_point: &Point) -> Point {
        self.get_transformation_inverse() * world_point
    }

    pub fn normal_to_world(&self, normal: &Vector) -> Vector {
        let mut n = self.transformation_inverse_transpose * normal;
        n.w = 0.0; // crucial
        n.normalize()
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

    pub fn bounds(&self) -> Bounds {
        self.bounds
    }

    pub fn divide(self, threshold: usize) -> Self {
        Self {
            shape: self.shape.divide(threshold),
            ..self
        }
    }

    /**
     * Need to call this manually on group objects for transformations
     */
    pub fn transform(self, new_transformation: &Matrix4) -> Self {
        if let Shape::Group(g) = self.shape() {
            let children_group_builders =
                g.children().iter().map(GroupBuilder::from_object).collect();

            // We then create a new top GroupBuilder Node from which the new transformation is
            // applied.
            let group_builder = GroupBuilder::Node(
                Object::new_dummy().with_transformation(*new_transformation),
                children_group_builders,
            );

            // Convert back to a Group.
            group_builder.build(false, self.get_material())
        } else {
            let new_t = new_transformation * self.transform;
            self.with_transformation(new_t)
        }
    }

    /**
     * Extra function for groups to propagate materials to their children
     */
    pub fn set_group_material(self, new_material: Material) -> Self {
        if let Shape::Group(g) = self.shape() {
            let children_group_builders =
                g.children().iter().map(GroupBuilder::from_object).collect();

            let group_builder = GroupBuilder::Node(Object::new_dummy(), children_group_builders);

            // Convert back to a Group.
            group_builder.build(true, &new_material)
        } else {
            self
        }
    }
}

impl Default for Object {
    fn default() -> Self {
        Self {
            id: get_unique_id().to_string(),
            transform: glm::identity(),
            transformation_inverse: glm::identity(),
            transformation_inverse_transpose: glm::identity(),
            material: Material::default(),
            bounds: Bounds::default(),
            has_shadow: true,
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
            "\nid: {}\nmaterial: {:?}\ntransform: {}\ninverse: {}\ninverse transpose: {}\nbounds: {:?}",
            self.get_id(),
            self.get_material(),
            self.get_transform(),
            self.get_transformation_inverse(),
            self.transformation_inverse_transpose,
            self.bounds,
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
        let b = s.bounds();
        assert_eq!(b.min, point(0.5, -5.0, 1.0));
        assert_eq!(b.max, point(1.5, -1.0, 9.0));
    }
}
