use crate::intersection::Intersection;
use crate::math::F3D;
use crate::ray::Ray;

pub trait Intersectable {
    fn intersect(&self, r: Ray) -> Vec<Intersection<Self>>
    where
        Self: Sized;

    fn intersection(&self, t: F3D) -> Intersection<Self> {
        Intersection { t, object: self }
    }
}
