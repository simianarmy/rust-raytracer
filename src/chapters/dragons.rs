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
use crate::shapes::group::*;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;

pub fn run(fixture: &String, hsize: usize, vsize: usize) {
    let mut world = World::new(vec![
        point_light(point(-10.0, 100.0, -100.0), Color::white()),
        point_light(point(0.0, 100.0, 0.0), color(0.1, 0.1, 0.1)),
        point_light(point(100.0, 10.0, -25.0), color(0.2, 0.2, 0.2)),
        point_light(point(-100.0, 10.0, -25.0), color(0.2, 0.2, 0.2)),
    ]);

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

    let dragon_material = Material {
        ambient: 0.1,
        diffuse: 0.6,
        specular: 0.3,
        shininess: 15.0,
        ..Material::default()
    };

    let mut pedestal = cylinder(-0.15, 0.0, true);
    pedestal.set_material(Material {
        color: color(0.2, 0.2, 0.2),
        ambient: 0.0,
        diffuse: 0.8,
        specular: 0.0,
        reflective: 0.2,
        ..Material::default()
    });

    let d1 = dragon.clone().set_group_material(Material {
        color: color(1.0, 0.0, 0.1),
        ..dragon_material.clone()
    });

    let mut box1 = bbox.clone();
    box1.set_material(Material {
        ambient: 0.0,
        diffuse: 0.4,
        specular: 0.0,
        transparency: 0.6,
        refractive_index: 1.0,
        ..Material::default()
    });

    let g1 = Object::new_group(vec![pedestal.clone(), Object::new_group(vec![d1, box1])])
        .with_transformation(make_translation(0.0, 2.0, 0.0));

    let d2 = dragon.clone().set_group_material(Material {
        color: color(1.0, 0.5, 0.1),
        ..dragon_material.clone()
    });

    let mut box2 = bbox.clone();
    box2.set_material(Material {
        ambient: 0.0,
        diffuse: 0.2,
        specular: 0.0,
        transparency: 0.8,
        refractive_index: 1.0,
        ..Material::default()
    });

    let g2 = Object::new_group(vec![
        pedestal.clone(),
        Object::new_group(vec![d2, box2])
            .with_transformation(make_scaling(0.75, 0.75, 0.75) * make_rotation_y(4.0)),
    ])
    .with_transformation(make_translation(2.0, 1.0, -1.0));

    let d3 = dragon.clone().set_group_material(Material {
        color: color(0.9, 0.5, 0.1),
        ..dragon_material.clone()
    });

    let g3 = Object::new_group(vec![
        pedestal.clone(),
        Object::new_group(vec![d3])
            .with_transformation(make_rotation_y(-0.4) * make_scaling(0.75, 0.75, 0.75)),
    ])
    .with_transformation(make_translation(-2.0, 0.75, -1.0));

    let d4 = dragon.clone().set_group_material(Material {
        color: color(1.0, 0.9, 0.1),
        ..dragon_material.clone()
    });

    let g4 = Object::new_group(vec![
        pedestal.clone(),
        Object::new_group(vec![d4])
            .with_transformation(make_rotation_y(-0.2) * make_scaling(0.5, 0.5, 0.5)),
    ])
    .with_transformation(make_translation(-4.0, 0.0, -2.0));

    let d5 = dragon.clone().set_group_material(Material {
        color: color(0.9, 1.0, 0.1),
        ..dragon_material.clone()
    });

    let g5 = Object::new_group(vec![
        pedestal.clone(),
        Object::new_group(vec![d5])
            .with_transformation(make_rotation_y(3.3) * make_scaling(0.5, 0.5, 0.5)),
    ])
    .with_transformation(make_translation(4.0, 0.0, -2.0));

    let d6 = dragon.clone().set_group_material(dragon_material.clone());

    let g6 = Object::new_group(vec![
        pedestal.clone(),
        Object::new_group(vec![d6]).with_transformation(make_rotation_y(glm::pi())),
    ])
    .with_transformation(make_translation(0.0, 0.5, -4.0));

    //world.add_shape(g1);
    //world.add_shape(g2);
    //world.add_shape(g3);
    //world.add_shape(g4);
    //world.add_shape(g5);
    world.add_shape(g6);

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
