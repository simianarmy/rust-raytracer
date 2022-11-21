use crate::canvas::Canvas;
use crate::math::*;
use crate::matrix::Matrix4;
use crate::ray::Ray;
use crate::tuple::*;
use crate::world::World;
use glm;

#[derive(Debug)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    half_width: F3D,
    half_height: F3D,
    fov: F3D,
    pixel_size: F3D,
    pub transform: Matrix4,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: F3D) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as F3D / vsize as F3D;
        let half_width = if aspect >= 1.0 {
            half_view
        } else {
            half_view * aspect
        };
        let half_height = if aspect >= 1.0 {
            half_view / aspect
        } else {
            half_view
        };
        let pixel_size = (half_width * 2.0) / hsize as F3D;

        Camera {
            hsize,
            vsize,
            half_width,
            half_height,
            fov: field_of_view,
            pixel_size,
            transform: glm::identity(),
        }
    }

    pub fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        let xoffset = (x as F3D + 0.5) * self.pixel_size;
        let yoffset = (y as F3D + 0.5) * self.pixel_size;

        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        let pixel = glm::inverse(&self.transform) * point(world_x, world_y, -1.0);
        let origin = glm::inverse(&self.transform) * point_zero();
        let direction = (pixel - origin).normalize();

        Ray { origin, direction }
    }

    pub fn render(&self, world: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize, None);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let r = self.ray_for_pixel(x, y);
                let c = world.color_at(&r);
                image.write_pixel(x, y, c);
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::transformation::*;
    use glm;

    #[test]
    fn constructing_camera() {
        let c = Camera::new(160, 200, glm::half_pi());
        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 200);
        assert_eq!(c.fov, glm::half_pi());
        assert_eq!(c.transform, glm::identity::<F3D, 4>());
    }

    #[test]
    fn pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, glm::half_pi());
        println!("{:?}", c);
        assert_eq!(c.pixel_size, 0.01);
    }

    #[test]
    fn construct_ray_through_center_of_canvas() {
        let c = Camera::new(201, 101, glm::half_pi());
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, point_zero());
        assert_eq_eps!(r.direction, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn construct_ray_through_corner_of_canvas() {
        let c = Camera::new(201, 101, glm::half_pi());
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, point_zero());
        assert_eq_eps!(r.direction, vector(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn construct_ray_when_camera_transformed() {
        let mut c = Camera::new(201, 101, glm::half_pi());
        c.transform = make_rotation_y(glm::quarter_pi()) * make_translation(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq_eps!(r.origin, point(0.0, 2.0, -5.0));
        assert_eq_eps!(
            r.direction,
            vector(2_f32.sqrt() / 2.0, 0.0, -2_f32.sqrt() / 2.0)
        );
    }

    #[test]
    fn render_a_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, glm::half_pi());
        let from = point(0.0, 0.0, -5.0);
        let to = point_zero();
        let up = vector_y();
        c.transform = view_transform(&from, &to, &up);
        let image = c.render(&w);
        assert_eq_eps!(
            image.pixel_at(5, 5).tuple(),
            Color::new(0.38066, 0.47583, 0.2855).tuple()
        );
    }
}
