/**
 * Simple scene
 * Testing Bounding Box Optimization
 */
extern crate nalgebra_glm as glm;
extern crate raytracer;

use rand::Rng;
use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::cone::*;
use raytracer::cube::*;
use raytracer::cylinder::*;
use raytracer::group::*;
use raytracer::group::*;
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
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

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

fn main() {
    let mut world = World::new(point_light(point(-10.0, 10.0, -10.0), Color::white()));

    let mut floor = plane(); // unit sphere
    floor.props.material.color = Color::new(0.8, 0.7, 0.8);
    floor.props.material.specular = 0.0;
    floor.props.material.transparency = 0.3;
    floor.props.material.reflective = 0.8;
    floor.set_transform(&make_rotation_z(0.01));

    world.add_shape(Box::new(floor));
    let mut groups = Vec::new();
    groups.push(default_group());
    groups.push(default_group());
    groups.push(default_group());
    groups.push(default_group());

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
        let gidx = get_quadrant(xmod, ymod, zmod);
        add_child_shape(&mut groups[gidx], Box::new(glass_ball));
    }
    println!("group 0 size: {}", groups[0].shapes.borrow().len());
    println!("group 1 size: {}", groups[1].shapes.borrow().len());
    println!("group 2 size: {}", groups[2].shapes.borrow().len());
    println!("group 3 size: {}", groups[3].shapes.borrow().len());
    world.add_group(&groups[0]);
    world.add_group(&groups[1]);
    world.add_group(&groups[2]);
    world.add_group(&groups[3]);

    let mut camera = Camera::new(500, 250, glm::pi::<F3D>() / 3.0);
    //let mut camera = Camera::new(100, 50, glm::pi::<F3D>() / 3.0);
    camera.transform = view_transform(&point(0.0, 3.5, -5.0), &point_y(), &vector_y());

    let canvas = camera.render(&world);

    let filename = format!("./ppms/chapter{}-bb.ppm", CHAPTER);
    match create_file_from_data(&filename, &canvas.to_ppm()) {
        Ok(_) => {
            println!("file created ({})!", filename);
        }
        Err(err) => {
            println!("Error writing file! {}", err);
        }
    }
    println!(
        "bounding box opts: {}",
        NUM_BOUNDING_OPTS.load(Ordering::SeqCst)
    );
}