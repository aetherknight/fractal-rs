//! Library of things to explore and draw various fractal curves.
extern crate argparse;
extern crate graphics;
extern crate piston;
extern crate piston_window;

// must be before any local modules that use the macros
#[macro_use]
mod macros;

pub mod curves;
pub mod geometry;
pub mod lindenmayer;
pub mod turtle;
