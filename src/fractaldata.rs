// Copyright (c) 2016 William (B.J.) Snow Orvis
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

//! Data structures describing each fractal or curve that the program can draw.
//!
//! The data structures also provide a closure to process configuration for
//! each curve and configure a WindowHandler to handle callbacks from the event
//! loop.

use std::collections::HashMap;
use std::sync::Arc;

use super::chaosgame::barnsleyfern;
use super::chaosgame::sierpinski::SierpinskiChaosGame;
use super::curves::cesaro::CesaroFractal;
use super::curves::cesarotri::CesaroTriFractal;
use super::curves::dragon::DragonFractal;
use super::curves::kochcurve::KochCurve;
use super::curves::levyccurve::LevyCCurve;
use super::curves::terdragon::TerdragonFractal;
use super::escapetime::burningship::*;
use super::escapetime::mandelbrot::Mandelbrot;
use super::lindenmayer::LindenmayerSystemTurtleProgram;
use super::pistonrendering::WindowHandler;
use super::pistonrendering;

/// Print a message to stderr and exit with a non-zero exit code
macro_rules! abort {
    ( $($arg:tt)* ) => {
        {
            use std::io::{stderr, Write};
            use std;
            writeln!(&mut stderr(), $($arg)*).unwrap();
            std::process::exit(1);
        }
    }
}

pub struct Arguments {
    pub curve: String,
    pub iterations: u64,
    pub drawrate: u64,
    pub power: u64,
    pub threadcount: u32,
}

pub struct FractalData {
    pub name: &'static str,
    pub desc: &'static str,
    pub args: &'static [&'static str],
    /// Function or closure that takes in a list of arguments and constructs a WindowHandler for
    /// the curve. It should then call the passed in function pointer and give it the
    /// WindowHandler.
    pub with_window_handler: &'static (Fn(&Arguments, &Fn(&mut WindowHandler)) + Sync),
}

impl FractalData {
    pub fn print_info(&self, dollar_zero: &str) {
        println!("{}", self.name);
        println!("{}", self.desc);
        print!("Usage: {} {}", dollar_zero, self.name);
        for arg in self.args {
            print!(" {}", arg);
        }
        println!("");
    }
}

static BARNSLEYFERN: FractalData = FractalData {
    name: "barnsleyfern",
    desc: "Draws the Barnsley Fern fractal using a chaos game with affine transforms.",
    args: &[],
    with_window_handler: &|args, runner| {
        let game = Arc::new(barnsleyfern::BarnsleyFern::new(&barnsleyfern::REFERENCE_TRANSFORMS,
                                                            &barnsleyfern::REFERENCE_WEIGHTS));
        let mut handler =
            pistonrendering::chaosgame::ChaosGameWindowHandler::new(game, args.drawrate as usize);
        runner(&mut handler);
    },
};

static BURNINGSHIP: FractalData = FractalData {
    name: "burningship",
    desc: "Draws the burning ship fractal",
    args: &["MAX_ITERATIONS", "POWER"],
    with_window_handler: &|args, runner| {
        if args.iterations < 1 {
            abort!("Must specify a MAX_ITERATIONS of 1 or greater!");
        }
        let burningship = Arc::new(BurningShip::new(args.iterations, args.power));
        let mut handler =
            pistonrendering::escapetime::EscapeTimeWindowHandler::new(burningship,
                                                                      args.threadcount);
        runner(&mut handler);
    },
};

static BURNINGMANDEL: FractalData = FractalData {
    name: "burningmandel",
    desc: "Draws a variation of the burning ship fractal",
    args: &["MAX_ITERATIONS", "POWER"],
    with_window_handler: &|args, runner| {
        if args.iterations < 1 {
            abort!("Must specify a MAX_ITERATIONS of 1 or greater!");
        }
        let burningship = Arc::new(BurningMandel::new(args.iterations, args.power));
        let mut handler =
            pistonrendering::escapetime::EscapeTimeWindowHandler::new(burningship,
                                                                      args.threadcount);
        runner(&mut handler);
    },
};

static CESARO: FractalData = FractalData {
    name: "cesaro",
    desc: "Draws a square Césaro fractal. Needs an ITERATION.",
    args: &["ITERATION"],
    with_window_handler: &|args, runner| {
        let program = LindenmayerSystemTurtleProgram::new(CesaroFractal::new(args.iterations));
        let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                   args.drawrate);
        runner(&mut *handler);
    },
};

static CESAROTRI: FractalData = FractalData {
    name: "cesarotri",
    desc: "Draws a triangle Césaro fractal. Needs an ITERATION.",
    args: &["ITERATION"],
    with_window_handler: &|args, runner| {
        let program = LindenmayerSystemTurtleProgram::new(CesaroTriFractal::new(args.iterations));
        let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                   args.drawrate);
        runner(&mut *handler);
    },
};

static DRAGON: FractalData = FractalData {
    name: "dragon",
    desc: "Draws dragon curve fractal. Needs an ITERATION.",
    args: &["ITERATION"],
    with_window_handler: &|args, runner| {
        let program = DragonFractal::new(args.iterations);
        let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                   args.drawrate);
        runner(&mut *handler);
    },
};

static KOCHCURVE: FractalData = FractalData {
    name: "kochcurve",
    desc: "Draws a Koch snowflake. Needs an ITERATION.",
    args: &["ITERATION"],
    with_window_handler: &|args, runner| {
        let program = LindenmayerSystemTurtleProgram::new(KochCurve::new(args.iterations));
        let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                   args.drawrate);
        runner(&mut *handler);
    },
};

static LEVYCCURVE: FractalData = FractalData {
    name: "levyccurve",
    desc: "Draws a Levy C Curve. Needs an ITERATION.",
    args: &["ITERATION"],
    with_window_handler: &|args, runner| {
        let program = LindenmayerSystemTurtleProgram::new(LevyCCurve::new(args.iterations));
        let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                   args.drawrate);
        runner(&mut *handler);
    },
};

static MANDELBROT: FractalData = FractalData {
    name: "mandelbrot",
    desc: "Draws the traditional mandelbrot fractal",
    args: &["MAX_ITERATIONS", "POWER"],
    with_window_handler: &|args, runner| {
        if args.iterations < 1 {
            abort!("Must specify a MAX_ITERATIONS of 1 or greater!");
        }
        let mandelbrot = Arc::new(Mandelbrot::new(args.iterations, args.power));
        let mut handler =
            pistonrendering::escapetime::EscapeTimeWindowHandler::new(mandelbrot, args.threadcount);
        runner(&mut handler);
    },
};

static ROADRUNNER: FractalData = FractalData {
    name: "roadrunner",
    desc: "Draws a variation of the burning ship fractal",
    args: &["MAX_ITERATIONS", "POWER"],
    with_window_handler: &|args, runner| {
        if args.iterations < 1 {
            abort!("Must specify a MAX_ITERATIONS of 1 or greater!");
        }
        let burningship = Arc::new(RoadRunner::new(args.iterations, args.power));
        let mut handler =
            pistonrendering::escapetime::EscapeTimeWindowHandler::new(burningship,
                                                                      args.threadcount);
        runner(&mut handler);
    },
};

static SIERPINSKI: FractalData = FractalData {
    name: "sierpinski",
    desc: "Draws a Sierpinski triangle using a chaos game. It randomly picks 3 points on the \
           screen as the vertices of the triangle, finds the center of the triangle, and then \
           moves halfway to a random vertex. Repeat ad nauseum.",
    args: &["foo"],
    with_window_handler: &|args, runner| {
        let game = Arc::new(SierpinskiChaosGame::new());
        let mut handler =
            pistonrendering::chaosgame::ChaosGameWindowHandler::new(game, args.drawrate as usize);
        runner(&mut handler);
    },
};

static TERDRAGON: FractalData = FractalData {
    name: "terdragon",
    desc: "Draws a terdragon curve. Needs an ITERATION.",
    args: &["ITERATION"],
    with_window_handler: &|args, runner| {
        let program = LindenmayerSystemTurtleProgram::new(TerdragonFractal::new(args.iterations));
        let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                   args.drawrate);
        runner(&mut *handler);
    },
};


pub fn get_chaos_data() -> HashMap<&'static str, &'static FractalData> {
    let mut data = HashMap::new();
    data.insert("barnsleyfern", &BARNSLEYFERN);
    data.insert("burningmandel", &BURNINGMANDEL);
    data.insert("burningship", &BURNINGSHIP);
    data.insert("cesaro", &CESARO);
    data.insert("cesarotri", &CESAROTRI);
    data.insert("dragon", &DRAGON);
    data.insert("kochcurve", &KOCHCURVE);
    data.insert("levyccurve", &LEVYCCURVE);
    data.insert("mandelbrot", &MANDELBROT);
    data.insert("roadrunner", &ROADRUNNER);
    data.insert("sierpinski", &SIERPINSKI);
    data.insert("terdragon", &TERDRAGON);
    data
}
