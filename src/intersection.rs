use crate::computations::Computations;
use crate::math::*;
use crate::object::*;
use std::clone::Clone;
use std::fmt;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct Intersection {
    pub t: F3D,
    pub object: Arc<Object>,
    pub u: F3D,
    pub v: F3D,
}

impl Intersection {
    pub fn new(object: &Object, t: F3D) -> Self {
        Self::with_uv(object, t, 0.0, 0.0)
    }

    pub fn with_uv(object: &Object, t: F3D, u: F3D, v: F3D) -> Self {
        Self {
            object: Arc::new(object.clone()), // sorry lifetimes, you're too annoying
            t,
            u,
            v,
        }
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

#[derive(Clone, Debug)]
pub struct Intersections {
    intersections: Vec<Intersection>,
}

impl Intersections {
    pub fn from_intersections(intersections: Vec<Intersection>) -> Self {
        let mut is = Self::new();
        is.intersections = intersections;
        is.sort_intersections()
    }

    pub fn new() -> Self {
        Self {
            intersections: Vec::<Intersection>::with_capacity(16),
        }
    }

    pub fn vec(&self) -> &Vec<Intersection> {
        &self.intersections
    }

    pub fn len(&self) -> usize {
        self.intersections.len()
    }

    pub fn is_empty(&self) -> bool {
        self.intersections.is_empty()
    }

    pub fn push(&mut self, is: Intersection) {
        self.intersections.push(is);
    }

    pub fn iter(&self) -> std::slice::Iter<Intersection> {
        self.intersections.iter()
    }

    pub fn extend(&mut self, is: &Intersections) {
        for is in is.intersections.iter() {
            self.intersections.push(is.clone());
        }
    }

    pub fn sort_intersections(mut self) -> Self {
        self.intersections
            .sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        self
    }

    /**
     * "Closest" intersection in a collection
     */
    pub fn hit(&self) -> Option<&Intersection> {
        // filter out negative t values here
        self.intersections.iter().find(|i| i.t >= 0.0)
    }
}

// intersections[i]
impl std::ops::Index<usize> for Intersections {
    type Output = Intersection;

    fn index(&self, i: usize) -> &Intersection {
        &self.intersections[i]
    }
}

impl FromIterator<Intersection> for Intersections {
    fn from_iter<I: IntoIterator<Item = Intersection>>(iter: I) -> Self {
        let mut c = Intersections::new();

        for i in iter {
            c.push(i);
        }

        c
    }
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
    use crate::shapes::sphere::*;
    use crate::shapes::triangle::*;
    use crate::transformation::*;
    use crate::tuple::*;
    use std::clone::Clone;

    #[test]
    fn intersections_macro_builds_list_from_args() {
        let s = sphere();
        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);
        let is = Intersections::from_intersections(vec![i1.clone(), i2]);
        assert_eq!(is.len(), 2);
        assert_eq!(is[0], i1);
    }

    #[test]
    fn hit_all_intersections_pos_t() {
        let s = sphere();
        let i1 = Intersection::new(&s, 1.0);
        let i2 = Intersection::new(&s, 2.0);
        let is = Intersections::from_intersections(vec![i1.clone(), i2]);
        let i = is.hit();
        assert_eq!(i, Some(&i1));
    }

    #[test]
    fn hit_some_intersections_neg_t() {
        let s = sphere();
        let i1 = Intersection::new(&s, -1.0);
        let i2 = Intersection::new(&s, 2.0);
        let is = Intersections::from_intersections(vec![i1, i2.clone()]);
        let i = is.hit();
        assert_eq!(i, Some(&i2));
    }

    #[test]
    fn hit_all_intersections_neg_t() {
        let s = sphere();
        let i1 = Intersection::new(&s, -2.0);
        let i2 = Intersection::new(&s, -1.0);
        let is = Intersections::from_intersections(vec![i1, i2]);
        let i = is.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn hit_always_lowest_nonneg_t() {
        let s = sphere();
        let i1 = Intersection::new(&s, 5.0);
        let i2 = Intersection::new(&s, 7.0);
        let i3 = Intersection::new(&s, -3.0);
        let i4 = Intersection::new(&s, 2.0);
        let is = Intersections::from_intersections(vec![i1, i2, i3, i4.clone()]);
        let i = is.hit();
        assert_eq!(i, Some(&i4));
    }

    #[test]
    fn hit_should_offset_point() {
        let r = Ray::new(point(0.0, 0.0, -5.0), vector_z());
        let mut s = sphere();
        s.set_transform(&make_translation(0.0, 0.0, 1.0));
        let i = Intersection::new(&s, 5.0);
        let comps =
            prepare_computations(&i, &r, &Intersections::from_intersections(vec![i.clone()]));
        assert!(comps.over_point.z < -crate::math::EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn schlick_under_total_internal_reflection() {
        let sphere = glass_sphere();
        let ray = Ray::new(point(0.0, 0.0, SQRT_2_DIV_2), vector_y());
        let xs = Intersections::from_intersections(vec![
            Intersection::new(&sphere, -SQRT_2_DIV_2),
            Intersection::new(&sphere, SQRT_2_DIV_2),
        ]);
        let comps = prepare_computations(&xs[1], &ray, &xs);
        println!("comps: {:?}", comps);
        let reflectance = schlick(&comps);
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn schlick_with_perpendicular_viewing_angle() {
        let sphere = glass_sphere();
        let ray = Ray::new(point_zero(), vector_y());
        let xs = Intersections::from_intersections(vec![
            Intersection::new(&sphere, -1.0),
            Intersection::new(&sphere, 1.0),
        ]);
        let comps = prepare_computations(&xs[1], &ray, &xs);
        let reflectance = schlick(&comps);
        assert_eq_feps!(reflectance, 0.04);
    }

    #[test]
    fn schlick_with_small_angle() {
        let sphere = glass_sphere();
        let ray = Ray::new(point(0.0, 0.99, -2.0), vector_z());
        let xs = Intersections::from_intersections(vec![Intersection::new(&sphere, 1.8589)]);
        let comps = prepare_computations(&xs[0], &ray, &xs);
        let reflectance = schlick(&comps);
        assert_eq_feps!(reflectance, 0.48873);
    }

    #[test]
    fn can_encapsulate_u_v() {
        let s = triangle(point_y(), point(-1.0, 0.0, 0.0), point_x());
        let i = Intersection::with_uv(&s, 3.5, 0.2, 0.4);
        assert_eq!(i.u, 0.2);
        assert_eq!(i.v, 0.4);
    }
}
