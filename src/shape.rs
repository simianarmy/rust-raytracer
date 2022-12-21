use crate::bounds::*;
use crate::intersection::Intersection;
use crate::materials::Material;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::*;
use glm::*;
use std::fmt;
use std::sync::atomic::{AtomicUsize, Ordering};

pub type ShapeBox = Box<dyn Shape>;

pub fn get_unique_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[derive(Clone, Debug)]
pub struct Shape3D {
    pub id: String,
    pub transform: Matrix4,
    pub material: Material,
}

impl Shape3D {
    pub fn new(id: Option<String>) -> Shape3D {
        Shape3D {
            id: id.unwrap_or(get_unique_id().to_string()),
            transform: glm::identity(),
            material: Material::default(),
        }
    }
}

impl Default for Shape3D {
    fn default() -> Self {
        Shape3D::new(None)
    }
}

impl PartialEq for Shape3D {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

pub trait Shape: ShapeClone {
    fn get_id(&self) -> String;

    fn get_transform(&self) -> &Matrix4;
    fn set_transform(&mut self, t: &Matrix4);

    fn get_material(&self) -> &Material;
    fn set_material(&mut self, t: Material);

    fn local_intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let t_ray = ray.transform(inverse(&self.get_transform()));
        self.local_intersect(&t_ray)
    }

    fn local_normal_at(&self, world_point: Point) -> Vector;
    fn normal_at(&self, world_point: Point) -> Vector {
        let t = self.get_transform();
        let local_point = inverse(t) * world_point;
        let local_normal = self.local_normal_at(local_point);

        let mut world_normal = transpose(&inverse(t)) * local_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }

    fn bounds(&self) -> Bounds {
        Bounds::default()
    }
    fn parent_space_bounds(&self) -> Bounds {
        self.bounds().transform(self.get_transform())
    }
}

// Allow cloning boxed traits
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928#30353928
pub trait ShapeClone {
    fn clone_box(&self) -> ShapeBox;
}

impl<T> ShapeClone for T
where
    T: 'static + Shape + Clone,
{
    fn clone_box(&self) -> ShapeBox {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for ShapeBox {
    fn clone(&self) -> ShapeBox {
        self.clone_box()
    }
}

impl<'a> PartialEq for dyn Shape + 'a {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl fmt::Debug for dyn Shape {
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

#[derive(Clone, Debug, PartialEq)]
pub struct TestShape {
    props: Shape3D,
}
impl Shape for TestShape {
    fn get_id(&self) -> String {
        format!("testshape_{}", self.props.id)
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
    fn bounds(&self) -> Bounds {
        Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0))
    }
}

pub fn test_shape() -> TestShape {
    TestShape {
        props: Shape3D::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::*;
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
