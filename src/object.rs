/**
 * Represents an object in a World
 */
use crate::bounds::*;
use crate::intersection::Intersection;
use crate::materials::Material;
use crate::math::F3D;
use crate::matrix::Matrix4;
use crate::ray::Ray;
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
            transform: glm::identity(),
            material: Material::default(),
            bounds: Bounds::default(),
            shape: Shape::None,
        }
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

    pub fn get_material(&self) -> &Material {
        &self.material
    }
    pub fn set_material(&mut self, t: Material) {
        self.material = t;
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let t_ray = ray.transform(inverse(&self.get_transform()));
        self.shape
            .intersect(&t_ray)
            .into_iter()
            .map(|t| Intersection::new(self, t))
            .collect()
    }

    pub fn normal_at(&self, world_point: Point) -> Vector {
        let t = self.get_transform();
        let local_point = inverse(t) * world_point;
        let local_normal = self.shape.normal_at(local_point);
        let mut world_normal = transpose(&inverse(t)) * local_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }

    pub fn parent_space_bounds(&self) -> Bounds {
        self.bounds.transform(self.get_transform())
    }
}

impl Default for Object {
    fn default() -> Self {
        Object::new(None)
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
    use crate::shapes::shape::*;
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
