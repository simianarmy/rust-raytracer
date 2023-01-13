use crate::color::Color;
use crate::math;
use crate::tuple::*;
use crate::world::World;
use rand::rngs::ThreadRng;
use rand::Rng;

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

const NUM_AREA_SAMPLES: u32 = 5;

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
            Light::Point(p) => p.intensity_at(world, point),
            Light::Area(a) => a.intensity_at(world, point),
        }
    }
}

impl PointLight {
    fn intensity_at(&self, world: &World, point: &Point) -> math::F3D {
        if world.is_shadowed(&self.position, point) {
            0.0
        } else {
            1.0
        }
    }
}

impl AreaLight {
    fn intensity_at(&self, world: &World, point: &Point) -> math::F3D {
        // For # samples, calculate random point within the area
        // and call is_shadowed to that point.
        // Return average of non-shadowed rays
        let mut rng = rand::thread_rng();
        let mut tot = 0.0;

        for _ in 0..NUM_AREA_SAMPLES {
            if !world.is_shadowed(&self.rnd_point(&mut rng), point) {
                tot += 1.0;
            }
        }
        tot / NUM_AREA_SAMPLES as math::F3D
    }

    fn rnd_point(&self, rng: &mut ThreadRng) -> Point {
        let x = rng.gen::<f64>() * self.radius;
        let y = rng.gen::<f64>() * self.radius;
        self.light.position + vector(x, y, 0.0)
    }
}

// backwards compat helpers
pub fn point_light(position: Point, intensity: Color) -> Light {
    Light::point(position, intensity)
}

pub fn area_light(position: Point, intensity: Color, radius: math::F3D) -> Light {
    Light::area(position, intensity, radius)
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
