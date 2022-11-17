use crate::intersection::Intersection;
use crate::materials::Material;
use crate::math::F3D;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::*;
use glm::*;

#[derive(Debug, PartialEq)]
pub struct Shape3D {
    pub id: String,
    pub transform: Matrix4,
    pub material: Material,
}

pub trait Intersectable {
    fn intersect(&self, r: &Ray) -> Vec<Intersection<Self>>
    where
        Self: Sized;

    fn intersection(&self, t: F3D) -> Intersection<Self> {
        Intersection { t, object: self }
    }
}

pub trait NormalAt {
    fn get_transform(&self) -> &Matrix4;
    fn normal_at(&self, world_point: Point) -> Vector {
        let t = self.get_transform();
        let object_point = inverse(t) * world_point;
        let object_normal = object_point - point(0.0, 0.0, 0.0);
        let mut world_normal = transpose(&inverse(t)) * object_normal;
        world_normal.w = 0.0;
        world_normal.normalize()
    }
}

/*
impl Shape for Shape3D {
    fn get_id(&self) -> String {
        self.id
    }
    fn get_transform(&self) -> &Matrix4 {
        &self.transform
    }
    fn get_material(&self) -> &Matrix4;
}
*/
