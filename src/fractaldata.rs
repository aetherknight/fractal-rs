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

use std;
use std::sync::Arc;

use clap;

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
use super::pistonrendering;
use super::pistonrendering::WindowHandler;

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

/// Helper to get the size of a list of expressions
macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
}



pub struct GlobalArguments {
    pub drawrate: u64,
    pub threadcount: u32,
}

// #[derive(Debug)]
pub struct FractalData {
    /// The name of the fractal that will be used on the command line as the sub-command to
    /// actually render this fractal.
    pub name: String,

    /// A short description of the fractal.
    pub summary: String,

    /// A longer description of the fractal.
    pub desc: String,

    /// The names of command line arguments required by this fractal.
    pub args: Box<[&'static str]>,

    // pub clap_subcommand: Box<for<'a, 'b> Fn() -> clap::App<'a, 'b>>,
    // We only use static strs when constructing app.
    pub clap_subcommand: Box<Fn() -> clap::App<'static, 'static>>,

    /// Closure that extracts and parses command line arguments for the fractal, constructs a
    /// WindowHandler for the fractal, and then runs the provided callback to start the event loop
    /// for rendering.
    ///
    /// The arguments are:
    ///
    /// * A clap::ArgMatches for the fractal's clap subcommand.
    /// * Some optional arguments that are set globally.
    /// * A callback that should be called with the constructed WindowHandler for the fractal that
    ///   launches the actual event loop.
    ///
    /// Use the `fractal_data!` macro to construct this.
    pub parse_and_run: Box<(Fn(&clap::ArgMatches, &GlobalArguments, &Fn(&mut WindowHandler)) + Sync)>,
}

fn parse_arg<T>(opt_name: &str, opt_val: &str) -> T
where T: std::str::FromStr,
      <T as std::str::FromStr>::Err: std::fmt::Display
{
    match opt_val.parse::<T>() {
        Err(e) => panic!("Error parsing {}: {}", opt_name, e),
        Ok(v) => v,
    }
}


/// Build functions to access the fractal data using a list of descriptive data structures.
///
/// The macro constructs a standard data structure for all of the fractals that includes
/// automatically generated closures for defining and parsing command line arguments and actually
/// running the fractal/curve's renderer.
///
/// This might eventually become one or more traits that are then implemented for each fractal (eg
/// to separate out command line parsing), but my current approach reasons that the only real
/// differentiator between each fractal's data is how to call it, which should be straightforward
/// to handle with closures (and a macro to keep the descriptive definition of each fractal clean
/// of boilerplate).
///
/// It is also an exercise in what macros can do.
macro_rules! fractal_data {
    ( $($name:ident: { args: [$($argname:ident => $argtype:ty),*], summary: $summary:expr, desc: $desc:expr, run_runner: $run_runner:expr }),+ $(;)*) => (

        /// Returns a FractalData describing the requested fractal, or it returns an Err(&str)
        /// indicating the fractal name is not known.
        // disabling unused_variables just for the closure. If a fractal has no arguments, then
        // `matches` is never used.
        #[allow(unused_variables)]
        pub fn get_fractal_data(name: &str) -> Result<FractalData, &str> {
            match name {
                $(stringify!($name) => {
                    Ok(FractalData {
                        name: stringify!($name).to_string(),
                        summary: $summary.to_string(),
                        desc: $desc.to_string(),
                        args: Box::new([$(stringify!($argname)),*]),
                        clap_subcommand: Box::new(|| {
                            let mut sc = clap::SubCommand::with_name(stringify!($name)).about($summary);

                            let args:&[&str] = &[$(stringify!($argname)),*];

                            sc = args.into_iter()
                                .enumerate()
                                .fold(sc, |sc, (index, arg)| {
                                    sc.arg(clap::Arg::with_name(arg).required(true).index(index as u64 +1))
                                });
                            sc
                        }),
                        parse_and_run: Box::new(|matches, opts, runner| {
                            #[allow(dead_code)]
                            struct SubArgs {
                                drawrate: u64,
                                threadcount: u32,
                                $($argname: $argtype),*
                            }

                            // let runner : &Fn(&mut WindowHandler) = handler;
                            let rr: &Fn(SubArgs, &Fn(&mut WindowHandler)) = &$run_runner;
                            rr(
                                SubArgs {
                                    drawrate: opts.drawrate,
                                    threadcount: opts.threadcount,
                                    $( $argname:
                                       matches.value_of(stringify!($argname)).and_then(|d| Some(parse_arg::<$argtype>(stringify!($argname), d))).unwrap()
                                     ),*
                                }, runner);
                        }),
                    })
                })+
                _ => { Err("Unknown or unsupported fractal type")}
            }
        }

    /// Returns a Vec of FractalDatas describing all of the supported fractals.
    pub fn get_all_fractal_data() -> Vec<FractalData> {
        let mut fd_list = Vec::new();
        // the following _shouldn't_ ever return an Err
        $(if let Ok(fd) = get_fractal_data(stringify!($name)) {
            fd_list.push(fd);
        })+
        fd_list
    }
    )
}

fractal_data!(
    barnsleyfern: {
        args: [],
        summary: "Barnsley Fern (chaos game)",
        desc: "Draws the Barnsley Fern fractal using a chaos game with affine transforms.",
        run_runner: |args, runner| {
            let game = Arc::new(barnsleyfern::BarnsleyFern::new(&barnsleyfern::REFERENCE_TRANSFORMS,
                                                                &barnsleyfern::REFERENCE_WEIGHTS));
            let mut handler =
                pistonrendering::chaosgame::ChaosGameWindowHandler::new(game, args.drawrate);
            runner(&mut handler);
        }
    },

    burningship: {
        args: [max_iterations => u64, power => u64],
        summary: "Burning Ship fractal",
        desc: "Draws the burning ship fractal",
        run_runner: |args, runner| {
            if args.max_iterations < 1 {
                abort!("Must specify a MAX_ITERATIONS of 1 or greater!");
            }
            let burningship = Arc::new(BurningShip::new(args.max_iterations, args.power));
            let mut handler =
                pistonrendering::escapetime::EscapeTimeWindowHandler::new(burningship,
                                                                          args.threadcount);
            runner(&mut handler);
        }
    },

    burningmandel: {
        args: [max_iterations => u64, power => u64],
        summary: "Burning Ship fractal variation",
        desc: "Draws a variation of the burning ship fractal",
        run_runner: |args, runner| {
            if args.max_iterations < 1 {
                abort!("Must specify a MAX_ITERATIONS of 1 or greater!");
            }
            let burningmandel = Arc::new(BurningMandel::new(args.max_iterations, args.power));
            let mut handler =
                pistonrendering::escapetime::EscapeTimeWindowHandler::new(burningmandel,
                                                                          args.threadcount);
            runner(&mut handler);
        }
    },


    cesaro: {
        args: [iteration => u64],
        summary: "Césaro square curve",
        desc: "Draws a square Césaro fractal. Needs an ITERATION.",
        run_runner: |args, runner| {
            let program = LindenmayerSystemTurtleProgram::new(CesaroFractal::new(args.iteration));
            let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                       args.drawrate);
            runner(&mut *handler);
        }
    },

    cesarotri: {
        args: [iteration => u64],
        summary: "Césaro triangle curve",
        desc: "Draws a triangle Césaro fractal. Needs an ITERATION.",
        run_runner: |args, runner| {
            let program = LindenmayerSystemTurtleProgram::new(CesaroTriFractal::new(args.iteration));
            let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                       args.drawrate);
            runner(&mut *handler);
        }
    },

    dragon: {
        args: [iteration => u64],
        summary: "Dragon curve",
        desc: "Draws dragon curve fractal. Needs an ITERATION.",
        run_runner: |args, runner| {
            let program = DragonFractal::new(args.iteration);
            let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                       args.drawrate);
            runner(&mut *handler);
        }
    },

    kochcurve: {
        args: [iteration => u64],
        summary: "Koch snowflake curve",
        desc: "Draws a Koch snowflake. Needs an ITERATION.",
        run_runner: |args, runner| {
            let program = LindenmayerSystemTurtleProgram::new(KochCurve::new(args.iteration));
            let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                       args.drawrate);
            runner(&mut *handler);
        }
    },

    levyccurve: {
        args: [iteration => u64],
        summary: "Levy C Curve",
        desc: "Draws a Levy C Curve. Needs an ITERATION.",
        run_runner: |args, runner| {
            let program = LindenmayerSystemTurtleProgram::new(LevyCCurve::new(args.iteration));
            let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                       args.drawrate);
            runner(&mut *handler);
        }
    },

    mandelbrot: {
        args: [max_iterations => u64, power => u64],
        summary: "Mandelbrot fractal",
        desc: "Draws the traditional mandelbrot fractal",
        run_runner: |args, runner| {
            if args.max_iterations < 1 {
                abort!("Must specify a MAX_ITERATIONS of 1 or greater!");
            }
            let mandelbrot = Arc::new(Mandelbrot::new(args.max_iterations, args.power));
            let mut handler =
                pistonrendering::escapetime::EscapeTimeWindowHandler::new(mandelbrot, args.threadcount);
            runner(&mut handler);
        }
    },

    roadrunner:  {
        args: [max_iterations => u64, power => u64],
        summary:"Roadrunner fractal (burning ship variation)",
        desc: "Draws a variation of the burning ship fractal",
        run_runner:|args, runner| {
            if args.max_iterations < 1 {
                abort!("Must specify a MAX_ITERATIONS of 1 or greater!");
            }
            let burningship = Arc::new(RoadRunner::new(args.max_iterations, args.power));
            let mut handler =
                pistonrendering::escapetime::EscapeTimeWindowHandler::new(burningship,
                                                                          args.threadcount);
            runner(&mut handler);
        }
    },

    sierpinski: {
        args: [],
        summary: "Sierpinski triangle (chaos game)",
        desc: "Draws a Sierpinski triangle using a chaos game. It randomly picks 3 points on the screen as the vertices of the triangle, finds the center of the triangle, and then moves halfway to a random vertex. Repeat ad nauseum.",
        run_runner: |args, runner| {
            let game = Arc::new(SierpinskiChaosGame::new());
            let mut handler =
                pistonrendering::chaosgame::ChaosGameWindowHandler::new(game, args.drawrate);
            runner(&mut handler);
        }
    },

    terdragon: {
        args: [iteration => u64],
        summary:"Terdragon curve",
        desc:"Draws a terdragon curve. Needs an ITERATION.",
        run_runner:|args, runner| {
            let program = LindenmayerSystemTurtleProgram::new(TerdragonFractal::new(args.iteration));
            let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                       args.drawrate);
            runner(&mut *handler);
        }
    }

);
