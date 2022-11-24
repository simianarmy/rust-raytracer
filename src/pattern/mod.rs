use crate::color::Color;
use crate::matrix::Matrix4;
use crate::shape::*;
use crate::tuple::Point;
use glm;
use std::fmt;

pub trait Pattern: PatternClone {
    fn get_transform(&self) -> Matrix4;
    fn set_transform(&mut self, m: Matrix4);

    fn pattern_at(&self, point: &Point) -> Color;

    fn pattern_at_shape(&self, object: ShapeBox, point: &Point) -> Color {
        let object_point = glm::inverse(object.get_transform()) * point;
        let pattern_point = glm::inverse(&self.get_transform()) * object_point;
        self.pattern_at(&pattern_point)
    }
}

pub fn default_transform() -> Matrix4 {
    glm::identity()
}

pub mod checkers;
pub mod gradient;
pub mod ring;
pub mod stripe;

pub type PatternBox = Box<dyn Pattern>;

// Allow cloning boxed traits
// https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928#30353928
pub trait PatternClone {
    fn clone_box(&self) -> PatternBox;
}

impl<T> PatternClone for T
where
    T: 'static + Pattern + Clone,
{
    fn clone_box(&self) -> PatternBox {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for PatternBox {
    fn clone(&self) -> PatternBox {
        self.clone_box()
    }
}

impl<'a> PartialEq for dyn Pattern + 'a {
    fn eq(&self, other: &Self) -> bool {
        self.get_transform() == other.get_transform()
    }
}

impl fmt::Debug for dyn Pattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "pattern: {}", self.get_transform())
    }
}

#[derive(Clone)]
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
        transform: default_transform(),
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
        let c = pattern.pattern_at_shape(sb, &point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_pattern_transform() {
        let mut pattern = setup();
        let object = sphere();
        pattern.transform = make_scaling(2.0, 2.0, 2.0);
        let c = pattern.pattern_at_shape(Box::new(object), &point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }

    #[test]
    fn pattern_with_object_and_pattern_transform() {
        let mut pattern = setup();
        let mut object = sphere();
        object.set_transform(&make_scaling(2.0, 2.0, 2.0));
        pattern.transform = make_translation(0.5, 1.0, 1.5);
        let c = pattern.pattern_at_shape(Box::new(object), &point(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }
}
