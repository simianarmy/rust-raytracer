extern crate raytracer;
use clap::Parser;

use raytracer::chapters::appendix1;
use raytracer::chapters::chapter1;
use raytracer::chapters::chapter11;
use raytracer::chapters::chapter12;
use raytracer::chapters::chapter14;
use raytracer::chapters::chapter15;
use raytracer::chapters::chapter2;
use raytracer::chapters::chapter3;
use raytracer::chapters::chapter5;
use raytracer::chapters::chapter7;
use raytracer::chapters::chapter8;
use raytracer::chapters::chapter9;

/// Run a chapter program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Book chapter
    #[arg(short, long)]
    chapter: u8,

    /// obj file path
    #[arg(short, long, default_value_t = String::from(""))]
    fixture: String,

    /// output resolution: 1 = low, 2 = medium, 3 = high
    #[arg(short, long, default_value_t = 1)]
    res: u8,
}

fn main() {
    let args = Args::parse();
    let (hsize, vsize) = match args.res {
        1 => (100, 50),
        2 => (300, 150),
        _ => (500, 250),
    };

    match args.chapter {
        1 => chapter1::run(),
        2 => chapter2::run(),
        3 => chapter3::run(),
        5 => chapter5::run(),
        7 => chapter7::run(),
        8 => chapter8::run(),
        9 => chapter9::run(),
        11 => chapter11::run(),
        12 => chapter12::run(),
        14 => chapter14::run(hsize, vsize),
        15 => chapter15::run(&args.fixture, hsize, vsize),
        16 => appendix1::run(hsize, vsize),
        _ => println!("No such chapter: {}", args.chapter),
    }
}
