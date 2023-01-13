use crate::color::Color;
use crate::matrix::Matrix4;
use crate::object::Object;
use crate::tuple::Point;
use glm;

pub mod checkers;
pub mod gradient;
pub mod ring;
pub mod stripe;
pub mod texture_map;

pub trait Pattern {
    fn get_transform(&self) -> Matrix4;

    fn set_transform(&mut self, m: Matrix4);

    fn pattern_at(&self, point: &Point) -> Color;

    fn pattern_at_shape(&self, obj: &Object, point: &Point) -> Color {
        let local_point = obj.world_to_object(point);
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
    TextureMap(texture_map::TextureMapPattern),
}

impl TPattern {
    pub fn default_transform() -> Matrix4 {
        glm::identity()
    }

    pub fn pattern_at_shape(&self, object: &Object, point: &Point) -> Color {
        match self {
            TPattern::Test(tp) => tp.pattern_at_shape(object, point),
            TPattern::Checkers(cp) => cp.pattern_at_shape(object, point),
            TPattern::Gradient(gp) => gp.pattern_at_shape(object, point),
            TPattern::Ring(rp) => rp.pattern_at_shape(object, point),
            TPattern::Stripe(sp) => sp.pattern_at_shape(object, point),
            TPattern::TextureMap(tm) => tm.pattern_at_shape(object, point),
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
    use crate::shapes::shape::*;
    use crate::shapes::sphere::*;
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
        let c = pattern.pattern_at_shape(&object, &point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transform() {
        let mut pattern = setup();
        let object = sphere();
        pattern.transform = make_scaling(2.0, 2.0, 2.0);
        let c = pattern.pattern_at_shape(&object, &point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_object_and_pattern_transform() {
        let mut pattern = setup();
        let mut object = sphere();
        object.set_transform(&make_scaling(2.0, 2.0, 2.0));
        pattern.transform = make_translation(0.5, 1.0, 1.5);
        let c = pattern.pattern_at_shape(&object, &point(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
