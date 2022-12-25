use crate::bounds::*;
use crate::intersection::Intersection;
use crate::math::F3D;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::{cone, cube, cylinder, group, plane, sphere};
use crate::tuple::*;
use glm::*;

#[derive(Clone, Debug)]
pub enum Shape {
    None,
    Cube(),
    Cone(cone::Cone),
    Cylinder(cylinder::Cylinder),
    Group(group::Group),
    Plane(),
    Sphere(),
    TestShape(TestShape),
}

impl Shape {
    pub fn get_id(&self) -> &str {
        match self {
            Shape::Cube() => "cube",
            Shape::Cone(_) => "cone",
            Shape::Cylinder(_) => "cylinder",
            Shape::Group(_) => "group",
            Shape::Plane() => "plane",
            Shape::Sphere() => "sphere",
            Shape::TestShape(_) => "test_shape",
            Shape::None => "none",
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<F3D> {
        match self {
            Shape::Cube() => cube::Cube::local_intersect(ray),
            Shape::Cone(c) => c.local_intersect(ray),
            Shape::Cylinder(c) => c.local_intersect(ray),
            Shape::Group(g) => vec![], // g.local_intersect(ray),
            Shape::Plane() => plane::Plane::local_intersect(ray),
            Shape::Sphere() => sphere::Sphere::local_intersect(ray),
            Shape::TestShape(c) => c.local_intersect(ray),
            Shape::None => unreachable!("Shape::None::intersect"),
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        match self {
            Shape::Cube() => cube::Cube::local_normal_at(point),
            Shape::Cone(c) => c.local_normal_at(point),
            Shape::Cylinder(c) => c.local_normal_at(point),
            Shape::Plane() => plane::Plane::local_normal_at(point),
            Shape::Sphere() => sphere::Sphere::local_normal_at(point),
            Shape::TestShape(c) => c.local_normal_at(point),
            Shape::Group(g) => panic!("implement me"),
            Shape::None => unreachable!("Shape::None::normal_at"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestShape {}
impl TestShape {
    pub fn local_intersect(&self, _ray: &Ray) -> Vec<F3D> {
        //self.saved_ray = ray;
        vec![]
    }

    pub fn local_normal_at(&self, _point: Point) -> Vector {
        point_zero()
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0))
    }
}

pub fn test_shape() -> Object {
    let mut o = Object::new(Some("test_shape".to_string()));
    o.shape = Shape::TestShape(TestShape {});
    o
}
