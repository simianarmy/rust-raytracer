use crate::color::Color;
use crate::math;
use crate::tuple::*;
use crate::world::World;

#[derive(Debug, PartialEq)]
pub enum Light {
    Point(PointLight),
    Area(AreaLight),
}

#[derive(Debug, PartialEq)]
pub struct PointLight {
    pub position: Point,
    pub intensity: Color,
}

#[derive(Debug, PartialEq)]
pub struct AreaLight {
    pub light: PointLight,
    pub radius: math::F3D,
}

impl Light {
    pub fn point(position: Point, intensity: Color) -> Self {
        Light::Point(PointLight {
            position,
            intensity,
        })
    }

    pub fn area(position: Point, intensity: Color, radius: math::F3D) -> Self {
        Light::Area(AreaLight {
            light: PointLight {
                position,
                intensity,
            },
            radius,
        })
    }

    pub fn position(&self) -> Point {
        match self {
            Light::Point(p) => p.position,
            Light::Area(a) => a.light.position,
        }
    }

    pub fn intensity(&self) -> Color {
        match self {
            Light::Point(p) => p.intensity,
            Light::Area(a) => a.light.intensity,
        }
    }

    pub fn radius(&self) -> math::F3D {
        if let Light::Area(a) = self {
            a.radius
        } else {
            0.0
        }
    }

    pub fn intensity_at(&self, world: &World, point: &Point) -> math::F3D {
        match self {
            Light::Point(p) => {
                if world.is_shadowed(self, point) {
                    0.0
                } else {
                    1.0
                }
            }
            Light::Area(a) => 0.0,
        }
    }
}

// backwards compat
pub fn point_light(position: Point, intensity: Color) -> Light {
    Light::point(position, intensity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_light_has_position_and_intensity() {
        let p = Light::point(point_zero(), Color::black());
        assert_eq!(p.position(), point_zero());
        assert_eq!(p.intensity(), Color::black());
    }

    #[test]
    fn area_light_contains_point_props_and_radius() {
        let al = Light::area(point_x(), Color::white(), 2.0);
        assert_eq!(al.position(), point_x());
        assert_eq!(al.intensity(), Color::white());
        assert_eq!(al.radius(), 2.0);
    }
}
