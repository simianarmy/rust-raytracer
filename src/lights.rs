use crate::color::Color;
use crate::tuple::*;

#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

pub fn point_light(position: Point, intensity: Color) -> PointLight {
    PointLight {
        position,
        intensity,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let p = point_light(point_zero(), Color::black());
        assert_eq!(p.position, point_zero());
        assert_eq!(p.intensity, Color::black());
    }
}
