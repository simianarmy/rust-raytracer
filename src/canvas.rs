use crate::color::Color;
use crate::ppm::canvas_to_string;

pub struct Canvas {
    width: usize,
    height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize, color: Option<Color>) -> Canvas {
        let pixels = vec![color.unwrap_or(Color::white()); width * height];
        Canvas {
            width,
            height,
            pixels,
        }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, c: Color) {
        let i = x + self.width * y;
        self.pixels[i] = c;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        &self.pixels[x + self.width * y]
    }

    pub fn to_ppm(&self) -> String {
        canvas_to_string(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor_populates_wxh_with_white_pixels() {
        let c = Canvas::new(10, 20, None);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert_eq!(c.pixels.len(), 200);
        assert!(c.pixels.iter().all(|p| *p == Color::white()));
    }

    #[test]
    fn writing_pixels_to_canvas() {
        let mut c = Canvas::new(10, 20, None);
        c.write_pixel(2, 3, Color::new(1.0, 0.0, 0.0));
        let res = c.pixel_at(2, 3);
        assert_eq!(*res, Color::new(1.0, 0.0, 0.0));
        let white = c.pixel_at(1, 3);
        assert_ne!(*white, Color::new(1.0, 0.0, 0.0));
    }
}
