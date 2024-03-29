/**
 * Simple scene with area lighting
 */
extern crate nalgebra_glm as glm;

use crate::camera::Camera;
use crate::color::Color;
use crate::lights::*;
use crate::materials::Material;
use crate::math::F3D;
use crate::shapes::sphere::sphere;
use crate::transformation::*;
use crate::tuple::*;
use crate::world::World;

pub fn run(hsize: usize, vsize: usize) {
    let mut floor = sphere(); // unit sphere
    floor.material.color = Color::new(1.0, 0.9, 0.9);
    floor.material.specular = 0.0;
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
    rsphere.material.color = Color::new(0.5, 1.0, 0.1);
    rsphere.material.diffuse = 0.7;
    rsphere.material.specular = 0.3;

    let mut lsphere = sphere();
    let st = make_translation(-1.5, 0.33, -0.75) * make_scaling(0.33, 0.33, 0.33);
    lsphere.set_transform(&st);
    lsphere.material.color = Color::new(1.0, 0.8, 0.1);
    lsphere.material.diffuse = 0.7;
    lsphere.material.specular = 0.3;

    let mut world = World::new(vec![
        area_light(point(-10.0, 10.0, -10.0), Color::white(), 1.5), //point_light(point(-10.0, 10.0, -10.0), Color::white())]);
    ]);
    world.add_shape(floor);
    world.add_shape(lwall);
    world.add_shape(rwall);
    world.add_shape(msphere);
    world.add_shape(rsphere);
    world.add_shape(lsphere);

    let mut camera = Camera::new(hsize, vsize, glm::pi::<F3D>() / 3.0);
    camera.transform = view_transform(&point(0.0, 1.5, -5.0), &point_y(), &vector_y());

    camera.render(&world).to_file("./ppms/chapter8.ppm")
}
