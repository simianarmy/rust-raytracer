use crate::math::F3D;

use crate::shape::Intersectable;

pub struct Intersection<'a, T: Intersectable + ?Sized> {
    pub t: F3D,
    pub object: &'a T,
}
