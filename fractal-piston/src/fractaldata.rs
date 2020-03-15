// Copyright (c) 2015-2019 William (B.J.) Snow Orvis
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
//! The data structures also provide a closure to process configuration for each curve and
//! configure a `WindowHandler` to handle callbacks from the event loop.

use clap;
use std;
use std::sync::Arc;

use super::pistonrendering;
use fractal_lib::chaosgame::barnsleyfern;
use fractal_lib::chaosgame::sierpinski::SierpinskiChaosGame;
use fractal_lib::chaosgame::ChaosGameMoveIterator;
use fractal_lib::curves::cesaro::CesaroFractal;
use fractal_lib::curves::cesarotri::CesaroTriFractal;
use fractal_lib::curves::dragon::DragonFractal;
use fractal_lib::curves::kochcurve::KochCurve;
use fractal_lib::curves::levyccurve::LevyCCurve;
use fractal_lib::curves::terdragon::TerdragonFractal;
use fractal_lib::escapetime::burningship::*;
use fractal_lib::escapetime::mandelbrot::Mandelbrot;
use fractal_lib::escapetime::EscapeTime;
use fractal_lib::lindenmayer::LindenmayerSystemTurtleProgram;
use fractal_lib::turtle::TurtleProgram;

/// Helper to get the size of a list of expressions
#[allow(unused_macros)]
macro_rules! count_exprs {
    () => (0);
    ($head:expr) => (1);
    ($head:expr, $($tail:expr),*) => (1 + count_exprs!($($tail),*));
}

/// Helper to extract and parse a value from a command line argument.
macro_rules! extract {
    ($matches:expr, $name:expr) => {
        $matches
            .value_of($name)
            .map(|s| parse_arg($name, s))
            .unwrap_or_else(|| Err(format!("Missing {}", $name)))
    };
}

/// Method to help with parsing a command line argument into some other type.
pub fn parse_arg<T>(opt_name: &str, opt_val: &str) -> Result<T, String>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Display,
{
    match opt_val.parse::<T>() {
        Err(e) => Err(format!("Error parsing {}: {}", opt_name, e)),
        Ok(v) => Ok(v),
    }
}

/// A subcommand that can configure and run a particular fractal renderer.
pub trait FractalSubcommand {
    /// Returns a clap::App definition of this subcommand. The command line arguments it
    /// specifies
    /// should then be handled by the `run()` method.
    fn command(&self) -> clap::App<'static, 'static>;

    /// Runs the command with the given command line arguments and the provided callback.
    fn run(&self, matches: &clap::ArgMatches) -> Result<(), String>;
}

pub struct ChaosGameCommand<E>
where
    E: ChaosGameMoveIterator,
{
    name: &'static str,
    description: &'static str,
    ctor: Box<dyn Fn() -> E>,
}

impl<E> ChaosGameCommand<E>
where
    E: ChaosGameMoveIterator,
{
    pub fn new(
        name: &'static str,
        description: &'static str,
        ctor: Box<dyn Fn() -> E>,
    ) -> ChaosGameCommand<E> {
        ChaosGameCommand {
            name,
            description,
            ctor,
        }
    }
}

impl<E> FractalSubcommand for ChaosGameCommand<E>
where
    E: ChaosGameMoveIterator + 'static,
{
    fn command(&self) -> clap::App<'static, 'static> {
        clap::SubCommand::with_name(self.name)
            .about(self.description)
            .arg(
                clap::Arg::with_name("drawrate")
                    .takes_value(true)
                    .help("The number of points to draw per frame")
                    .long("drawrate")
                    .value_name("MPF")
                    .default_value("1"),
            )
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<(), String> {
        let drawrate = extract!(matches, "drawrate")?;

        let game = Box::new((self.ctor)());
        let mut handler = pistonrendering::chaosgame::ChaosGameWindowHandler::new(game, drawrate);
        pistonrendering::run(&mut handler);

        Ok(())
    }
}

pub struct EscapeTimeCommand<E>
where
    E: EscapeTime + Send + Sync,
{
    name: &'static str,
    description: &'static str,
    ctor: Box<dyn Fn(u64, u64) -> E>,
}

impl<E> EscapeTimeCommand<E>
where
    E: EscapeTime + Send + Sync,
{
    pub fn new(
        name: &'static str,
        description: &'static str,
        ctor: Box<dyn Fn(u64, u64) -> E>,
    ) -> EscapeTimeCommand<E> {
        EscapeTimeCommand {
            name,
            description,
            ctor,
        }
    }
}

impl<E> FractalSubcommand for EscapeTimeCommand<E>
where
    E: EscapeTime + Send + Sync + 'static,
{
    fn command(&self) -> clap::App<'static, 'static> {
        clap::SubCommand::with_name(self.name)
            .about(self.description)
            .arg(
                clap::Arg::with_name("MAX_ITERATIONS")
                    .required(true)
                    .index(1)
                    .help(
                        "The maximum number of iterations of the escape time function before \
                         deciding the fracal has escaped",
                    ),
            )
            .arg(
                clap::Arg::with_name("POWER")
                    .required(true)
                    .index(2)
                    .help("The exponent used in the escape time function (positive integer)"),
            )
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<(), String> {
        let max_iterations = r#try!(extract!(matches, "MAX_ITERATIONS"));
        // .unwrap_or_else(|| return Err("Must specify a MAX_ITERATIONS of 1 or greater!"));
        let power = r#try!(extract!(matches, "POWER"));
        // .unwrap_or_else(|| return Err("Must specify a POWER of 1 or greater!"));

        // The ctor callback can return a raw object that implements EscapeTime because this method
        // is templated to E instead of handling a boxed object that implements EscapeTime.
        //
        // We could alternately avoid using templating, in which case the callback would have to
        // return an Arc<EscapeTime> in order to abstract away the implementation of the trait.
        let et = Arc::new((self.ctor)(max_iterations, power));
        // TODO: `et` when passed in here wants E to be constraint by `'static`. Why?
        let mut handler = pistonrendering::escapetime::EscapeTimeWindowHandler::new(et);
        pistonrendering::run(&mut handler);

        Ok(())
    }
}

pub struct TurtleCommand<E>
where
    E: TurtleProgram,
{
    name: &'static str,
    description: &'static str,
    ctor: Box<dyn Fn(u64) -> E>,
}

impl<E> TurtleCommand<E>
where
    E: TurtleProgram,
{
    pub fn new(
        name: &'static str,
        description: &'static str,
        ctor: Box<dyn Fn(u64) -> E>,
    ) -> TurtleCommand<E> {
        TurtleCommand {
            name,
            description,
            ctor,
        }
    }
}

impl<E> FractalSubcommand for TurtleCommand<E>
where
    E: TurtleProgram + 'static,
{
    fn command(&self) -> clap::App<'static, 'static> {
        clap::SubCommand::with_name(self.name)
            .about(self.description)
            .arg(
                clap::Arg::with_name("drawrate")
                    .takes_value(true)
                    .help("The number of points to draw per frame")
                    .long("drawrate")
                    .value_name("MPF")
                    .default_value("1"),
            )
            .arg(clap::Arg::with_name("ITERATION").required(true).index(1))
    }

    fn run(&self, matches: &clap::ArgMatches) -> Result<(), String> {
        let drawrate = r#try!(extract!(matches, "drawrate"));
        let iteration = r#try!(extract!(matches, "ITERATION"));
        // .unwrap_or_else(|| Err("Must specify an ITERATION of 1 or greater!"));

        let program = (self.ctor)(iteration);
        let mut handler =
            pistonrendering::turtle::construct_turtle_window_handler(&program, drawrate);
        pistonrendering::run(&mut *handler);

        Ok(())
    }
}

macro_rules! define_subcommands {
    ( $($name:ident: $expr:expr),+ ) => {

        pub fn add_subcommands<'a, 'b>(app: clap::App<'a, 'b>) -> clap::App<'a, 'b> {
            app $(.subcommand(($expr).command()))+
        }

        pub fn run_subcommand(app_argmatches: &clap::ArgMatches) -> Result<(), String> {
            match app_argmatches.subcommand() {
                $(
                    (stringify!($name), Some(args)) => {
                        ($expr).run(&args)
                    }
                 )+
                    _ => Err("Unknown subcommand".to_string()),
            }
        }
    }
}

define_subcommands! {
    barnsleyfern: {
        ChaosGameCommand::new(
            "barnsleyfern",
            "Draws the Barnsley Fern fractal using a chaos game with affine transforms.",
            Box::new(|| {
                barnsleyfern::BarnsleyFern::new(
                    &barnsleyfern::REFERENCE_TRANSFORMS,
                    &barnsleyfern::REFERENCE_WEIGHTS
                    )
            })
        )
    },

    burningship: {
        EscapeTimeCommand::new(
            "burningship",
            "Draws the burning ship fractal",
            Box::new(|max_iterations, power| {
                BurningShip::new(max_iterations, power)
            })
        )
    },

    burningmandel: {
        EscapeTimeCommand::new(
            "burningmandel",
            "Draws a variation of the burning ship fractal",
            Box::new(|max_iterations, power| {
                BurningMandel::new(max_iterations, power)
            })
        )
    },

    cesaro: {
        TurtleCommand::new(
            "cesaro",
            "Draws a square Césaro fractal",
            Box::new(|iteration| {
                LindenmayerSystemTurtleProgram::new(CesaroFractal::new(iteration))
            })
        )
    },

    cesarotri: {
        TurtleCommand::new(
            "cestarotri",
            "Draws a triangle Césaro fractal",
            Box::new(|iteration| {
                LindenmayerSystemTurtleProgram::new(CesaroTriFractal::new(iteration))
            })
        )
    },

    dragon: {
        TurtleCommand::new(
            "dragon",
            "Draws a dragon curve fractal",
            Box::new(DragonFractal::new)
        )
    },

    kochcurve: {
        TurtleCommand::new(
            "kochcurve",
            "Draws a Koch snowflake curve",
            Box::new(|iteration| {
                LindenmayerSystemTurtleProgram::new(KochCurve::new(iteration))
            })
        )
    },

    levyccurve: {
        TurtleCommand::new(
            "levyccurve",
            "Draws a Levy C Curve",
            Box::new(|iteration| {
                LindenmayerSystemTurtleProgram::new(LevyCCurve::new(iteration))
            })
        )
    },

    mandelbrot: {
        EscapeTimeCommand::new(
            "mandelbrot",
            "Draws the mandelbrot fractal",
            Box::new(|max_iterations, power| {
                Mandelbrot::new(max_iterations, power)
            })
        )
    },

    roadrunner: {
        EscapeTimeCommand::new(
            "roadrunner",
            "Draws a variation of the burning ship fractal",
            Box::new(|max_iterations, power| {
                RoadRunner::new(max_iterations, power)
            })
        )
    },

    sierpinski: {
        let ctor = Box::new(SierpinskiChaosGame::new);
        ChaosGameCommand::new(
            "sierpinski",
            "Draws a Sierpinski triangle using a chaos game and 3 randomly chosen points on the \
            screen",
            ctor
        )
    },

    terdragon: {
        TurtleCommand::new(
            "terdragon",
            "Draws a terdragon curve",
            Box::new(|iteration| {
                LindenmayerSystemTurtleProgram::new(TerdragonFractal::new(iteration))
            })
        )
    }

}
