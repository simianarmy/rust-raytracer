use crate::math::F3D;
use crate::ray::Ray;
use crate::shape::*;
use crate::tuple::*;
use std::clone::Clone;
use std::fmt;

#[derive(Clone)]
pub struct Intersection {
    pub t: F3D,
    pub object: ShapeBox,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t // todo: object comparison
    }
}

impl fmt::Debug for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "intersection t = {}, object = {:p}", self.t, self.object)
    }
}

impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Intersection: t = {}, object = {:p}",
            self.t, self.object
        )
    }
}

// Intersection list builder
#[macro_export]
macro_rules! intersections {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x.clone());
            )*
            temp_vec.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
            temp_vec
        }
    };
}

/**
 * "Closest" intersection in a collection
 */
pub fn hit(is: &Vec<Intersection>) -> Option<&Intersection> {
    // filter out negative t values here
    is.iter().map(|is| is).find(|i| i.t >= 0.0)
}

pub struct Computations {
    pub t: F3D,
    pub object: ShapeBox,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
}

pub fn prepare_computations(i: &Intersection, ray: &Ray) -> Computations {
    let p = ray.position(i.t);
    let normal = i.object.normal_at(p);
    let eyev = -ray.direction;
    let inside = normal.dot(&eyev) < 0.0;

    Computations {
        t: i.t,
        object: i.object.clone(),
        point: p,
        eyev,
        normalv: if inside { -normal } else { normal },
        inside,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::sphere::*;

    #[test]
    fn intersections_macro_builds_list_from_args() {
        let s = sphere();
        let i1 = s.intersection(1.0);
        let i2 = s.intersection(2.0);
        let is = intersections!(i1, i2);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0], i1);
    }

    #[test]
    fn hit_all_intersections_pos_t() {
        let s = sphere();
        let i1 = s.intersection(1.0);
        let i2 = s.intersection(2.0);
        let xs = intersections!(i2, i1);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i1);
    }

    #[test]
    fn hit_some_intersections_neg_t() {
        let s = sphere();
        let i1 = s.intersection(-1.0);
        let i2 = s.intersection(2.0);
        let xs = intersections!(i2, i1);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i2);
    }

    #[test]
    fn hit_all_intersections_neg_t() {
        let s = sphere();
        let i1 = s.intersection(-2.0);
        let i2 = s.intersection(-1.0);
        let xs = intersections!(i2, i1);
        let i = hit(&xs);
        assert_eq!(i, None);
    }

    #[test]
    fn hit_always_lowest_nonneg_t() {
        let s = sphere();
        let i1 = s.intersection(5.0);
        let i2 = s.intersection(7.0);
        let i3 = s.intersection(-3.0);
        let i4 = s.intersection(2.0);
        let xs = intersections!(i1, i2, i3, i4);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i4);
    }

    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let shape = sphere();
        let i = shape.intersection(4.0);
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.t, i.t);
        assert_eq!(&comps.object, &i.object);
        assert_eq!(comps.point, point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn precomputing_intersection_on_outside() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let shape = sphere();
        let i = shape.intersection(4.0);
        let comps = prepare_computations(&i, &r);
        assert!(!comps.inside);
    }

    #[test]
    fn precomputing_intersection_on_inside() {
        let r = Ray::new(point_zero(), vector_z());
        let shape = sphere();
        let i = shape.intersection(1.0);
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.point, point_z());
        assert_eq!(comps.eyev, vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }
}
