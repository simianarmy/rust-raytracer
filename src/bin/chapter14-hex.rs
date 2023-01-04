/**
 * Simple scene demonstrating groups to draw a hexagon
 */
extern crate nalgebra_glm as glm;
extern crate raytracer;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::lights::*;
use raytracer::math::F3D;
use raytracer::object::*;
use raytracer::ppm::*;
use raytracer::shapes::cylinder::*;
use raytracer::shapes::plane::plane;
use raytracer::shapes::sphere::*;
use raytracer::transformation::*;
use raytracer::tuple::*;
use raytracer::world::World;

const CHAPTER: u8 = 14;

fn hexagon_corner() -> Object {
    sphere().with_transformation(make_translation(0.0, 0.0, -1.0) * make_scaling(0.25, 0.25, 0.25))
}

fn hexagon_edge() -> Object {
    cylinder(0.0, 1.0, false).with_transformation(
        make_translation(0.0, 0.0, -1.0)
            * make_rotation_y(-glm::pi::<F3D>() / 6.0)
            * make_rotation_z(-glm::half_pi::<F3D>())
            * make_scaling(0.25, 1.0, 0.25),
    )
}

fn hexagon_side(i: usize) -> Object {
    let corner = hexagon_corner();
    let edge = hexagon_edge();
    Object::new_group(vec![corner, edge])
        .transform(&make_rotation_y((i as F3D) * glm::pi::<F3D>() / 3.0))
}

fn hexagon() -> Object {
    let mut sides = vec![];
    for i in 0..6 {
        sides.push(hexagon_side(i));
    }
    Object::new_group(sides)
        .transform(&(make_translation(0.0, 1.0, 0.0) * make_rotation_x(-glm::pi::<F3D>() / 6.0)))
}

fn main() {
    let mut world = World::new(point_light(point(-10.0, 10.0, -15.0), Color::white()));
    let mut floor = plane();
    floor.material.color = Color::new(0.8, 0.7, 0.8);
    floor.material.specular = 0.0;
    floor.material.transparency = 0.3;
    floor.material.reflective = 0.8;
    floor.set_transform(&make_rotation_z(0.01));

    world.add_shape(floor);
    world.add_shape(hexagon());

    let mut camera = Camera::new(500, 250, glm::pi::<F3D>() / 3.0);
    //let mut camera = Camera::new(100, 50, glm::pi::<F3D>() / 3.0);
    camera.transform = view_transform(&point(0.0, 2.0, -5.0), &point_y(), &vector_y());

    let canvas = camera.render(&world);

    let filename = format!("./ppms/chapter{}-hex.ppm", CHAPTER);
    match create_file_from_data(&filename, &canvas.to_ppm()) {
        Ok(_) => {
            println!("file created ({})!", filename);
        }
        Err(err) => {
            println!("Error writing file! {}", err);
        }
    }
}
