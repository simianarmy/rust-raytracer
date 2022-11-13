use crate::canvas::Canvas;
use crate::color::Color;
use crate::math::F3D;

const PPM_MAX_COLOR: u32 = 255;

/**
 * Scale rgb color from 0 -> 255
 */
fn scale_color(val: F3D) -> u16 {
    let scaled = (val * PPM_MAX_COLOR as f32).round();
    // clamp() is too annoying with types
    if scaled > 255.0 {
        255
    } else if scaled < 0.0 {
        0
    } else {
        scaled as u16
    }
}

pub fn canvas_to_string(c: &Canvas) -> String {
    let (w, h) = c.dimensions();
    let header = String::from(format!("P3\n{} {}\n{}", w, h, PPM_MAX_COLOR));
    let mut body_lines: Vec<String> = vec![];

    // for each row
    for i in 0..h {
        let mut rgbs: Vec<String> = vec![];
        // for each column
        for j in 0..w {
            let color = c.pixel_at(j, i);
            rgbs.push(String::from(format!(
                "{} {} {}",
                scale_color(color.red()),
                scale_color(color.green()),
                scale_color(color.blue()),
            )));
        }
        let mut line = rgbs.join(" ");
        while line.len() > 70 {
            match line[0..69].rfind(' ') {
                Some(idx) => {
                    let (l1, rest) = line.split_at(idx);
                    body_lines.push(String::from(l1.trim()));
                    line = String::from(rest.trim());
                }
                _ => panic!("oh noes"),
            }
        }
        body_lines.push(line);
    }
    format!("{}\n{}\n", header, body_lines.join("\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scale_color_when_over_255() {
        assert_eq!(scale_color(1.5), 255);
    }

    #[test]
    fn scale_color_when_less_than_zero() {
        assert_eq!(scale_color(-1.5), 0);
    }

    #[test]
    fn scale_color_when_less_than_one() {
        assert_eq!(scale_color(0.5), 128);
    }

    #[test]
    fn generating_ppm_string() {
        let mut c = Canvas::new(5, 3, None);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        let ppm = c.to_ppm();
        let lines = ppm.split("\n").collect::<Vec<&str>>();
        assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    #[test]
    fn ppm_lines_under_70_chars() {
        let c1 = Color::new(1.0, 0.8, 0.6);
        let c = Canvas::new(10, 2, Some(c1));
        let ppm = c.to_ppm();
        let lines = ppm.split("\n").collect::<Vec<&str>>();
        assert_eq!(
            lines[3],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[4],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
        assert_eq!(
            lines[5],
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204"
        );
        assert_eq!(
            lines[6],
            "153 255 204 153 255 204 153 255 204 153 255 204 153"
        );
    }

    #[test]
    fn ppm_terminates_with_newline() {
        let c1 = Color::new(1.0, 0.8, 0.6);
        let c = Canvas::new(10, 2, Some(c1));
        let ppm = c.to_ppm();
        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }
}
