use crate::color::Color;
use crate::math::f_equals;
use crate::matrix::Matrix4;
use crate::pattern::{default_transform, Pattern};
use crate::tuple::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RingPattern {
    a: Color,
    b: Color,
    transform: Matrix4,
}

pub fn ring_pattern(a: Color, b: Color) -> RingPattern {
    RingPattern {
        a,
        b,
        transform: default_transform(),
    }
}

impl Pattern for RingPattern {
    fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, m: Matrix4) {
        self.transform = m;
    }

    fn pattern_at(&self, point: &Point) -> Color {
        if f_equals((point.x.powi(2) + point.z.powi(2)).sqrt() % 2.0, 0.0) {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> RingPattern {
        RingPattern {
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
    fn ring_should_use_x_and_z() {
        let p = setup();
        assert_eq!(p.pattern_at(&point_zero()), Color::white());
        assert_eq!(p.pattern_at(&point_x()), Color::black());
        assert_eq!(p.pattern_at(&point_z()), Color::black());
        assert_eq!(p.pattern_at(&point(0.708, 0.0, 0.708)), Color::black());
    }
}
