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
use crate::shapes::cylinder::*;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;

pub fn run(fixture: &String, hsize: usize, vsize: usize) {
    let mut world = World::new(point_light(point(-10.0, 100.0, -100.0), Color::white()));

    // dragon obj
    let obj = parse_obj_file(fixture).unwrap();

    let dragon = obj
        .to_group()
        .transform(
            &(
                make_translation(0.0, 0.1217, 0.0) * make_scaling(0.268, 0.268, 0.268)
                //with this model
            ),
        )
        .divide(40);

    let mut raw_bbox = cube();
    raw_bbox.set_transform(
        &(make_translation(1.0, 1.0, 1.0)
            * make_scaling(3.73335, 2.5845, 1.6283)
            * make_translation(-3.9863, -0.1217, -1.1820)),
    );

    // overwrite other transformations or add to them?
    let bbox = raw_bbox.clone().with_transformation(
        make_translation(0.0, 0.1217, 0.0) * make_scaling(0.268, 0.268, 0.268),
    );

    let mut pedestal = cylinder(-0.15, 0.0, true);
    pedestal.set_material(Material {
        color: color(0.2, 0.2, 0.2),
        ambient: 0.0,
        diffuse: 0.8,
        specular: 0.0,
        reflective: 0.2,
        ..Material::default()
    });

    let mut d1 = dragon.clone();
    d1.material.color = color(1.0, 0.0, 0.1);
    d1.material.ambient = 0.1;
    d1.material.diffuse = 0.6;
    d1.material.specular = 0.3;
    d1.material.shininess = 15.0;

    let mut box1 = bbox.clone();
    box1.material.ambient = 0.0;
    box1.material.diffuse = 0.4;
    box1.material.specular = 0.0;
    box1.material.transparency = 0.6;
    box1.material.refractive_index = 1.0;

    let g1 = Object::new_group(vec![pedestal.clone(), Object::new_group(vec![d1, box1])])
        .with_transformation(make_translation(0.0, 2.0, 0.0));

    let mut d2 = dragon.clone();
    d2.material.color = color(1.0, 0.5, 0.1);
    d2.material.ambient = 0.1;
    d2.material.diffuse = 0.6;
    d2.material.specular = 0.3;
    d2.material.shininess = 15.0;

    let g2 = Object::new_group(vec![
        pedestal.clone(),
        Object::new_group(vec![d2])
            .with_transformation(make_scaling(0.75, 0.75, 0.75) * make_rotation_y(4.0)),
    ])
    .with_transformation(make_translation(2.0, 1.0, -1.0));

    let mut d3 = dragon.clone();
    d3.material.color = Color::white();
    d3.material.ambient = 0.1;
    d3.material.diffuse = 0.6;
    d3.material.specular = 0.3;
    d3.material.shininess = 15.0;

    let g3 = Object::new_group(vec![
        pedestal.clone(),
        Object::new_group(vec![d3]).with_transformation(make_rotation_y(glm::pi())),
    ])
    .with_transformation(make_translation(0.0, 0.5, -4.0));

    world.add_shape(g1);
    world.add_shape(g2);
    world.add_shape(g3);

    let mut camera = Camera::new(hsize, vsize, 1.2);

    camera.transform = view_transform(&point(0.0, 2.5, -10.0), &point(0.0, 1.0, 0.0), &vector_y());

    let canvas = camera.render(&world);

    let filename = format!("./ppms/dragons.ppm");
    match create_file_from_data(&filename, &canvas.to_ppm()) {
        Ok(_) => {
            println!("file created ({})!", filename);
        }
        Err(err) => {
            println!("Error writing file! {}", err);
        }
    }
}
