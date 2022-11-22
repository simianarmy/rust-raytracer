use crate::color::Color;
use crate::shape::*;
use crate::tuple::Point;

pub trait Pattern {
    fn pattern_at(&self, point: &Point) -> Color;

    fn pattern_at_shape(&self, object: ShapeBox, point: &Point) -> Color {
        let object_point = glm::inverse(object.get_transform()) * point;
        let pattern_point = glm::inverse(&self.get_transform()) * object_point;
        self.pattern_at(&pattern_point)
    }
}

pub mod stripe;
