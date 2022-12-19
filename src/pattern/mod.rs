use crate::color::Color;
use crate::group::*;
use crate::matrix::Matrix4;
use crate::shape::*;
use crate::tuple::Point;
use glm;

pub mod checkers;
pub mod gradient;
pub mod ring;
pub mod stripe;

pub trait Pattern {
    fn get_transform(&self) -> Matrix4;

    fn set_transform(&mut self, m: Matrix4);

    fn pattern_at(&self, point: &Point) -> Color;

    fn pattern_at_shape(&self, group: GroupRef, point: &Point) -> Color {
        let local_point = world_to_object(&group, point);
        let pattern_point = glm::inverse(&self.get_transform()) * local_point;
        self.pattern_at(&pattern_point)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TPattern {
    Test(TestPattern),
    Checkers(checkers::CheckersPattern),
    Gradient(gradient::GradientPattern),
    Ring(ring::RingPattern),
    Stripe(stripe::StripePattern),
}

impl TPattern {
    pub fn default_transform() -> Matrix4 {
        glm::identity()
    }
    // Used to get the trait object from the enum variant
    pub fn into_pattern(&self) -> Box<dyn Pattern> {
        match *self {
            TPattern::Test(tp) => Box::new(tp),
            TPattern::Checkers(cp) => Box::new(cp),
            TPattern::Gradient(gp) => Box::new(gp),
            TPattern::Ring(rp) => Box::new(rp),
            TPattern::Stripe(sp) => Box::new(sp),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TestPattern {
    transform: Matrix4,
}

impl Pattern for TestPattern {
    fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, m: Matrix4) {
        self.transform = m;
    }

    fn pattern_at(&self, point: &Point) -> Color {
        Color::from_tuple(point)
    }
}

pub fn test_pattern() -> TestPattern {
    TestPattern {
        transform: TPattern::default_transform(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sphere::sphere;
    use crate::transformation::*;
    use crate::tuple::*;

    fn setup() -> TestPattern {
        test_pattern()
    }

    #[test]
    fn assign_pattern_transformation() {
        let mut tp = setup();
        let t = make_translation(1.0, 0.0, 0.0);
        tp.set_transform(t);
        assert_eq!(tp.get_transform(), t);
    }

    #[test]
    fn pattern_with_object_transform() {
        let pattern = setup();
        let mut object = sphere();
        object.set_transform(&make_scaling(2.0, 2.0, 2.0));
        let sb = Box::new(object);
        let c = pattern.pattern_at_shape(Group::from_shape(sb), &point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transform() {
        let mut pattern = setup();
        let object = sphere();
        pattern.transform = make_scaling(2.0, 2.0, 2.0);
        let c =
            pattern.pattern_at_shape(Group::from_shape(Box::new(object)), &point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_object_and_pattern_transform() {
        let mut pattern = setup();
        let mut object = sphere();
        object.set_transform(&make_scaling(2.0, 2.0, 2.0));
        pattern.transform = make_translation(0.5, 1.0, 1.5);
        let c =
            pattern.pattern_at_shape(Group::from_shape(Box::new(object)), &point(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
