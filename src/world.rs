use crate::intersection::Intersection;
use crate::lights::PointLight;
use crate::ray::Ray;
use crate::shape::Shape;

pub struct World {
    pub light: PointLight,
    pub shapes: Vec<Box<dyn Shape>>,
}

impl World {
    pub fn new(light: PointLight) -> World {
        World {
            light,
            shapes: vec![],
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    // returns all ray/shape intersections sorted by t
    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs = self.shapes.iter().fold(vec![], |mut acc, curr| {
            let is = curr.intersect(ray);
            if is.len() > 0 {
                acc.extend(is);
            }
            acc
        });
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::lights::point_light;
    use crate::materials::Material;
    use crate::ray::Ray;
    use crate::sphere::sphere_with_id;
    use crate::transformation::make_scaling;
    use crate::tuple::*;

    fn default_world() -> World {
        let light = point_light(point(-10.0, 10.0, -10.0), Color::white());
        let mut s1 = sphere_with_id(Some("s1".to_string()));
        let mut m = Material::new();
        m.color = Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.props.material = m;
        let mut s2 = sphere_with_id(Some("s2".to_string()));
        s2.props.transform = make_scaling(0.5, 0.5, 0.5);
        let mut world = World::new(light);
        world.add_shape(Box::new(s1));
        world.add_shape(Box::new(s2));
        world
    }

    #[test]
    fn constructor_assigns() {
        let light = point_light(point(-10.0, 10.0, -10.0), Color::white());
        let world = default_world();
        assert_eq!(world.light, light);
        let s1 = &world.shapes[0];
        let s2 = &world.shapes[1];
        assert_eq!(s1.get_id(), "sphere_s1");
        assert_eq!(s2.get_id(), "sphere_s2");
    }

    #[test]
    fn intersect_world_with_ray() {
        let world = default_world();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let xs = world.intersect(&ray);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }
}
