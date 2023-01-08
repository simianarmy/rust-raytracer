use crate::color::Color;
use crate::math::*;
use crate::matrix::Matrix4;
use crate::pattern::{Pattern, TPattern};
use crate::tuple::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVPattern {
    Checkers(UVCheckers),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UVMap {
    Spherical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct UVCheckers {
    width: F3D,
    height: F3D,
    a: Color,
    b: Color,
}

impl UVCheckers {
    pub fn new(width: F3D, height: F3D, a: Color, b: Color) -> UVCheckers {
        UVCheckers {
            width,
            height,
            a,
            b,
        }
    }

    pub fn uv_pattern_at(&self, point: &Point) -> Color {
        let u2 = (self.width * point.x).floor();
        let v2 = (self.height * point.y).floor();

        if f_equals((u2 + v2) % 2.0, 0.0) {
            self.a
        } else {
            self.b
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TextureMapPattern {
    uv_pattern: UVPattern,
    uv_map: UVMap,
    transform: Matrix4,
}

impl TextureMapPattern {
    pub fn new(uv_pattern: UVPattern, uv_map: UVMap) -> Self {
        TextureMapPattern {
            uv_pattern,
            uv_map,
            transform: glm::identity(),
        }
    }

    pub fn uv_map_point(&self, p: &Point) -> (F3D, F3D) {
        match self.uv_map {
            UVMap::Spherical => spherical_map(p),
            _ => panic!(),
        }
    }
}

impl Pattern for TextureMapPattern {
    fn get_transform(&self) -> Matrix4 {
        TPattern::default_transform()
    }

    fn set_transform(&mut self, m: Matrix4) {
        self.transform = m;
    }

    fn pattern_at(&self, p: &Point) -> Color {
        let (u, v) = self.uv_map_point(p);

        match &self.uv_pattern {
            UVPattern::Checkers(c) => c.uv_pattern_at(&point(u, v, 0.0)),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn uv_checkers() -> UVCheckers {
        UVCheckers::new(2.0, 2.0, Color::black(), Color::white())
    }

    #[test]
    fn checkers_pattern_at() {
        let p = uv_checkers();
        for c in [
            (0.0, 0.0, Color::black()),
            (0.5, 0.0, Color::white()),
            (0.0, 0.5, Color::white()),
            (0.5, 0.5, Color::black()),
            (1.0, 1.0, Color::black()),
        ] {
            assert_eq!(p.uv_pattern_at(&point(c.0, c.1, 0.0)), c.2);
        }
    }

    #[test]
    fn texture_map_with_spherical_map() {
        let checkers = uv_checkers();
        let pattern = TextureMapPattern::new(UVPattern::Checkers(checkers), UVMap::Spherical);
        for c in [
            (point(0.4315, 0.4670, 0.7719), Color::white()),
            (point(-0.9654, 0.2552, -0.0534), Color::black()),
            (point(0.1039, 0.7090, 0.6975), Color::white()),
            (point(-0.7652, 0.2175, 0.6060), Color::black()),
        ] {
            assert_eq!(pattern.pattern_at(&c.0), c.1);
        }
    }
}
