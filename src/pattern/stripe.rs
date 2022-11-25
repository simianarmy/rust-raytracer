use crate::color::Color;
use crate::math::f_equals;
use crate::matrix::Matrix4;
use crate::pattern::{Pattern, TPattern};
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
        transform: TPattern::default_transform(),
    }
}

impl Pattern for StripePattern {
    fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, m: Matrix4) {
        self.transform = m;
    }

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
}
