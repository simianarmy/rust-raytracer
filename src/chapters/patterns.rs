/**
 * Patterns demo scene
 */
extern crate nalgebra_glm as glm;

use crate::camera::Camera;
use crate::color::*;
use crate::lights::*;
use crate::materials::Material;
use crate::math::F3D;
use crate::pattern::*;
use crate::ppm::*;
use crate::shapes::cube::cube;
use crate::shapes::plane::plane;
use crate::shapes::shape::*;
use crate::shapes::sphere::sphere;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;

pub fn run(hsize: usize, vsize: usize) {
    let mut floor = plane();
    floor.material.specular = 0.0;

    let mut checkers = checkers::checkers_pattern(Color::white(), color(0.4, 0.4, 0.4));
    checkers.set_transform(make_scaling(0.1, 0.1, 0.2));
    floor.material.pattern = Some(TPattern::Checkers(checkers));

    let mut backWall = plane();
    backWall.set_transform(&(make_translation(0.0, 0.0, 10.0) * make_rotation_x(glm::half_pi())));
    let mut rings = ring::ring_pattern(color(1.0, 0.1, 0.1), color(0.8, 0.7, 0.8));
    rings.set_transform(make_scaling(0.4, 0.4, 0.4));
    backWall.material.specular = 0.0;
    // doesn't work !?
    backWall.material.pattern = Some(TPattern::Ring(rings));
    println!("back wall: {:#?}", backWall);

    let mut left = sphere();
    left.set_transform(&(make_translation(-1.5, 0.33, -0.75) * make_scaling(0.33, 0.33, 0.33)));
    left.material.color = color(1.0, 0.8, 0.1);
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    let mut middle = sphere();
    middle.set_transform(&make_translation(-0.5, 1.0, 0.5));
    middle.material.color = color(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;

    let mut right = cube();
    right.set_transform(&(make_translation(1.5, 0.5, -0.5) * make_scaling(0.5, 0.5, 0.5)));
    right.material.color = color(0.5, 1.0, 0.1);
    right.material.diffuse = 0.7;
    right.material.specular = 0.3;

    let mut stripes = stripe::stripe_pattern(color(1.0, 0.0, 0.2), color(0.0, 0.0, 1.0));
    stripes.set_transform(make_rotation_z(0.5) * make_scaling(0.1, 0.1, 0.2));
    left.material.pattern = Some(TPattern::Stripe(stripes));

    let uv_checkers = texture_map::UVCheckers::new(16.0, 8.0, color(0.1, 1.0, 0.1), Color::white());
    middle.material.set_pattern(Some(TPattern::TextureMap(
        texture_map::TextureMapPattern::new(
            texture_map::UVPattern::Checkers(uv_checkers),
            texture_map::UVMap::Spherical,
        ),
    )));

    right.material.pattern = Some(TPattern::Gradient(gradient::gradient_pattern(
        Color::white(),
        color(0.0, 0.0, 0.9),
    )));

    let mut world = World::new(vec![point_light(point(-10.0, 10.0, -10.0), Color::white())]);
    world.add_shape(floor);
    world.add_shape(backWall);
    world.add_shape(middle);
    world.add_shape(right);
    world.add_shape(left);

    let mut camera = Camera::new(hsize, vsize, glm::pi::<F3D>() / 3.0);
    camera.transform = view_transform(&point(0.0, 1.5, -5.0), &point_y(), &vector_y());

    let canvas = camera.render(&world);

    let filename = format!("./ppms/patterns.ppm");
    match create_file_from_data(&filename, &canvas.to_ppm()) {
        Ok(_) => {
            println!("file created ({})!", filename);
        }
        Err(err) => {
            eprintln!("Error writing file! {}", err);
        }
    }
}
