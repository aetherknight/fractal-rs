// Copyright (c) 2015-2016 William (B.J.) Snow Orvis
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate clap;
extern crate gfx_device_gl;
extern crate graphics;
extern crate image;
extern crate num;
extern crate num_cpus;
extern crate piston;
extern crate piston_window;
extern crate rand;
extern crate rustc_serialize;
extern crate time;

// must be before any local modules that use the macros
#[macro_use]
mod macros;

pub mod chaosgame;
pub mod color;
pub mod curves;
pub mod escapetime;
pub mod fractaldata;
pub mod geometry;
pub mod lindenmayer;
pub mod pistonrendering;
pub mod turtle;
pub mod work_multiplexer;

fn main() {
    // Command line arguments specification
    let mut app = clap::App::new("fractal")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Renders fractals in another window.");
    app = fractaldata::add_subcommands(app);

    let matches = app.get_matches();
    let result = fractaldata::run_subcommand(&matches);

    match result {
        Ok(_) => {}
        Err(e) => {
            use std;
            use std::io::{stderr, Write};
            writeln!(&mut stderr(), "{}", e).unwrap();
            std::process::exit(1);
        }
    }
}
