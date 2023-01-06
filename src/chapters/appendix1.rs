extern crate nalgebra_glm as glm;

use crate::camera::Camera;
use crate::color::*;
use crate::lights::*;
use crate::materials::*;
use crate::math::F3D;
use crate::obj_file::*;
use crate::object::*;
use crate::ppm::*;
use crate::shapes::cube::*;
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

    let mut c1 = cube();
    c1.set_material(material_white.clone());
    c1.set_transform(&(make_translation(4.0, 0.0, 0.0) * medium_object));

    let mut c2 = cube();
    c2.set_material(material_blue.clone());
    c2.set_transform(&(make_translation(8.5, 1.5, -0.5) * large_object));

    let mut c3 = cube();
    c3.set_material(material_red.clone());
    c3.set_transform(&(make_translation(0.0, 0.0, 4.0) * large_object));

    let mut c4 = cube();
    c4.set_material(material_white.clone());
    c4.set_transform(&(make_translation(4.0, 0.0, 4.0) * small_object));

    let mut c5 = cube();
    c5.set_material(material_purple.clone());
    c5.set_transform(&(make_translation(7.5, 0.5, 4.0) * medium_object));

    let mut c6 = cube();
    c6.set_material(material_white.clone());
    c6.set_transform(&(make_translation(-0.25, 0.25, 8.0) * medium_object));

    let mut c7 = cube();
    c7.set_material(material_blue.clone());
    c7.set_transform(&(make_translation(4.0, 1.0, 7.5) * large_object));

    let mut c8 = cube();
    c8.set_material(material_red.clone());
    c8.set_transform(&(make_translation(10.0, 2.0, 7.5) * medium_object));

    let mut c9 = cube();
    c9.set_material(material_white.clone());
    c9.set_transform(&(make_translation(8.0, 2.0, 12.0) * small_object));

    let mut c10 = cube();
    c10.set_material(material_white.clone());
    c10.set_transform(&(make_translation(20.0, 1.0, 9.0) * small_object));

    let mut c11 = cube();
    c11.set_material(material_blue.clone());
    c11.set_transform(&(make_translation(-0.5, -5.0, 0.25) * large_object));

    let mut c12 = cube();
    c12.set_material(material_red.clone());
    c12.set_transform(&(make_translation(4.0, -4.0, 0.0) * large_object));

    let mut c13 = cube();
    c13.set_material(material_white.clone());
    c13.set_transform(&(make_translation(8.5, -4.0, 0.0) * large_object));

    let mut c14 = cube();
    c14.set_material(material_white.clone());
    c14.set_transform(&(make_translation(0.0, -4.0, 4.0) * large_object));

    let mut c15 = cube();
    c15.set_material(material_purple.clone());
    c15.set_transform(&(make_translation(-0.5, -4.5, 8.0) * large_object));

    let mut c16 = cube();
    c16.set_material(material_white.clone());
    c16.set_transform(&(make_translation(0.0, -8.0, 4.0) * large_object));

    let mut c17 = cube();
    c17.set_material(material_white.clone());
    c17.set_transform(&(make_translation(-0.5, -8.5, 8.0) * large_object));

    let g = Object::new_group(vec![
        s1, c1, c2, c3, c4, c5, c6, c7, c8, c9, c10, c11, c12, c13, c14, c15, c16, c17,
    ])
    .divide(40);

    world.add_shape(g);

    let mut camera = Camera::new(hsize, vsize, 0.785);
    camera.transform = view_transform(
        &point(-6.0, 6.0, -20.0),
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
