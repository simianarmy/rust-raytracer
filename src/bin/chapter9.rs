/**
 * Simple scene
 */
extern crate nalgebra_glm as glm;
extern crate raytracer;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::lights::*;
use raytracer::materials::Material;
use raytracer::math::F3D;
use raytracer::ppm::*;
use raytracer::shapes::plane::plane;
use raytracer::shapes::shape::*;
use raytracer::shapes::sphere::sphere;
use raytracer::transformation::*;
use raytracer::tuple::*;
use raytracer::world::World;

const CHAPTER: u8 = 9;

fn main() {
    let mut floor = plane(); // unit sphere
    floor.props.material.color = Color::new(0.0, 0.1, 0.9);
    floor.props.material.specular = 0.0;
    floor.set_transform(&make_rotation_z(glm::pi::<F3D>() / 8.0));

    let mut msphere = sphere();
    msphere.set_transform(&make_translation(-0.5, 1.0, 0.5));
    let mut m = Material::default();
    m.color = Color::new(0.1, 1.0, 0.5);
    m.diffuse = 0.7;
    m.specular = 0.3;
    msphere.set_material(m);

    let mut rsphere = sphere();
    let rt = make_translation(1.5, 0.5, -0.5) * make_scaling(0.5, 0.5, 0.5);
    rsphere.set_transform(&rt);
    rsphere.props.material.color = Color::new(0.5, 1.0, 0.1);
    rsphere.props.material.diffuse = 0.7;
    rsphere.props.material.specular = 0.3;

    let mut lsphere = sphere();
    let st = make_translation(-1.5, 0.33, -0.75) * make_scaling(0.33, 0.33, 0.33);
    lsphere.set_transform(&st);
    lsphere.props.material.color = Color::new(1.0, 0.8, 0.1);
    lsphere.props.material.diffuse = 0.7;
    lsphere.props.material.specular = 0.3;

    let mut world = World::new(point_light(point(-10.0, 10.0, -10.0), Color::white()));
    world.add_shape(Box::new(floor));
    world.add_shape(Box::new(msphere));
    world.add_shape(Box::new(rsphere));
    world.add_shape(Box::new(lsphere));

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
