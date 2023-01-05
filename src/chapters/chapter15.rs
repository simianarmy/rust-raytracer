extern crate nalgebra_glm as glm;

use crate::camera::Camera;
use crate::color::Color;
use crate::lights::*;
use crate::math::F3D;
use crate::obj_file::*;
use crate::ppm::*;
use crate::shapes::plane::*;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;

const CHAPTER: u8 = 15;

pub fn run(fixture: &String, hsize: usize, vsize: usize) {
    let mut world = World::new(point_light(point(-10.0, 20.0, -10.0), Color::white()));
    let mut floor = plane();
    floor.material.color = Color::new(0.8, 0.7, 0.8);
    floor.set_transform(&(make_translation(0.0, -10.0, 0.0) * make_rotation_z(0.01)));

    world.add_shape(floor);

    let obj = parse_obj_file(fixture).unwrap();

    world.add_shape(
        obj.to_group()
            .transform(&(make_translation(0.0, 5.0, 0.0) * make_rotation_y(-1.74)))
            .divide(40),
    );

    let mut camera = Camera::new(hsize, vsize, glm::pi::<F3D>() / 3.0);
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
