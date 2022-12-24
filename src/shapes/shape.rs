use crate::shapes::cone::Cone;
use crate::shapes::cube::Cube;
use crate::shapes::cylinder::Cylinder;
use crate::shapes::plane::Plane;
use crate::shapes::sphere::Sphere;

pub enum Shape {
    None,
    Cube(Cube),
    Cone(Cone),
    Cylinder(Cylinder),
    Plane(Plane),
    Sphere(Sphere),
    TestShape(Sphere),
}
