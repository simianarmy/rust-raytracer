use crate::color::Color;
use crate::math::f_equals;
use crate::matrix::Matrix4;
use crate::pattern::Pattern;
use crate::tuple::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StripePattern {
    a: Color,
    b: Color,
    transform: Matrix4,
}

pub fn stripe_pattern(a: Color, b: Color) -> StripePattern {
    StripePattern {
        a,
        b,
        transform: glm::identity(),
    }
}

impl Pattern for StripePattern {
    fn pattern_at(&self, point: &Point) -> Color {
        if f_equals(point.x.floor() % 2.0, 0.0) {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shape::*;
    use crate::sphere::sphere;
    use crate::transformation::*;

    fn setup() -> StripePattern {
        stripe_pattern(Color::white(), Color::black())
    }

    #[test]
    fn stripe_pattern_creates() {
        let p = setup();
        assert_eq!(p.a, Color::white());
        assert_eq!(p.b, Color::black());
    }

    #[test]
    fn stripe_pattern_is_constant_in_y() {
        let p = setup();
        assert_eq!(p.pattern_at(&point_zero()), Color::white());
        assert_eq!(p.pattern_at(&point_y()), Color::white());
    }

    #[test]
    fn stripe_pattern_is_constant_in_z() {
        let p = setup();
        assert_eq!(p.pattern_at(&point_zero()), Color::white());
        assert_eq!(p.pattern_at(&point_z()), Color::white());
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let p = setup();
        assert_eq!(p.pattern_at(&point_zero()), Color::white());
        assert_eq!(p.pattern_at(&point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(p.pattern_at(&point_x()), Color::black());
        assert_eq!(p.pattern_at(&point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(p.pattern_at(&point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.pattern_at(&point(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn stripes_with_object_transform() {
        let pattern = setup();
        let mut object = sphere();
        object.set_transform(&make_scaling(2.0, 2.0, 2.0));
        let sb = Box::new(object);
        let c = pattern.pattern_at_shape(sb, &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_pattern_transform() {
        let mut pattern = setup();
        let object = sphere();
        pattern.transform = make_scaling(2.0, 2.0, 2.0);
        let c = pattern.pattern_at_shape(Box::new(object), &point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_object_and_pattern_transform() {
        let mut pattern = setup();
        let mut object = sphere();
        object.set_transform(&make_scaling(2.0, 2.0, 2.0));
        pattern.transform = make_translation(0.5, 0.0, 0.0);
        let c = pattern.pattern_at_shape(Box::new(object), &point(2.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }
}
