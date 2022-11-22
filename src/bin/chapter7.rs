/**
 * Simple scene
 */
extern crate nalgebra_glm as glm;
extern crate raytracer;

use raytracer::camera::Camera;
use raytracer::color::Color;
use raytracer::lights::*;
use raytracer::materials::Material;
use raytracer::math::F3D;
use raytracer::ppm::*;
use raytracer::shape::*;
use raytracer::sphere::sphere;
use raytracer::transformation::*;
use raytracer::tuple::*;
use raytracer::world::World;

fn main() {
    let mut floor = sphere(); // unit sphere
    floor.props.material.color = Color::new(1.0, 0.9, 0.9);
    floor.props.material.specular = 0.0;
    floor.set_transform(&make_scaling(10.0, 0.01, 10.0));

    let mut lwall = sphere();
    let lwall_transform = make_translation(0.0, 0.0, 5.0)
        * make_rotation_y(-glm::quarter_pi::<F3D>())
        * make_rotation_x(glm::half_pi())
        * make_scaling(10.0, 0.01, 10.0);
    lwall.set_transform(&lwall_transform);
    lwall.set_material(floor.get_material().clone());

    let mut rwall = sphere();
    let rwall_transform = make_translation(0.0, 0.0, 5.0)
        * make_rotation_y(glm::quarter_pi::<F3D>())
        * make_rotation_x(glm::half_pi())
        * make_scaling(10.0, 0.01, 10.0);
    rwall.set_transform(&rwall_transform);
    rwall.set_material(floor.get_material().clone());

    let mut msphere = sphere();
    msphere.set_transform(&make_translation(-0.5, 1.0, 0.5));
    let mut m = Material::default();
    m.color = Color::new(0.1, 1.0, 0.5);
    m.diffuse = 0.7;
    m.specular = 0.3;
    msphere.set_material(m);

    let mut rsphere = sphere();
    let rt = make_translation(1.5, 0.5, -0.5) * make_scaling(0.5, 0.5, 0.5);
    rsphere.set_transform(&rt);
    rsphere.props.material.color = Color::new(0.5, 1.0, 0.1);
    rsphere.props.material.diffuse = 0.7;
    rsphere.props.material.specular = 0.3;

    //println!("lwall material: {}", lwall.props.transform);
    //println!("rwall material: {}", rwall.props.transform);
    let mut world = World::new(point_light(point(-10.0, 10.0, -10.0), Color::white()));
    world.add_shape(Box::new(floor));
    world.add_shape(Box::new(lwall));
    world.add_shape(Box::new(rwall));
    world.add_shape(Box::new(msphere));
    world.add_shape(Box::new(rsphere));

    let mut camera = Camera::new(500, 250, glm::pi::<F3D>() / 3.0);
    camera.transform = view_transform(&point(0.0, 1.5, -5.0), &point_y(), &vector_y());

    let canvas = camera.render(&world);

    let filename = "./ppms/chapter7.ppm";
    match create_file_from_data(filename, &canvas.to_ppm()) {
        Ok(_) => {
            println!("file created ({})!", filename);
        }
        Err(err) => {
            println!("Error writing file! {}", err);
        }
    }
}
