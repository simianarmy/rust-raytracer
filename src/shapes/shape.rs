use crate::bounds::*;
use crate::intersection::Intersection;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::cone::Cone;
use crate::shapes::cylinder::Cylinder;
use crate::shapes::group::*;
use crate::tuple::*;

#[derive(Clone, Debug)]
pub enum Shape {
    None,
    Cube(),
    Cone(Cone),
    Cylinder(Cylinder),
    Group(Group),
    Plane(),
    Sphere(),
    TestShape(TestShape),
}

#[derive(Clone, Debug, PartialEq)]
pub struct TestShape {}
impl TestShape {
    pub fn local_intersect(&self, _ray: &Ray) -> Vec<Intersection> {
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
    let o = Object::new(Some("test_shape".to_string()));
    o.shape = Shape::TestShape(TestShape {});
    o
}
