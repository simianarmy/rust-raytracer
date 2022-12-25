extern crate raytracer;

use raytracer::canvas::Canvas;
use raytracer::color::Color;
use raytracer::math::F3D;
use raytracer::ppm::*;
use raytracer::tuple::*;

struct Projectile {
    pos: Tuple,
    vel: Tuple,
}

struct Env {
    gravity: Tuple,
    wind: Tuple,
}

fn tick(env: &Env, proj: Projectile) -> Projectile {
    Projectile {
        pos: proj.pos + proj.vel,
        vel: proj.vel + env.gravity + env.wind,
    }
}

fn simulate(init_vel: F3D, c: &mut Canvas) {
    let (_, c_height) = c.dimensions();
    let mut p = Projectile {
        pos: point(0.0, 1.0, 0.0),
        vel: vector(1.0, 1.8, 0.0).normalize() * init_vel,
    };
    let e = Env {
        gravity: vector(0.0, -0.1, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };
    let mut ticks: u32 = 0;

    while p.pos.y > 0.0 {
        //println!("projectile at: {}", p.pos);
        let y: i32 = (c_height as i32) - (p.pos.y as i32);

        if y >= 0 {
            c.safe_write_pixel(p.pos.x as usize, y as usize, Color::new(0.9, 0.52, 0.5));
        }
        ticks += 1;
        p = tick(&e, p);
    }
    println!("Initial velocity {}, {} ticks", init_vel, ticks);
}

// Chpt 1
fn main() {
    println!("Running chapter2");

    let mut c = Canvas::new(900, 550, None);
    simulate(11.25, &mut c); // only this velocity scaler seems to work

    let filename = "./ppms/chapter2.ppm";
    match create_file_from_data(filename, &c.to_ppm()) {
        Ok(_) => {
            println!("file created ({})!", filename);
        }
        Err(err) => {
            println!("Error writing file! {}", err);
        }
    }
    ()
}
