use crate::color::Color;
use crate::intersection::*;
use crate::lights::*;
use crate::materials::lighting;
use crate::materials::Material;
use crate::ray::Ray;
use crate::shape::*;
use crate::sphere::sphere_with_id;
use crate::transformation::make_scaling;
use crate::tuple::*;

pub const MAX_RAY_DEPTH: u8 = 5;

pub struct World {
    light: PointLight,
    shapes: Vec<ShapeBox>,
}

impl World {
    pub fn new(light: PointLight) -> World {
        World {
            light,
            shapes: vec![],
        }
    }

    pub fn add_shape(&mut self, s: ShapeBox) {
        self.shapes.push(s);
    }

    pub fn get_shape(&self, i: usize) -> &ShapeBox {
        &self.shapes[i]
    }

    pub fn set_shape(&mut self, shape: ShapeBox, i: usize) {
        self.shapes[i] = shape;
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

    pub fn shade_hit(&self, comps: &Computations, remaining: u8) -> Color {
        let shadowed = self.is_shadowed(&comps.over_point);

        let surface = lighting(
            comps.object.get_material(),
            comps.object.clone(), // booooo
            &self.light,
            comps.over_point,
            comps.eyev,
            comps.normalv,
            shadowed,
        );
        let reflected = self.reflected_color(comps, remaining);

        surface + reflected
    }

    pub fn color_at(&self, ray: &Ray, remaining: u8) -> Color {
        let xs = self.intersect(ray);
        // find hit from the intersections
        match hit(&xs) {
            Some(is) => {
                let comps = prepare_computations(is, ray);
                self.shade_hit(&comps, remaining)
            }
            None => Color::black(),
        }
    }

    pub fn is_shadowed(&self, p: &Point) -> bool {
        let v = self.light.position - p;
        let distance = v.magnitude();
        let direction = v.normalize();
        let r = Ray::new(*p, direction);
        let xs = self.intersect(&r);
        match hit(&xs) {
            Some(is) if is.t < distance => true,
            _ => false,
        }
    }

    pub fn reflected_color(&self, comps: &Computations, remaining: u8) -> Color {
        // if at max recursive depth
        if remaining == 0 {
            Color::black()
        } else {
            let m = comps.object.get_material();
            if m.reflective == 0.0 {
                Color::black()
            } else {
                let reflection_ray = Ray::new(comps.over_point, comps.reflectv);
                let c = self.color_at(&reflection_ray, remaining - 1);
                c * m.reflective
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let light = point_light(point(-10.0, 10.0, -10.0), Color::white());
        let mut s1 = sphere_with_id(Some("s1".to_string()));
        let mut m = Material::default();
        m.color = Color::new(0.8, 1.0, 0.6);
        m.diffuse = 0.7;
        m.specular = 0.2;
        s1.set_material(m);
        let mut s2 = sphere_with_id(Some("s2".to_string()));
        s2.set_transform(&make_scaling(0.5, 0.5, 0.5));
        let mut world = World::new(light);
        world.add_shape(Box::new(s1)); // move operation
        world.add_shape(Box::new(s2));
        world
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;
    use crate::intersection::prepare_computations;
    use crate::lights::point_light;
    use crate::materials::Material;
    use crate::plane::plane;
    use crate::sphere::*;
    use crate::transformation::*;
    use std::f64::consts::SQRT_2;

    #[test]
    fn constructor_assigns() {
        let light = point_light(point(-10.0, 10.0, -10.0), Color::white());
        let world = World::default();
        assert_eq!(world.light, light);
        let s1 = &world.shapes[0];
        let s2 = &world.shapes[1];
        assert_eq!(s1.get_id(), "sphere_s1");
        assert_eq!(s2.get_id(), "sphere_s2");
    }

    #[test]
    fn set_shape() {
        let mut world = World::default();
        let s = sphere_with_id(Some("hi".to_string()));
        world.set_shape(Box::new(s), 1);
        let bs = world.get_shape(1);
        assert_eq!(bs.get_id(), String::from("sphere_hi"));
    }

    #[test]
    fn intersect_world_with_ray() {
        let world = World::default();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let xs = world.intersect(&ray);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 4.5);
        assert_eq!(xs[2].t, 5.5);
        assert_eq!(xs[3].t, 6.0);
    }

    #[test]
    fn shading_an_intersection() {
        let world = World::default();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let shape = &world.shapes[0];
        let i = shape.intersection(4.0);
        let comps = prepare_computations(&i, &ray);
        let c = world.shade_hit(&comps, MAX_RAY_DEPTH);
        assert_eq_eps!(c.tuple(), Color::new(0.38066, 0.47583, 0.2855).tuple());
    }

    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut world = World::default();
        world.light = point_light(point(0.0, 0.25, 0.0), Color::white());
        let ray = Ray::new(point_zero(), vector_z());
        let shape = &world.shapes[1];
        let i = shape.intersection(0.5);
        let comps = prepare_computations(&i, &ray);
        let c = world.shade_hit(&comps, MAX_RAY_DEPTH);
        assert_eq_eps!(c.tuple(), Color::new(0.90498, 0.90498, 0.90498).tuple());
    }

    #[test]
    fn color_when_ray_misses() {
        let world = World::default();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector_y());
        let c = world.color_at(&ray, MAX_RAY_DEPTH);
        assert_eq!(c, Color::black());
    }

    #[test]
    fn color_when_ray_hits() {
        let world = World::default();
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let c = world.color_at(&ray, MAX_RAY_DEPTH);
        assert_eq_eps!(c.tuple(), Color::new(0.38066, 0.47583, 0.2855).tuple());
    }

    #[test]
    fn color_with_intersection_behind_ray() {
        let mut world = World::default();

        // all this to change the material ambient value :(
        let outer = world.get_shape(0);
        let om = outer.get_material().clone();
        let om2 = Material { ambient: 1.0, ..om };
        let mut o2 = outer.clone();
        o2.set_material(om2);
        world.set_shape(o2, 0);

        let inner = world.get_shape(1);
        let im = inner.get_material().clone();
        let im2 = Material { ambient: 1.0, ..im };
        let mut i2 = inner.clone();
        i2.set_material(im2);
        world.set_shape(i2, 1);

        let ray = Ray::new(point(0.0, 0.0, 0.75), vector(0.0, 0.0, -1.0));
        let c = world.color_at(&ray, MAX_RAY_DEPTH);
        let i3 = world.get_shape(1);
        assert_eq!(c.tuple(), i3.get_material().color.tuple());
    }

    #[test]
    fn no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let world = World::default();
        let p = point(0.0, 10.0, 0.0);
        assert!(!world.is_shadowed(&p));
    }

    #[test]
    fn shadow_when_object_between_point_and_light() {
        let world = World::default();
        let p = point(10.0, -10.0, 10.0);
        assert!(world.is_shadowed(&p));
    }

    #[test]
    fn no_shadow_when_object_behind_light() {
        let world = World::default();
        let p = point(-20.0, 20.0, -20.0);
        assert!(!world.is_shadowed(&p));
    }

    #[test]
    fn no_shadow_when_object_behind_point() {
        let world = World::default();
        let p = point(-2.0, 2.0, -2.0);
        assert!(!world.is_shadowed(&p));
    }

    #[test]
    fn shade_hit_given_intersection_in_shadow() {
        let mut world = World::new(point_light(point(0.0, 0.0, -10.0), Color::white()));
        world.add_shape(Box::new(sphere()));

        let mut s2 = sphere();
        s2.set_transform(&make_translation(0.0, 0.0, 10.0));
        world.add_shape(Box::new(s2));

        let ray = Ray::new(point(0.0, 0.0, 5.0), vector_z());
        let i = world.get_shape(1).intersection(4.0);
        let comps = prepare_computations(&i, &ray);
        let c = world.shade_hit(&comps, MAX_RAY_DEPTH);
        assert_eq_eps!(c.tuple(), Color::new(0.1, 0.1, 0.1).tuple());
    }

    #[test]
    fn reflective_color_for_nonreflective_material() {
        let world = World::default();
        let r = Ray::new(point_zero(), vector_z());
        let mut s2 = world.get_shape(1).clone();
        s2.set_material(Material {
            ambient: 1.0,
            ..Material::default()
        });
        let i = s2.intersection(1.0);
        let comps = prepare_computations(&i, &r);
        let color = world.reflected_color(&comps, MAX_RAY_DEPTH);
        assert_eq!(color, Color::black());
    }

    #[test]
    fn reflective_color_for_reflective_material() {
        let mut world = World::default();
        let mut shape = plane();
        shape.set_material(Material {
            reflective: 0.5,
            ..Material::default()
        });
        shape.set_transform(&make_translation(0.0, -1.0, 0.0));
        world.add_shape(Box::new(shape.clone()));
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let i = shape.intersection(SQRT_2);
        let comps = prepare_computations(&i, &r);
        let color = world.reflected_color(&comps, MAX_RAY_DEPTH);
        assert_eq_eps!(color.tuple(), Color::new(0.19032, 0.2379, 0.14274).tuple());
    }

    #[test]
    fn shade_hit_for_reflective_material() {
        let mut world = World::default();
        let mut shape = plane();
        shape.set_material(Material {
            reflective: 0.5,
            ..Material::default()
        });
        shape.set_transform(&make_translation(0.0, -1.0, 0.0));
        world.add_shape(Box::new(shape.clone()));
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let i = shape.intersection(SQRT_2);
        let comps = prepare_computations(&i, &r);
        let color = world.shade_hit(&comps, MAX_RAY_DEPTH);
        assert_eq_eps!(
            color.tuple(),
            Color::new(0.87677, 0.924326, 0.82918).tuple()
        );
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut world = World::new(point_light(point_zero(), Color::white()));
        let mut lower = plane();
        lower.props.material.reflective = 1.0;
        lower.set_transform(&make_translation(0.0, -1.0, 0.0));
        let mut upper = plane();
        upper.props.material.reflective = 1.0;
        upper.set_transform(&make_translation(0.0, 1.0, 0.0));
        world.add_shape(Box::new(lower));
        world.add_shape(Box::new(upper));
        let r = Ray::new(point_zero(), vector_y());
        let color = world.color_at(&r, MAX_RAY_DEPTH);
        // we should get here
        assert!(color.tuple().x >= 0.0);
    }

    #[test]
    fn reflective_color_at_max_recursive_depth() {
        let mut world = World::default();
        let mut shape = plane();
        shape.set_material(Material {
            reflective: 0.5,
            ..Material::default()
        });
        shape.set_transform(&make_translation(0.0, -1.0, 0.0));
        world.add_shape(Box::new(shape.clone()));
        let r = Ray::new(
            point(0.0, 0.0, -3.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let i = shape.intersection(SQRT_2);
        let comps = prepare_computations(&i, &r);
        let color = world.reflected_color(&comps, 0);
        assert_eq!(color, Color::black());
    }
}
