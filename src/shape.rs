use crate::intersection::Intersection;
use crate::materials::Material;
use crate::math::F3D;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::*;
use glm::*;
use std::fmt;

pub type ShapeBox = Box<dyn Shape>;

#[derive(Clone, Debug, PartialEq)]
pub struct Shape3D {
    pub id: String,
    pub transform: Matrix4,
    pub material: Material,
}

pub trait Shape: ShapeClone {
    fn get_id(&self) -> String;
    fn get_transform(&self) -> &Matrix4;
    fn get_material(&self) -> &Material;
    fn mut_get_transform(&mut self) -> &Matrix4;
    fn mut_get_material(&mut self) -> &Material;
    fn set_transform(&mut self, t: &Matrix4);
    fn set_material(&mut self, t: &Material);
    fn intersect(&self, ray: &Ray) -> Vec<Intersection>;
    fn intersection(&self, t: F3D) -> Intersection {
        Intersection {
            t,
            object: self.clone_box(),
        }
    }
    fn normal_at(&self, world_point: Point) -> Vector {
        let t = self.get_transform();
        let object_point = inverse(t) * world_point;
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let mut world_normal = transpose(&inverse(t)) * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
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

//impl<T> Sized for T where T: Shape {}
//impl<T> AsRef<T> for dyn Shape {
//fn as_ref(&self) -> &T {
//&self as dyn Shape
//}
//}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for ShapeBox {
    fn clone(&self) -> ShapeBox {
        self.clone_box()
    }
}

impl PartialEq for ShapeBox {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
    }
}

impl fmt::Debug for ShapeBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "object id: {:?}", self.get_id())
    }
}
