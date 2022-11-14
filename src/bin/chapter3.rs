extern crate nalgebra_glm as glm;
extern crate raytracer;

use glm::*;
use raytracer::matrix::Matrix4;

fn main() {
    // Invert an identity matrix
    let identity: Matrix4 = identity();
    println!("identity matrix: {}", identity);
    println!("inverse identity matrix: {}", inverse(&identity));

    // m * inverse(m)
    let m = Mat4::new(
        1.0, 0.0, 0.0, 0.0, 0.0, 2.0, 0.0, 0.0, 0.0, 0.0, 3.0, 0.0, 0.0, 0.0, 0.0, 1.0,
    );
    println!("m: {}", m);
    println!("inverse m: {}", inverse(&m));
    println!("m * inverse(m): {}", m * inverse(&m));
    println!("inverse(transpose(m)): {}", inverse(&transpose(&m)));
    println!("transpose(inverse(m)): {}", inverse_transpose(m));
}
