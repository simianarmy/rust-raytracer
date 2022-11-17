use crate::math::F3D;
use crate::shape::Intersectable;
use std::clone::Clone;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a, T: Intersectable + ?Sized> {
    pub t: F3D,
    pub object: &'a T,
}

impl<'a, T: Intersectable> Clone for Intersection<'a, T> {
    fn clone(&self) -> Intersection<'a, T> {
        Intersection {
            t: self.t,
            object: self.object.clone(),
        }
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
pub fn hit<'a, T>(is: &'a Vec<Intersection<'a, T>>) -> Option<&Intersection<'a, T>>
where
    T: Intersectable,
{
    // filter out negative t values here
    is.iter().map(|is| is).find(|i| i.t >= 0.0)
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
}
