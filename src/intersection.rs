use crate::math::F3D;

use crate::shape::Intersectable;

#[derive(Debug, PartialEq)]
pub struct Intersection<'a, T: Intersectable + ?Sized> {
    pub t: F3D,
    pub object: &'a T,
}

// Intersection list builder
macro_rules! intersections {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
            temp_vec
        }
    };
}

/**
 * "Closest" intersection in a collection
 */
pub fn hit<'a, T>(is: &'a Vec<&Intersection<'a, T>>) -> Option<&'a Intersection<'a, T>>
where
    T: Intersectable,
{
    // filter out negative t values here
    match is.into_iter().find(|&i| i.t >= 0.0) {
        Some(&intersection) => Some(intersection),
        _ => None,
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
        let is = intersections!(&i1, &i2);
        assert_eq!(is.len(), 2);
        assert_eq!(*is[0], i1);
    }

    #[test]
    fn hit_all_intersections_pos_t() {
        let s = sphere();
        let i1 = s.intersection(1.0);
        let i2 = s.intersection(2.0);
        let xs = intersections!(&i2, &i1);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i1);
    }

    #[test]
    fn hit_some_intersections_neg_t() {
        let s = sphere();
        let i1 = s.intersection(-1.0);
        let i2 = s.intersection(2.0);
        let xs = intersections!(&i2, &i1);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i2);
    }

    #[test]
    fn hit_all_intersections_neg_t() {
        let s = sphere();
        let i1 = s.intersection(-2.0);
        let i2 = s.intersection(-1.0);
        let xs = intersections!(&i2, &i1);
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
        let xs = intersections!(&i1, &i2, &i3, &i4);
        let i = hit(&xs);
        assert_eq!(*i.unwrap(), i4);
    }
}
