extern crate nalgebra_glm as glm;

use crate::camera::Camera;
use crate::color::*;
use crate::lights::*;
use crate::materials::*;
use crate::math::F3D;
use crate::obj_file::*;
use crate::object::*;
use crate::pattern::*;
use crate::ppm::*;
use crate::shapes::csg::*;
use crate::shapes::cube::*;
use crate::shapes::plane::*;
use crate::shapes::sphere::*;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;

const CHAPTER: u8 = 16;

pub fn run(hsize: usize, vsize: usize) {
    let mut world = World::new(vec![point_light(point(-10.0, 10.0, -10.0), Color::white())]);

    let mut floor = plane();
    floor.material.specular = 0.0;

    let mut checkers = checkers::checkers_pattern(Color::white(), color(0.4, 0.4, 0.4));
    checkers.set_transform(make_scaling(0.1, 0.1, 0.2));
    floor.material.pattern = Some(TPattern::Checkers(checkers));

    let mut back_wall = plane();
    back_wall.set_transform(&(make_translation(0.0, 0.0, 10.0) * make_rotation_x(glm::half_pi())));

    let mut material_purple = Material::new(0.1, 0.7, 0.0, 0.0);
    material_purple.color = color(0.373, 0.404, 0.550);

    let mut s1 = sphere();
    s1.set_material(material_purple);
    let mut s2 = cube();
    s2.set_material(Material {
        color: color(1.0, 0.0, 0.0),
        ..Material::default()
    });
    s2.set_transform(
        &(make_translation(-0.5, 0.5, 0.0) * make_rotation_y(1.5) * make_scaling(0.5, 0.5, 0.5)),
    );
    let mut csg = Object::new_csg(CsgOp::Intersection, &s2, &s1);
    csg.set_transform(&make_translation(0.0, 1.3, 0.0));

    world.add_shape(floor);
    world.add_shape(back_wall);
    world.add_shape(csg);

    let mut camera = Camera::new(hsize, vsize, glm::pi::<F3D>() / 3.0);
    camera.transform = view_transform(&point(0.0, 1.5, -5.0), &point_y(), &vector_y());

    let filename = format!("./ppms/chapter{}.ppm", CHAPTER);
    camera.render(&world).to_file(&filename)
}
