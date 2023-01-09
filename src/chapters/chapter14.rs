/**
 * Simple scene
 * Testing no optimizations
 */
extern crate nalgebra_glm as glm;

use crate::camera::Camera;
use crate::color::Color;
use crate::lights::*;
use crate::materials::Material;
use crate::math::F3D;
use crate::ppm::*;
use crate::shapes::plane::plane;
use crate::shapes::sphere::*;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;
use rand::Rng;
//use std::sync::atomic::{AtomicUsize, Ordering};

const CHAPTER: u8 = 14;

// 0: bottom left
// 1: bottom right
// 2: top left
// 3: top right
fn get_quadrant(x: i32, y: i32, _z: i32) -> usize {
    if x < 0 {
        // left
        if y < 0 {
            // bottom
            0
        } else {
            2
        }
    } else {
        // right
        if y < 0 {
            1
        } else {
            3
        }
    }
}

pub fn run(hsize: usize, vsize: usize) {
    let mut world = World::new(vec![point_light(point(-10.0, 10.0, -10.0), Color::white())]);

    let mut floor = plane(); // unit sphere
    floor.material.color = Color::new(0.8, 0.7, 0.8);
    floor.material.specular = 0.0;
    floor.material.transparency = 0.3;
    floor.material.reflective = 0.8;
    floor.set_transform(&make_rotation_z(0.01));

    world.add_shape(floor);

    let mut rng = rand::thread_rng();
    for _i in 0..280 {
        let mut glass_ball = sphere();
        let xmod = if rng.gen::<i32>() % 2 == 0 { 1 } else { -1 };
        let ymod = if rng.gen::<i32>() % 2 == 0 { 1 } else { -1 };
        let zmod = if rng.gen::<i32>() % 2 == 0 { 1 } else { -1 };
        let scale = rng.gen::<f64>() * 0.4;
        let transform = make_translation(
            rng.gen::<f64>() * 10.0 * (xmod as F3D),
            rng.gen::<f64>() * 1.0 * (ymod as F3D),
            rng.gen::<f64>() * 1.0 * (zmod as F3D),
        ) * make_scaling(scale, scale, scale);
        glass_ball.set_transform(&transform);
        let mut m = Material::default();
        m.color = Color::new(0.5, 0.0, 0.0);
        glass_ball.set_material(m);

        // add shape to the proper quadrant
        world.add_shape(glass_ball);
    }
    let mut camera = Camera::new(hsize, vsize, glm::pi::<F3D>() / 3.0);
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
