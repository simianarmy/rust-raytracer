/**
 * Simple scene
 */
extern crate nalgebra_glm as glm;
extern crate raytracer;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::cube::*;
use raytracer::lights::*;
use raytracer::materials::Material;
use raytracer::math::F3D;
use raytracer::pattern;
use raytracer::plane::plane;
use raytracer::ppm::*;
use raytracer::shape::*;
use raytracer::sphere::*;
use raytracer::transformation::*;
use raytracer::tuple::*;
use raytracer::world::World;

const CHAPTER: u8 = 12;

fn main() {
    let mut floor = plane(); // unit sphere
    floor.props.material.color = Color::new(0.8, 0.7, 0.8);
    floor.props.material.specular = 0.0;
    floor.props.material.transparency = 0.3;
    floor.props.material.reflective = 0.8;
    floor.set_transform(&make_rotation_z(0.01));
    //floor
    //.props
    //.material
    //.set_pattern(Some(Box::new(pattern::ring::ring_pattern(
    //Color::white(),
    //Color::new(0.5, 0.3, 0.4),
    //))));

    let mut shape1 = sphere();
    let transform = make_translation(-0.5, 1.2, 0.5) * make_rotation_y(glm::pi());
    shape1.set_transform(&transform);
    let mut m = Material::default();
    m.color = Color::new(0.5, 0.0, 0.0);
    m.diffuse = 0.1;
    m.ambient = 0.1;
    m.specular = 0.9;
    m.shininess = 300.0;
    m.reflective = 0.9;
    m.transparency = 0.8;
    shape1.set_material(m);

    let mut shape = sphere();
    let rt = make_translation(0.5, 1.0, 1.0) * make_scaling(0.5, 0.5, 0.5);
    shape.set_transform(&rt);
    shape.props.material.color = Color::new(0.5, 1.0, 0.1);
    //shape.props.material.diffuse = 0.4;
    //shape.props.material.specular = 0.3;
    shape
        .props
        .material
        .set_pattern(Some(Box::new(pattern::checkers::checkers_pattern(
            Color::white(),
            Color::new(0.5, 0.8, 0.3),
        ))));

    let mut lcube = cube();
    let st = make_translation(-1.5, 0.8, -0.75)
        * make_rotation_y(glm::quarter_pi())
        * make_scaling(2.13, 0.13, 0.13);
    lcube.set_transform(&st);
    lcube.props.material.color = Color::new(0.5, 0.8, 0.8);
    lcube.props.material.reflective = 0.9;
    lcube.props.material.transparency = 0.7;
    lcube.props.material.shininess = 300.0;

    let mut world = World::new(point_light(point(-10.0, 10.0, -10.0), Color::white()));
    world.add_shape(Box::new(floor));
    world.add_shape(Box::new(shape1));
    world.add_shape(Box::new(shape));
    world.add_shape(Box::new(lcube));

    let mut camera = Camera::new(500, 250, glm::pi::<F3D>() / 3.0);
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
