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

extern crate gfx_device_gl;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

// must be before any local modules that use the macros
#[macro_use]
mod macros;

pub mod curves;
pub mod geometry;
pub mod lindenmayer;
pub mod turtle;
mod glwindow;

use std::env;

use curves::cesaro::CesaroFractal;
use curves::cesarotri::CesaroTriFractal;
use curves::dragon::DragonFractal;
use curves::kochcurve::KochCurve;
use curves::levyccurve::LevyCCurve;
use curves::terdragon::TerdragonFractal;
use lindenmayer::LindenmayerSystemTurtleProgram;
use turtle::TurtleProgram;

fn usage(program_name: &str) {
    println!("Usage: {} CURVE ARGS...", program_name);
    println!("");
    println!("CURVEs:");
    println!("    cesaro ITERATION     -- Césaro Square");
    println!("    cesarotri ITERATION  -- Césaro Triangle");
    println!("    dragon ITERATION     -- Dragon Curve");
    println!("    kochcurve ITERATION  -- Koch Snowflake");
    println!("    levyccurve ITERATION -- Lévy C Curve");
    println!("    terdragon ITERATION  -- Terdragon Curve");
    println!("");
    println!("ITERATION should be a a non-negative integer. Note that the complexity of the");
    println!("curves grows exponentially.");
    println!("");
}

fn validate_args(args: &Vec<String>) {
    if (*args).len() <= 1 {
        usage(&(*args)[0]);
        panic!("You must provide a fractal type and an iteration number");
    }
    if (*args).len() <= 2 {
        usage(&(*args)[0]);
        panic!("You must provide an iteration number");
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    validate_args(&args);

    let curve_name: &str = args.get(1).unwrap();
    let iterations: u64 = args.get(2).unwrap().parse::<u64>().unwrap();

    let program: Box<TurtleProgram> = match curve_name.as_ref() {
        "cesaro"     => Box::new(LindenmayerSystemTurtleProgram::new(CesaroFractal::new(iterations).unwrap())),
        "cesarotri"  => Box::new(LindenmayerSystemTurtleProgram::new(CesaroTriFractal::new(iterations).unwrap())),
        "dragon"     => Box::new(DragonFractal::new(iterations).unwrap()),
        "kochcurve"  => Box::new(LindenmayerSystemTurtleProgram::new(KochCurve::new(iterations).unwrap())),
        "levyccurve" => Box::new(LindenmayerSystemTurtleProgram::new(LevyCCurve::new(iterations).unwrap())),
        "terdragon"  => Box::new(LindenmayerSystemTurtleProgram::new(TerdragonFractal::new(iterations).unwrap())),
        _            => panic!("Unknown program type")
    };

    glwindow::run(&*program);
}
