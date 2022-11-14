use glm::*;

use crate::math::F3D;

pub type Matrix4 = TMat4<F3D>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn m4x4() {
        let m = Mat4::new(
            1.0, 2.0, 3.0, 4.0, 2.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0, 2.0, 3.0, 4.0, 5.0,
        );
        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 2.0);
        assert_eq!(m[(1, 2)], 4.0);
    }

    #[test]
    fn m4xtuple() {
        let m = Mat4::new(
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        );

        let t = Vec4::new(1.0, 2.0, 3.0, 1.0);
        let t1 = m * t;
        assert_eq!(t1, Vec4::new(18.0, 24.0, 33.0, 1.0));
    }
}
