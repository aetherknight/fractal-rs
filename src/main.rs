extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

#[macro_use]
pub mod common;
pub mod dragon;
mod glwindow;

use dragon::DragonFractal;
use glwindow::{WindowHandler};

fn main() {
    let window = WindowHandler::new();
    let dr = DragonFractal::new(17).unwrap();
    window.run(&dr);
}
