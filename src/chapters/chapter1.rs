use crate::math::F3D;
use crate::tuple::*;
use std::io;

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

fn simulate(init_vel: F3D) {
    let mut p = Projectile {
        pos: point(0.0, 1.0, 0.0),
        vel: vector(1.0, 1.0, 0.0).normalize() * init_vel,
    };
    let e = Env {
        gravity: vector(0.0, -1.0, 0.0),
        wind: vector(-0.01, 0.0, 0.0),
    };
    let mut ticks: u32 = 0;
    while p.pos.y > 0.0 {
        println!("projectile at: {}", p.pos);
        ticks += 1;
        p = tick(&e, p);
    }
    println!("Initial velocity {}, {} ticks", init_vel, ticks);
}

// Chpt 1
pub fn run() {
    println!("Running chapter1");

    loop {
        println!("Enter starting velocity or Q to quit: ");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                println!("input: {}", input);

                match input.trim() {
                    "q" | "Q" => {
                        break;
                    }
                    tin => {
                        simulate(tin.parse::<F3D>().unwrap_or(0.0));
                    }
                }
            }
            Err(err) => {
                println!("Shit!: {}", err);
                break;
            }
        }
    }
}
