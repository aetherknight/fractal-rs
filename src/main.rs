// Copyright (c) 2015 William (B.J.) Snow Orvis
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

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;


#[macro_use]
pub mod common;
pub mod dragon;
pub mod kochcurve;
pub mod levyccurve;
pub mod lindenmayer;
pub mod terdragon;
mod glwindow;

use std::env;
use glwindow::{WindowHandler};

use common::TurtleApp;
use dragon::DragonFractal;
use kochcurve::KochCurve;
use levyccurve::LevyCCurve;
use lindenmayer::LindenmayerSystemTurtleApp;
use terdragon::TerdragonFractal;

// TODO: Implement a proper "usage"
fn validate_args(args: &Vec<String>) {
    if (*args).len() <= 1 {
        panic!("You must provide a fractal type and an iteration number");
    }
    if (*args).len() <= 2 {
        panic!("You must provide an iteration number");
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    validate_args(&args);

    let program_name = args.get(1).unwrap();
    let iterations = args.get(2).unwrap().parse::<u64>().unwrap();

    let program: Box<TurtleApp> = match program_name.as_ref() {
        "dragon"     => Box::new(DragonFractal::new(iterations).unwrap()),
        "kochcurve"  => Box::new(LindenmayerSystemTurtleApp::new(KochCurve::new(iterations).unwrap())),
        "levyccurve" => Box::new(LindenmayerSystemTurtleApp::new(LevyCCurve::new(iterations).unwrap())),
        "terdragon"  => Box::new(LindenmayerSystemTurtleApp::new(TerdragonFractal::new(iterations).unwrap())),
        _            => panic!("Unknown program type")
    };

    let window = WindowHandler::new();
    window.run(&*program);
}
