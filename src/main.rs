extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::env;

#[macro_use]
pub mod common;
pub mod dragon;
mod glwindow;

use dragon::DragonFractal;
use glwindow::{WindowHandler};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        panic!("You must provide an iteration number");
    }
    let iterations = args.get(1).unwrap().parse::<u64>().unwrap();

    let window = WindowHandler::new();
    let dr = DragonFractal::new(iterations).unwrap();
    window.run(&dr);
}
