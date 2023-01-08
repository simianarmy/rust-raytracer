use crate::intersection::*;
use crate::math::*;
use crate::object::Object;
use crate::ray::Ray;
use crate::tuple::*;

#[derive(Debug)]
pub struct Computations<'a> {
    pub t: F3D,
    pub object: &'a Object,
    pub point: Point,
    pub over_point: Point,
    pub under_point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub reflectv: Vector,
    pub inside: bool,
    pub n1: F3D,
    pub n2: F3D,
}

fn calc_refractive_indices(i: &Intersection, xs: &Intersections) -> (F3D, F3D) {
    let mut containers: Vec<&Object> = Vec::new();
    let mut n1 = 0.0;
    let mut n2 = 0.0;

    for is in xs.iter() {
        let iid = i.object.get_id();
        let is_hit = i.t == is.t && iid == is.object.get_id();

        if is_hit {
            if containers.len() == 0 {
                n1 = 1.0;
            } else {
                n1 = containers.last().unwrap().get_material().refractive_index;
            }
        }
        // if container holds the current hit object
        if let Some(pos) = containers
            .iter()
            .position(|i| is.object.get_id() == i.get_id())
        {
            containers.swap_remove(pos);
        } else {
            containers.push(&is.object);
        }

        if is_hit {
            if containers.is_empty() {
                n2 = 1.0;
            } else {
                n2 = containers.last().unwrap().get_material().refractive_index;
            }
            break;
        }
    }
    (n1, n2)
}

pub fn prepare_computations<'a>(
    i: &Intersection<'a>,
    ray: &Ray,
    xs: &Intersections,
) -> Computations<'a> {
    let p = ray.position(i.t);
    let normal = i.object.normal_at(p, Some(i));
    let eyev = -ray.direction;
    let inside = normal.dot(&eyev) < 0.0;
    let normalv = if inside { -normal } else { normal };
    let reflectv = reflect(ray.direction, normalv);
    let (n1, n2) = calc_refractive_indices(i, xs);

    Computations {
        t: i.t,
        object: &i.object,
        point: p,
        over_point: p + normalv * EPSILON,
        under_point: p - normalv * EPSILON,
        eyev,
        normalv,
        reflectv,
        inside,
        n1,
        n2,
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;
    use crate::computations::prepare_computations;
    use crate::ray::Ray;
    use crate::shapes::plane::plane;
    use crate::shapes::sphere::*;
    use crate::transformation::*;

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let shape = sphere();
        let i = Intersection::new(&shape, 4.0);
        let comps =
            prepare_computations(&i, &r, &Intersections::from_intersections(vec![i.clone()]));
        assert_eq!(comps.t, i.t);
        assert_eq!(&comps.object.get_id(), &i.object.get_id());
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn precomputing_intersection_on_outside() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let shape = sphere();
        let i = Intersection::new(&shape, 4.0);
        let comps =
            prepare_computations(&i, &r, &Intersections::from_intersections(vec![i.clone()]));
        assert!(!comps.inside);
    }

    #[test]
    fn precomputing_intersection_on_inside() {
        let r = Ray::new(point_zero(), vector_z());
        let shape = sphere();
        let i = Intersection::new(&shape, 1.0);
        let comps =
            prepare_computations(&i, &r, &Intersections::from_intersections(vec![i.clone()]));
        assert_eq!(comps.point, point_z());
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }

    #[test]
    fn precomputing_reflection_vector() {
        let shape = plane();
        let r = Ray::new(
            point(0.0, 1.0, -1.0),
            vector(0.0, -SQRT_2 / 2.0, SQRT_2 / 2.0),
        );
        let i = Intersection::new(&shape, SQRT_2);
        let comps =
            prepare_computations(&i, &r, &Intersections::from_intersections(vec![i.clone()]));
        assert_eq!(comps.reflectv, vector(0.0, SQRT_2 / 2.0, SQRT_2 / 2.0));
    }

    #[test]
    fn finding_n1_n2_at_various_intersections() {
        let mut a = glass_sphere();
        a.set_transform(&make_scaling(2.0, 2.0, 2.0));
        a.material.refractive_index = 1.5;
        let mut b = glass_sphere();
        b.set_transform(&make_translation(0.0, 0.0, -0.25));
        b.material.refractive_index = 2.0;
        let mut c = glass_sphere();
        c.set_transform(&make_translation(0.0, 0.0, 0.25));
        c.material.refractive_index = 2.5;
        let ray = Ray::new(point(0.0, 0.0, -4.0), vector_y());
        let xs = Intersections::from_intersections(vec![
            Intersection::new(&a, 2.0),
            Intersection::new(&b, 2.75),
            Intersection::new(&c, 3.25),
            Intersection::new(&b, 4.75),
            Intersection::new(&c, 5.25),
            Intersection::new(&a, 6.0),
        ]);
        let cases = vec![
            (0, 1.0, 1.5),
            (1, 1.5, 2.0),
            (2, 2.0, 2.5),
            (3, 2.5, 2.5),
            (4, 2.5, 1.5),
            (5, 1.5, 1.0),
        ];
        for c in cases {
            let comps = prepare_computations(&xs[c.0], &ray, &xs);
            assert_eq!(comps.n1, c.1, "n1 mismatch {:?}", comps);
            assert_eq!(comps.n2, c.2, "n2 mismatch {:?}", comps);
        }
    }

    #[test]
    fn underpoint_is_offset_below_surface() {
        let ray = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let mut s = glass_sphere();
        s.set_transform(&make_translation(0.0, 0.0, 1.0));
        let i = Intersection::new(&s, 5.0);
        let xs = Intersections::from_intersections(vec![i.clone()]);
        let comps = prepare_computations(&i, &ray, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }
}
