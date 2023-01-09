/**
 * Simple scene
 */
extern crate nalgebra_glm as glm;

use crate::camera::Camera;
use crate::color::Color;
use crate::lights::*;
use crate::materials::Material;
use crate::math::F3D;
use crate::pattern;
use crate::ppm::*;
use crate::shapes::cone::*;
use crate::shapes::cube::*;
use crate::shapes::cylinder::*;
use crate::shapes::plane::plane;
use crate::shapes::shape::*;
use crate::shapes::sphere::*;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;

const CHAPTER: u8 = 12;

pub fn run() {
    let mut floor = plane(); // unit sphere
    floor.material.color = Color::new(0.8, 0.7, 0.8);
    floor.material.specular = 0.0;
    floor.material.transparency = 0.3;
    floor.material.reflective = 0.8;
    floor.set_transform(&make_rotation_z(0.01));
    //floor
    //.props
    //.material
    //.set_pattern(Some(Box::new(pattern::ring::ring_pattern(
    //Color::white(),
    //Color::new(0.5, 0.3, 0.4),
    //))));

    let mut glass_ball = sphere();
    let transform = make_translation(-0.5, 1.2, 0.5) * make_rotation_y(glm::pi());
    glass_ball.set_transform(&transform);
    let mut m = Material::default();
    m.color = Color::new(0.5, 0.0, 0.0);
    m.diffuse = 0.1;
    m.ambient = 0.1;
    m.specular = 0.9;
    m.shininess = 300.0;
    m.reflective = 0.9;
    m.transparency = 0.8;
    glass_ball.set_material(m);

    let mut checker_ball = sphere();
    let rt = make_translation(0.0, 1.0, 1.0) * make_scaling(0.5, 0.5, 0.5);
    checker_ball.set_transform(&rt);
    checker_ball.material.color = Color::new(0.5, 1.0, 0.1);
    //checker_ball.material.diffuse = 0.4;
    //checker_ball.material.specular = 0.3;
    checker_ball
        .material
        .set_pattern(Some(pattern::TPattern::Checkers(
            pattern::checkers::checkers_pattern(Color::white(), Color::new(0.5, 0.8, 0.3)),
        )));

    let mut lcube = cube();
    let st = make_translation(-1.5, 0.8, -0.75)
        * make_rotation_y(glm::quarter_pi())
        * make_scaling(2.13, 0.13, 0.13);
    lcube.set_transform(&st);
    lcube.material.color = Color::new(0.5, 0.8, 0.8);
    lcube.material.reflective = 0.9;
    lcube.material.transparency = 0.7;
    lcube.material.shininess = 300.0;

    let mut cone1 = cone(0.0, 2.0, true);
    cone1.material.color = Color::new(0.4, 0.4, 1.0);
    cone1.material.diffuse = 0.1;
    cone1.material.ambient = 0.1;
    cone1.material.specular = 0.9;
    cone1.material.shininess = 300.0;
    cone1.material.reflective = 0.9;
    let st = make_translation(0.5, 0.8, -1.75)
        * make_rotation_x(glm::quarter_pi())
        * make_rotation_y(glm::half_pi())
        * make_scaling(0.3, 0.3, 0.3);
    cone1.set_transform(&st);

    let mut world = World::new(vec![point_light(point(-10.0, 10.0, -10.0), Color::white())]);
    world.add_shape(floor);
    world.add_shape(glass_ball);
    world.add_shape(checker_ball);
    world.add_shape(lcube);
    world.add_shape(cone1);

    let mut camera = Camera::new(500, 250, glm::pi::<F3D>() / 3.0);
    //let mut camera = Camera::new(100, 50, glm::pi::<F3D>() / 3.0);
    camera.transform = view_transform(&point(0.0, 1.5, -5.0), &point_y(), &vector_y());

    let canvas = camera.render(&world);

    let filename = format!("./ppms/chapter{}.ppm", CHAPTER);
    match create_file_from_data(&filename, &canvas.to_ppm()) {
        Ok(_) => {
            println!("file created ({})!", filename);
        }
        Err(err) => {
            println!("Error writing file! {}", err);
        }
    }
}
