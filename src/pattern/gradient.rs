use crate::color::Color;
use crate::matrix::Matrix4;
use crate::pattern::{default_transform, Pattern};
use crate::tuple::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GradientPattern {
    a: Color,
    b: Color,
    transform: Matrix4,
}

pub fn gradient_pattern(a: Color, b: Color) -> GradientPattern {
    GradientPattern {
        a,
        b,
        transform: default_transform(),
    }
}

impl Pattern for GradientPattern {
    fn get_transform(&self) -> Matrix4 {
        self.transform
    }

    fn set_transform(&mut self, m: Matrix4) {
        self.transform = m;
    }

    fn pattern_at(&self, point: &Point) -> Color {
        let lerp = self.a.tuple() + (self.b.tuple() - self.a.tuple()) * point.x.fract();
        Color::from_tuple(&lerp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> GradientPattern {
        GradientPattern {
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
    fn gradient_linearly_interpolates_between_colors() {
        let p = setup();
        assert_eq!(p.pattern_at(&point_zero()), Color::white());
        assert_eq!(
            p.pattern_at(&point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            p.pattern_at(&point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
