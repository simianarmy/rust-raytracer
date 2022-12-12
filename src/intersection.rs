use crate::computations::Computations;
use crate::group::*;
use crate::math::*;
use crate::shape::*;
use std::clone::Clone;
use std::fmt;

#[derive(Clone)]
pub struct Intersection {
    pub t: F3D,
    // TODO: object -> GroupRef
    //pub object: ShapeBox,
    pub object: GroupRef,
}

impl Intersection {
    pub fn new(shape: ShapeBox, t: F3D) -> Intersection {
        Intersection {
            t,
            object: Group::from_shape(shape),
        }
    }

    pub fn from_group(g: &GroupRef, t: F3D) -> Intersection {
        Intersection {
            t,
            object: g.clone(),
        }
    }
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        f_equals(self.t, other.t) // todo: object comparison
    }
}

impl fmt::Debug for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(
            f,
            "intersection t = {}, object = {}",
            self.t,
            self.object.get_id()
        )
    }
}

impl fmt::Display for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Intersection: t = {}, object = {}",
            self.t,
            self.object.get_id()
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
            crate::intersection::sort_intersections(&mut temp_vec);
            temp_vec
        }
    };
}

pub fn sort_intersections(xs: &mut Vec<Intersection>) {
    xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
}

/**
 * "Closest" intersection in a collection
 */
pub fn hit(is: &Vec<Intersection>) -> Option<&Intersection> {
    // filter out negative t values here
    is.iter().map(|is| is).find(|i| i.t >= 0.0)
}

// Approximate Fresnel effect
pub fn schlick(comps: &Computations) -> F3D {
    let mut cos = comps.eyev.dot(&comps.normalv);

    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n * n * (1.0 - cos.powi(2));

        if sin2_t > 1.0 {
            return 1.0;
        }
        let cos_t = (1.0 - sin2_t).sqrt();
        cos = cos_t;
    }
    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_eq_feps;
    use crate::computations::prepare_computations;
    use crate::ray::Ray;
    use crate::sphere::*;
    use crate::transformation::*;
    use crate::tuple::*;
    use std::clone::Clone;

    #[test]
    fn intersections_macro_builds_list_from_args() {
        let s = sphere();
        let i1 = Intersection::new(Box::new(s.clone()), 1.0);
        let i2 = Intersection::new(Box::new(s.clone()), 2.0);
        let is = intersections!(i1, i2);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0], i1);
    }

    //#[test]
    //fn hit_all_intersections_pos_t() {
    //let s = sphere();
    //let i1 = s.intersection(1.0);
    //let i2 = s.intersection(2.0);
    //let xs = intersections!(i2, i1);
    //let i = hit(&xs);
    //assert_eq!(*i.unwrap(), i1);
    //}

    //#[test]
    //fn hit_some_intersections_neg_t() {
    //let s = sphere();
    //let i1 = s.intersection(-1.0);
    //let i2 = s.intersection(2.0);
    //let xs = intersections!(i2, i1);
    //let i = hit(&xs);
    //assert_eq!(*i.unwrap(), i2);
    //}

    //#[test]
    //fn hit_all_intersections_neg_t() {
    //let s = sphere();
    //let i1 = s.intersection(-2.0);
    //let i2 = s.intersection(-1.0);
    //let xs = intersections!(i2, i1);
    //let i = hit(&xs);
    //assert_eq!(i, None);
    //}

    //#[test]
    //fn hit_always_lowest_nonneg_t() {
    //let s = sphere();
    //let i1 = s.intersection(5.0);
    //let i2 = s.intersection(7.0);
    //let i3 = s.intersection(-3.0);
    //let i4 = s.intersection(2.0);
    //let xs = intersections!(i1, i2, i3, i4);
    //let i = hit(&xs);
    //assert_eq!(*i.unwrap(), i4);
    //}

    //#[test]
    //fn hit_should_offset_point() {
    //let r = Ray::new(point(0.0, 0.0, -5.0), vector_z());
    //let mut s = sphere();
    //s.set_transform(&make_translation(0.0, 0.0, 1.0));
    //let i = s.intersection(5.0);
    //let comps = prepare_computations(&i, &r, &intersections!(i));
    //assert!(comps.over_point.z < -crate::math::EPSILON / 2.0);
    //assert!(comps.point.z > comps.over_point.z);
    //}

    //#[test]
    //fn schlick_under_total_internal_reflection() {
    //let sphere = glass_sphere();
    //let ray = Ray::new(point(0.0, 0.0, -SQRT_2_DIV_2), vector_y());
    //let xs = intersections!(
    //sphere.intersection(-SQRT_2_DIV_2),
    //sphere.intersection(SQRT_2_DIV_2)
    //);
    //let comps = prepare_computations(&xs[1], &ray, &xs);
    //let reflectance = schlick(&comps);
    //assert_eq!(reflectance, 1.0);
    //}

    //#[test]
    //fn schlick_with_perpendicular_viewing_angle() {
    //let sphere = glass_sphere();
    //let ray = Ray::new(point_zero(), vector_y());
    //let xs = intersections!(sphere.intersection(-1.0), sphere.intersection(1.0));
    //let comps = prepare_computations(&xs[1], &ray, &xs);
    //let reflectance = schlick(&comps);
    //assert_eq_feps!(reflectance, 0.04);
    //}

    //#[test]
    //fn schlick_with_small_angle() {
    //let sphere = glass_sphere();
    //let ray = Ray::new(point(0.0, 0.99, -2.0), vector_z());
    //let xs = intersections!(sphere.intersection(1.8589));
    //let comps = prepare_computations(&xs[0], &ray, &xs);
    //let reflectance = schlick(&comps);
    //assert_eq_feps!(reflectance, 0.48873);
    //}
}
