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
use raytracer::chapters::dragons;
use raytracer::chapters::patterns;

/// Run a chapter program
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// program name: exs: chapter11, appendix1
    #[arg(short, long)]
    name: String,

    /// obj file path
    #[arg(short, long, default_value_t = String::from(""))]
    fixture: String,

    /// output resolution
    #[arg(long, default_value_t = 100)]
    hres: usize,
    #[arg(long, default_value_t = 100)]
    vres: usize,
}

fn main() {
    let args = Args::parse();

    match args.name.as_str() {
        "chapter1" => chapter1::run(),
        "chapter2" => chapter2::run(),
        "chapter3" => chapter3::run(),
        "chapter5" => chapter5::run(),
        "chapter7" => chapter7::run(),
        "chapter8" => chapter8::run(args.hres, args.vres),
        "chapter9" => chapter9::run(),
        "chapter11" => chapter11::run(args.hres, args.vres),
        "chapter12" => chapter12::run(),
        "chapter14" => chapter14::run(args.hres, args.vres),
        "chapter15" => chapter15::run(&args.fixture, args.hres, args.vres),
        "patterns" => patterns::run(args.hres, args.vres),
        "appendix1" => appendix1::run(args.hres, args.vres),
        "dragons" => dragons::run(&args.fixture, args.hres, args.vres),
        _ => println!("No such program: {}", args.name),
    }
}
