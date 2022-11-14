use glm::*;

use crate::math::F3D;
use crate::matrix::Matrix4;
use crate::tuple::*;

// wrap calls to glm transformation functions
pub fn make_translation(p: Tuple) -> Matrix4 {
    translation(&p.xyz())
}

pub fn make_scaling(p: Tuple) -> Matrix4 {
    scaling(&p.xyz())
}

pub fn make_rotation_x(angle: F3D) -> Matrix4 {
    rotation(angle, &vec3(1.0, 0.0, 0.0))
}

pub fn make_rotation_y(angle: F3D) -> Matrix4 {
    rotation(angle, &vec3(0.0, 1.0, 0.0))
}

pub fn make_rotation_z(angle: F3D) -> Matrix4 {
    rotation(angle, &vec3(0.0, 0.0, 1.0))
}

// glm implementation takes a 4x4matrix - this works
pub fn make_shearing(xy: F3D, xz: F3D, yx: F3D, yz: F3D, zx: F3D, zy: F3D) -> Matrix4 {
    Mat4::new(
        1.0, xy, xz, 0.0, yx, 1.0, yz, 0.0, zx, zy, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn translate_point() {
        let t = make_translation(point(5.0, -3.0, 2.0));
        let p = point(-3.0, 4.0, 5.0);
        let p2 = t * p;
        assert_eq!(p2, point(2.0, 1.0, 7.0));
    }

    #[test]
    fn translate_does_not_affect_vectors() {
        let t = make_translation(point(5.0, -3.0, 2.0));
        let v = vector(-3.0, 4.0, 5.0);
        let v2 = t * v;
        assert_eq!(v, v2);
    }

    #[test]
    fn scale_point() {
        let transform = make_scaling(point(2.0, 3.0, 4.0));
        let p = point(-4.0, 6.0, 8.0);
        let p2 = transform * p;
        assert_eq!(p2, point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn rotate_point_x_axis() {
        let p = point_y();
        let half_quarter = make_rotation_x(quarter_pi());
        let p2 = half_quarter * p;
        let expected = point(0.0, 2_f32.sqrt() / 2.0, 2_f32.sqrt() / 2.0);
        assert_eq!(p2, expected);
    }

    #[test]
    fn shearing() {
        // x in proportion to y
        let p = point(2.0, 3.0, 4.0);
        let t = make_shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p2 = t * p;
        assert_eq!(p2, point(5.0, 3.0, 4.0));

        // z in proportion to y
        let t = make_shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p2 = t * p;
        assert_eq!(p2, point(2.0, 3.0, 7.0));
    }

    #[test]
    fn chained() {
        let p = point(1.0, 0.0, 1.0);
        let a = make_rotation_x(half_pi());
        let b = make_scaling(point(5.0, 5.0, 5.0));
        let c = make_translation(point(10.0, 5.0, 7.0));
        let t = c * b * a;
        let tp = t * p;
        assert_eq!(tp, point(15.0, 0.0, 7.0));
    }
}
