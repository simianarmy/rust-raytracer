use crate::bounds::*;
use crate::intersection::*;
use crate::math::F3D;
use crate::object::Object;
use crate::ray::Ray;
use crate::shapes::{cone, cube, cylinder, group, plane, smooth_triangle, sphere, triangle};
use crate::tuple::*;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub enum Shape {
    None,
    Cube(),
    Cone(cone::Cone),
    Cylinder(cylinder::Cylinder),
    Group(group::Group),
    Plane(),
    Sphere(),
    Triangle(triangle::Triangle),
    SmoothTriangle(smooth_triangle::SmoothTriangle),
    TestShape(TestShape),
}

fn add_uvs_to_ts(ts: &Vec<F3D>) -> Vec<(F3D, F3D, F3D)> {
    ts.into_iter().map(|t| (*t, 0.0, 0.0)).collect()
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
            Shape::Triangle(_) => "triangle",
            Shape::SmoothTriangle(_) => "smooth_triangle",
            Shape::TestShape(_) => "test_shape",
            Shape::None => "none",
        }
    }

    pub fn intersect<'a>(&'a self, ray: &Ray) -> Vec<(F3D, F3D, F3D)> {
        match self {
            Shape::Cube() => add_uvs_to_ts(&cube::Cube::local_intersect(ray)),
            Shape::Cone(c) => add_uvs_to_ts(&c.local_intersect(ray)),
            Shape::Cylinder(c) => add_uvs_to_ts(&c.local_intersect(ray)),
            Shape::Plane() => add_uvs_to_ts(&plane::Plane::local_intersect(ray)),
            Shape::Sphere() => add_uvs_to_ts(&sphere::Sphere::local_intersect(ray)),
            Shape::Triangle(t) => add_uvs_to_ts(&t.local_intersect(ray)),
            Shape::SmoothTriangle(t) => t.local_intersect(ray),
            Shape::TestShape(c) => add_uvs_to_ts(&c.local_intersect(ray)),
            Shape::Group(_) => unreachable!("Group::intersect from Shape"),
            Shape::None => unreachable!("Shape::None::intersect"),
        }
    }

    pub fn normal_at(&self, point: &Point, is: Option<&Intersection>) -> Vector {
        match self {
            Shape::Cube() => cube::Cube::local_normal_at(point),
            Shape::Cone(c) => c.local_normal_at(point),
            Shape::Cylinder(c) => c.local_normal_at(point),
            Shape::Plane() => plane::Plane::local_normal_at(point),
            Shape::Sphere() => sphere::Sphere::local_normal_at(point),
            Shape::Triangle(t) => t.local_normal_at(point),
            Shape::SmoothTriangle(t) => t.local_normal_at(point, is),
            Shape::TestShape(c) => c.local_normal_at(point),
            Shape::Group(g) => g.normal_at(point),
            Shape::None => unreachable!("Shape::None::normal_at"),
        }
    }

    pub fn bounds(&self) -> Bounds {
        match self {
            Shape::Cube() => cube::Cube::bounds(),
            Shape::Cone(c) => c.bounds(),
            Shape::Cylinder(c) => c.bounds(),
            Shape::Plane() => plane::Plane::bounds(),
            Shape::Sphere() => sphere::Sphere::bounds(),
            Shape::Triangle(t) => t.bounds(),
            Shape::SmoothTriangle(t) => t.bounds(),
            Shape::TestShape(c) => c.bounds(),
            Shape::Group(g) => g.bounds(),
            Shape::None => Bounds::default(),
        }
    }

    pub fn divide(self, threshold: usize) -> Self {
        match self {
            Shape::Group(g) => Shape::Group(g.divide(threshold)),
            _ => self,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TestShape {
    // seems to be the only way to save this property?
    ray: Arc<Mutex<Option<Ray>>>,
}
impl TestShape {
    pub fn local_intersect<'a>(&'a self, ray: &Ray) -> Vec<F3D> {
        let mut reference = self.ray.lock().unwrap();
        *reference = Some(*ray);
        vec![]
    }

    pub fn local_normal_at(&self, _point: &Point) -> Vector {
        point_zero()
    }

    pub fn bounds(&self) -> Bounds {
        Bounds::new(point(-1.0, -1.0, -1.0), point(1.0, 1.0, 1.0))
    }

    pub fn ray(&self) -> Option<Ray> {
        *self.ray.lock().unwrap()
    }
}

pub fn test_shape() -> Object {
    let mut o = Object::new(None);
    o.shape = Shape::TestShape(TestShape {
        ray: Arc::new(Mutex::new(None)),
    });
    o
}
