extern crate nalgebra_glm as glm;

use crate::camera::Camera;
use crate::color::*;
use crate::lights::*;
use crate::materials::*;
use crate::math::F3D;
use crate::obj_file::*;
use crate::ppm::*;
use crate::shapes::plane::*;
use crate::shapes::sphere::*;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;

const CHAPTER: u8 = 16;

pub fn run(hsize: usize, vsize: usize) {
    let mut world = World::new(point_light(point(50.0, 100.0, -50.0), Color::white()));

    let mut material_white = Material::new(0.1, 0.7, 0.0, 0.0);
    material_white.reflective = 0.1;
    let mut material_blue = Material::new(0.1, 0.7, 0.0, 0.0);
    material_blue.reflective = 0.1;
    material_blue.color = color(0.537, 0.831, 0.914);
    let mut material_red = Material::new(0.1, 0.7, 0.0, 0.0);
    material_red.reflective = 0.1;
    material_red.color = color(0.941, 0.322, 0.388);
    let mut material_purple = Material::new(0.1, 0.7, 0.0, 0.0);
    material_purple.reflective = 0.1;
    material_purple.color = color(0.373, 0.404, 0.550);

    let standard_transform = make_translation(1.0, -1.0, 1.0) * make_scaling(0.5, 0.5, 0.5);
    let large_object = standard_transform * make_scaling(3.5, 3.5, 3.5);
    let medium_object = standard_transform * make_scaling(3.0, 3.0, 3.0);
    let small_object = standard_transform * make_scaling(2.0, 2.0, 2.0);

    let mut plane = plane();
    let mut mat = Material::default();
    mat.ambient = 1.0;
    mat.diffuse = 0.0;
    mat.specular = 0.0;
    plane.set_material(mat);
    plane.set_transform(&(make_translation(0.0, 0.0, 500.0) * make_rotation_x(glm::half_pi())));
    world.add_shape(plane);

    let mut s1 = sphere();
    let mut mat = Material::new(0.0, 0.2, 1.0, 200.0);
    mat.reflective = 0.7;
    mat.transparency = 0.7;
    mat.refractive_index = 1.5;
    mat.color = color(0.373, 0.404, 0.550);
    s1.set_material(mat);
    s1.set_transform(&large_object);
    world.add_shape(s1);

    let mut camera = Camera::new(hsize, vsize, 0.785);
    camera.transform = view_transform(
        &point(-6.0, 6.0, 10.0),
        &point(6.0, 0.0, 6.0),
        &vector(-0.45, 1.0, 0.0),
    );

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
