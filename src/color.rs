use crate::math::F3D;
use crate::tuple::{tuple, Tuple};
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Copy, Clone)]
pub struct Color {
    rgb: Tuple,
}

impl Color {
    pub fn new(r: F3D, g: F3D, b: F3D) -> Color {
        Color {
            rgb: tuple(r, g, b, 0.0),
        }
    }

    pub fn from_tuple(t: &Tuple) -> Color {
        Color::new(t.x, t.y, t.z)
    }

    pub fn white() -> Color {
        Color::new(1.0, 1.0, 1.0)
    }

    pub fn black() -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    pub fn red(&self) -> F3D {
        self.rgb.x
    }

    pub fn green(&self) -> F3D {
        self.rgb.y
    }

    pub fn blue(&self) -> F3D {
        self.rgb.z
    }

    pub fn tuple(&self) -> &Tuple {
        &self.rgb // immutable ref, readonly
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.rgb == other.rgb
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        //Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
        let added = self.tuple() + other.tuple();
        Self::from_tuple(&added)
    }
}

impl Mul<F3D> for Color {
    type Output = Self;

    fn mul(self, scalar: F3D) -> Self::Output {
        let rgb = self.tuple() * scalar;
        Color::from_tuple(&rgb)
    }
}

impl Mul for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        let rgb1 = self.tuple();
        let rgb2 = other.tuple();
        // doesn't work :(
        //Color::from_tuple(rgb1 * rgb2);
        Color::new(rgb1.x * rgb2.x, rgb1.y * rgb2.y, rgb1.z * rgb2.z)
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "r: {}, g: {}, b: {}", self.rgb.x, self.rgb.y, self.rgb.z)
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "r: {}, g: {}, b: {}", self.rgb.x, self.rgb.y, self.rgb.z)
    }
}

// constructor utility
pub fn color(r: F3D, g: F3D, b: F3D) -> Color {
    Color::new(r, g, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_component_accessors() {
        let c = color(4.3, -2.1, 0.0);
        assert_eq!(c.red(), 4.3);
        assert_eq!(c.green(), -2.1);
        assert_eq!(c.blue(), 0.0);
    }

    #[test]
    fn color_data_accessor() {
        let c = color(4.3, -2.1, 0.0);
        let d = c.tuple();
        assert_eq!(d.x, 4.3);
        assert_eq!(d.y, -2.1);
        assert_eq!(d.z, 0.0);
    }
}
