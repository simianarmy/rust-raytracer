/**
 * Cast rays to draw a sphere
 */
extern crate nalgebra_glm as glm;
extern crate raytracer;

use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::group;
use raytracer::intersection::hit;
use raytracer::lights::*;
use raytracer::materials::lighting;
use raytracer::math::F3D;
use raytracer::ppm::*;
use raytracer::ray::Ray;
use raytracer::shapes::shape::*;
use raytracer::shapes::sphere::sphere;
use raytracer::tuple::*;
use std::sync::Arc;

fn main() {
    let mut sphere = sphere(); // unit sphere
    sphere.props.material.color = Color::new(1.0, 0.2, 1.0);
    let light_pos = point(-10.0, 10.0, -10.0);
    let color = Color::white();
    let light = point_light(light_pos, color);
    let eye = point(0.0, 0.0, -5.0);
    let wall = point(0.0, 0.0, 10.0);
    let wall_size = 7.0;
    let canvas_size = 500;
    let pixel_size: F3D = wall_size / canvas_size as F3D;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_size, canvas_size, None);

    for y in 0..canvas_size {
        // compute world y coordinate
        let world_y = half - pixel_size * y as F3D;

        for x in 0..canvas_size {
            let world_x = -half + pixel_size * x as F3D;

            // cast ray to canvas pixel
            let eye_to_wall = point(world_x, world_y, wall.z) - eye;
            let ray = Ray::new(eye, glm::normalize(&eye_to_wall));
            let xs = sphere.intersect(&ray);

            match hit(&xs) {
                Some(is) => {
                    let p = ray.position(is.t);
                    let normal = group::normal_at(&is.group, &p);
                    let eye = -ray.direction;
                    let color = lighting(
                        &is.group.get_material(),
                        Arc::clone(&is.group),
                        &light,
                        &p,
                        &eye,
                        &normal,
                        false,
                    );
                    canvas.write_pixel(x, y, color);
                }
                _ => canvas.write_pixel(x, y, Color::black()),
            }
        }
    }
    let filename = "./ppms/chapter5.ppm";
    match create_file_from_data(filename, &canvas.to_ppm()) {
        Ok(_) => {
            println!("file created ({})!", filename);
        }
        Err(err) => {
            println!("Error writing file! {}", err);
        }
    }
}
