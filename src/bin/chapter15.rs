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
use raytracer::transformation::*;
use raytracer::tuple::*;
use raytracer::world::World;

const CHAPTER: u8 = 15;

fn main() {
    let fname = "tests/cow.obj";

    let mut world = World::new(point_light(point(-10.0, 10.0, -10.0), Color::white()));

    let obj = parse_obj_file(fname).unwrap();

    world.add_shape(obj.to_group().divide(40));

    let mut camera = Camera::new(500, 250, glm::pi::<F3D>() / 3.0);
    //let mut camera = Camera::new(100, 50, glm::pi::<F3D>() / 3.0);
    camera.transform = view_transform(&point(0.0, 3.5, -5.0), &point_y(), &vector_y());

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
