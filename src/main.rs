// extern crate elmesque;
extern crate piston;
// extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
// extern crate gfx_graphics;
// extern crate gfx_device_gl;
// extern crate gfx;

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
