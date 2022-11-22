use crate::color::Color;
use crate::math::f_equals;
use crate::matrix::Matrix4;
use crate::pattern::{default_transform, Pattern};
use crate::tuple::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CheckersPattern {
    a: Color,
    b: Color,
    transform: Matrix4,
}

pub fn checkers_pattern(a: Color, b: Color) -> CheckersPattern {
    CheckersPattern {
        a,
        b,
        transform: default_transform(),
    }
}

impl Pattern for CheckersPattern {
    fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, m: Matrix4) {
        self.transform = m;
    }

    fn pattern_at(&self, point: &Point) -> Color {
        if f_equals(
            (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0,
            0.0,
        ) {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> CheckersPattern {
        CheckersPattern {
            a: Color::white(),
            b: Color::black(),
            transform: default_transform(),
        }
    }

    #[test]
    fn pattern_creates() {
        let p = setup();
        assert_eq!(p.a, Color::white());
        assert_eq!(p.b, Color::black());
    }

    #[test]
    fn checkers_should_repeat_in_x() {
        let p = setup();
        assert_eq!(p.pattern_at(&point_zero()), Color::white());
        assert_eq!(p.pattern_at(&point(0.99, 0.0, 0.0)), Color::white());
        assert_eq!(p.pattern_at(&point(1.01, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let p = setup();
        assert_eq!(p.pattern_at(&point_zero()), Color::white());
        assert_eq!(p.pattern_at(&point(0.0, 0.0, 0.99)), Color::white());
        assert_eq!(p.pattern_at(&point(0.0, 0.0, 1.01)), Color::black());
    }
}
