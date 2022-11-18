use crate::intersection::Intersection;
use crate::materials::Material;
use crate::math::F3D;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::*;
use glm::*;

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
    fn clone_box(&self) -> Box<dyn Shape>;
}

impl<T> ShapeClone for T
where
    T: 'static + Shape + Clone,
{
    fn clone_box(&self) -> Box<dyn Shape> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Box<dyn Shape> {
        self.clone_box()
    }
}
