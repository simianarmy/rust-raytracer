extern crate nalgebra_glm as glm;
extern crate raytracer;

use rand::Rng;
use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::lights::*;
use raytracer::materials::Material;
use raytracer::math::F3D;
use raytracer::obj_file::*;
use raytracer::ppm::*;
use raytracer::shapes::plane::*;
use raytracer::transformation::*;
use raytracer::tuple::*;
use raytracer::world::World;

const CHAPTER: u8 = 15;

fn main() {
    let fname = "fixtures/teapot-lowres.obj";

    let mut world = World::new(point_light(point(-10.0, 20.0, -10.0), Color::white()));
    let mut floor = plane();
    floor.material.color = Color::new(0.8, 0.7, 0.8);
    floor.set_transform(&(make_translation(0.0, -10.0, 0.0) * make_rotation_z(0.01)));

    world.add_shape(floor);

    let obj = parse_obj_file(fname).unwrap();

    world.add_shape(
        obj.to_group()
            .transform(&(make_translation(0.0, -5.0, 0.0) * make_rotation_x(-1.74)))
            .divide(40),
    );

    //let mut camera = Camera::new(500, 250, glm::pi::<F3D>() / 3.0);
    let mut camera = Camera::new(300, 150, glm::pi::<F3D>() / 3.0);
    //let mut camera = Camera::new(100, 50, glm::pi::<F3D>() / 3.0);
    camera.transform = view_transform(&point(0.0, 16.0, -60.0), &point_zero(), &vector_y());

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
    //println!(
    //"bounding box opts: {}",
    //NUM_BOUNDING_OPTS.load(Ordering::SeqCst)
    //);
}
