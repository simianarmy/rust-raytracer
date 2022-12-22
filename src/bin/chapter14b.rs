/**
 * Simple scene
 */
extern crate nalgebra_glm as glm;
extern crate raytracer;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::cylinder::*;
use raytracer::group::*;
use raytracer::lights::*;
use raytracer::math::F3D;
use raytracer::plane::plane;
use raytracer::ppm::*;
use raytracer::shape::*;
use raytracer::sphere::*;
use raytracer::transformation::*;
use raytracer::tuple::*;
use raytracer::world::World;
use std::sync::Arc;

const CHAPTER: u8 = 14;

fn hexagon_corner() -> Sphere {
    let mut corner = sphere();
    corner.set_transform(&(make_translation(0.0, 0.0, -1.0) * make_scaling(0.25, 0.25, 0.25)));
    corner
}

fn hexagon_edge() -> Cylinder {
    let mut edge = cylinder();
    edge.set_bounds(0.0, 1.0);
    edge.set_transform(
        &(make_translation(0.0, 0.0, -1.0)
            * make_rotation_y(-glm::pi::<F3D>() / 6.0)
            * make_rotation_z(-glm::half_pi::<F3D>())
            * make_scaling(0.25, 1.0, 0.25)),
    );
    edge
}

fn hexagon_side(i: usize) -> GroupRef {
    let mut side = default_group();
    set_transform(
        &mut side,
        &make_rotation_y((i as F3D) * glm::pi::<F3D>() / 3.0),
    );
    add_child_group(&mut side, &Group::from_shape(Box::new(hexagon_corner())));
    add_child_group(&mut side, &Group::from_shape(Box::new(hexagon_edge())));
    println!("weak count: {}", Arc::weak_count(&side));
    println!("strong count: {}", Arc::strong_count(&side));
    side
}

fn hexagon() -> GroupRef {
    let mut hex = default_group();
    set_transform(
        &mut hex,
        &(make_translation(0.0, 1.0, 0.0) * make_rotation_x(-glm::pi::<F3D>() / 6.0)),
    );
    for i in 0..6 {
        let side = hexagon_side(i);
        add_child_group(&mut hex, &side);
    }
    hex
}

fn main() {
    let mut world = World::new(point_light(point(-10.0, 10.0, -15.0), Color::white()));
    let mut floor = plane();
    floor.props.material.color = Color::new(0.8, 0.7, 0.8);
    floor.props.material.specular = 0.0;
    floor.props.material.transparency = 0.3;
    floor.props.material.reflective = 0.8;
    floor.set_transform(&make_rotation_z(0.01));

    world.add_shape(Box::new(floor));
    world.add_group(&hexagon());

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
