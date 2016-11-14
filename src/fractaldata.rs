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

use clap;
use std;
use std::sync::Arc;

use super::chaosgame::ChaosGame;
use super::chaosgame::barnsleyfern;
use super::chaosgame::sierpinski::SierpinskiChaosGame;
use super::curves::cesaro::CesaroFractal;
use super::curves::cesarotri::CesaroTriFractal;
use super::curves::dragon::DragonFractal;
use super::curves::kochcurve::KochCurve;
use super::curves::levyccurve::LevyCCurve;
use super::curves::terdragon::TerdragonFractal;
use super::escapetime::EscapeTime;
use super::escapetime::burningship::*;
use super::escapetime::mandelbrot::Mandelbrot;
use super::lindenmayer::LindenmayerSystemTurtleProgram;
use super::pistonrendering;
use super::turtle::TurtleProgram;

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

/// Helper to extract and parse a value from a command line argument.
macro_rules! extract {
    ($matches:expr, $name:expr) => {
        $matches.value_of($name).map(|s| parse_arg($name, s) )
    }
}

/// Method to help with parsing a command line argument into some other type.
fn parse_arg<T>(opt_name: &str, opt_val: &str) -> T
where T: std::str::FromStr,
      <T as std::str::FromStr>::Err: std::fmt::Display
{
    match opt_val.parse::<T>() {
        Err(e) => panic!("Error parsing {}: {}", opt_name, e),
        Ok(v) => v,
    }
}

/// A subcommand that can configure and run a particular fractal renderer.
pub trait FractalSubcommand {
    /// Returns a clap::App definition of this subcommand. The command line
    /// arguments it specifies
    /// should then be handled by the `run()` method.
    fn command(&self) -> clap::App<'static, 'static>;

    /// Runs the command with the given command line arguments and the provided
    /// callback.
    fn run(&self, matches: &clap::ArgMatches);
}

pub struct ChaosGameCommand<E> where E: ChaosGame + Send + Sync {
    name: &'static str,
    description: &'static str,
    ctor: Box<Fn() -> E>,
}

impl<E> ChaosGameCommand<E> where E: ChaosGame + Send + Sync {
    pub fn new(name: &'static str,
               description: &'static str,
               ctor: Box<Fn() -> E>)
        -> ChaosGameCommand<E> {
            ChaosGameCommand {
                name: name,
                description: description,
                ctor: ctor,
            }
        }
}

impl<E> FractalSubcommand for ChaosGameCommand<E> where E: ChaosGame + Send + Sync + 'static {
    fn command(&self) -> clap::App<'static, 'static> {
        clap::SubCommand::with_name(self.name)
            .about(self.description)
            .arg(clap::Arg::with_name("drawrate")
                 .takes_value(true)
                 .help("The number of points to draw per frame")
                 .long("drawrate")
                 .value_name("MPF")
                 .default_value("1"))
    }

    fn run(&self, matches: &clap::ArgMatches) {
        let drawrate = extract!(matches, "drawrate").unwrap_or(1);

        let game = Arc::new((self.ctor)());
        let mut handler = pistonrendering::chaosgame::ChaosGameWindowHandler::new(game, drawrate);
        pistonrendering::run(&mut handler);
    }
}

pub struct EscapeTimeCommand<E>
where E: EscapeTime + Send + Sync
{
    name: &'static str,
    description: &'static str,
    ctor: Box<Fn(u64, u64) -> E>,
}

impl<E> EscapeTimeCommand<E>
where E: EscapeTime + Send + Sync
{
    pub fn new(name: &'static str,
               description: &'static str,
               ctor: Box<Fn(u64, u64) -> E>)
        -> EscapeTimeCommand<E> {
            EscapeTimeCommand {
                name: name,
                description: description,
                ctor: ctor,
            }
        }
}

impl<E> FractalSubcommand for EscapeTimeCommand<E>
where E: EscapeTime + Send + Sync + 'static
{
    fn command(&self) -> clap::App<'static, 'static> {
        clap::SubCommand::with_name(self.name)
            .about(self.description)
            .arg(clap::Arg::with_name("threadcount")
                 .long("threadcount")
                 .takes_value(true)
                 .value_name("N")
                 .help("The number of worker threads to use for rendering")
                 .default_value("1"))
            .arg(clap::Arg::with_name("MAX_ITERATIONS")
                 .required(true)
                 .index(1)
                 .help("The maximum number of iterations of the escape time function before \
                       deciding the fracal has escaped"))
            .arg(clap::Arg::with_name("POWER")
                 .required(true)
                 .index(2)
                 .help("The exponent used in the escape time function (positive integer)"))
    }

    fn run(&self, matches: &clap::ArgMatches) {
        let threadcount = extract!(matches, "threadcount").unwrap_or(1);
        let max_iterations = extract!(matches, "MAX_ITERATIONS")
            .unwrap_or_else(|| abort!("Must specify a MAX_ITERATIONS of 1 or greater!"));
        let power = extract!(matches, "POWER")
            .unwrap_or_else(|| abort!("Must specify a POWER of 1 or greater!"));

        // The ctor callback can return a raw object that implements EscapeTime because
        // this method
        // is templated to E instead of handling a boxed object that implements
        // EscapeTime.
        //
        // We could alternately avoid using templating, in which case the callback
        // would have to
        // return an Arc<EscapeTime> in order to abstract away the implementation of
        // the trait.
        let et = Arc::new((self.ctor)(max_iterations, power));
        // TODO: `et` when passed in here wants E to be constraint by `'static`. Why?
        let mut handler = pistonrendering::escapetime::EscapeTimeWindowHandler::new(et,
                                                                                    threadcount);
        pistonrendering::run(&mut handler);
    }
}

pub struct TurtleCommand<E> where E: TurtleProgram {
    name: &'static str,
    description: &'static str,
    ctor: Box<Fn(u64) -> E>,
}

impl<E> TurtleCommand<E> where E: TurtleProgram {
    pub fn new(name: &'static str,
               description: &'static str,
               ctor: Box<Fn(u64) -> E>)
        -> TurtleCommand<E> {
            TurtleCommand {
                name: name,
                description: description,
                ctor: ctor,
            }
        }
}

impl<E> FractalSubcommand for TurtleCommand<E> where E: TurtleProgram + 'static {
    fn command(&self) -> clap::App<'static, 'static> {
        clap::SubCommand::with_name(self.name)
            .about(self.description)
            .arg(clap::Arg::with_name("drawrate")
                 .takes_value(true)
                 .help("The number of points to draw per frame")
                 .long("drawrate")
                 .value_name("MPF")
                 .default_value("1"))
            .arg(clap::Arg::with_name("ITERATION")
                 .required(true)
                 .index(1))
    }

    fn run(&self, matches: &clap::ArgMatches) {
        let drawrate = extract!(matches, "drawrate").unwrap_or(1);
        let iteration = extract!(matches, "ITERATION")
            .unwrap_or_else(|| abort!("Must specify an ITERATION of 1 or greater!"));

        let program = (self.ctor)(iteration);
        let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program,
                                                                                   drawrate);
        pistonrendering::run(&mut *handler);
    }
}

pub fn barnsleyfern_command() -> clap::App<'static, 'static> {
    let ctor = Box::new(|| barnsleyfern::BarnsleyFern::new(&barnsleyfern::REFERENCE_TRANSFORMS,
                                                           &barnsleyfern::REFERENCE_WEIGHTS));
    let cgc = ChaosGameCommand::new("barnsleyfern", "Draws the Barnsley Fern fractal using a chaos game with affine transforms.", ctor);
    cgc.command()
}

pub fn barnsleyfern_run(matches: &clap::ArgMatches) {
    let ctor = Box::new(|| barnsleyfern::BarnsleyFern::new(&barnsleyfern::REFERENCE_TRANSFORMS,
                                                           &barnsleyfern::REFERENCE_WEIGHTS));
    let cgc = ChaosGameCommand::new("barnsleyfern", "Draws the Barnsley Fern fractal using a chaos game with affine transforms.", ctor);
    cgc.run(matches);
}


pub fn burningship_command() -> clap::App<'static, 'static> {
    let ctor = Box::new(|max_iterations, power| BurningShip::new(max_iterations, power));
    let etc = EscapeTimeCommand::new("burningship", "Draws the burning ship fractal", ctor);
    etc.command()
}

pub fn burningship_run(matches: &clap::ArgMatches) {
    let ctor = Box::new(|max_iterations, power| BurningShip::new(max_iterations, power));
    let etc = EscapeTimeCommand::new("burningship", "Draws the burning ship fractal", ctor);
    etc.run(matches);
}

pub fn burningmandel_command() -> clap::App<'static, 'static> {
    let ctor = Box::new(|max_iterations, power| BurningMandel::new(max_iterations, power));
    let etc = EscapeTimeCommand::new("burningmandel",
                                     "Draws a variation of the burning ship fractal",
                                     ctor);
    etc.command()
}

pub fn burningmandel_run(matches: &clap::ArgMatches) {
    let ctor = Box::new(|max_iterations, power| BurningMandel::new(max_iterations, power));
    let etc = EscapeTimeCommand::new("burningmandel",
                                     "Draws a variation of the burning ship fractal",
                                     ctor);
    etc.run(matches);
}

pub fn cesaro_command() -> clap::App<'static, 'static> {
    let tc = TurtleCommand::new("cesaro",
                                "Draws a square Césaro fractal",
                                Box::new(|iteration| LindenmayerSystemTurtleProgram::new(CesaroFractal::new(iteration))));
    tc.command()
}

pub fn cesaro_run(matches: &clap::ArgMatches) {
    let tc = TurtleCommand::new("cesaro",
                                "Draws a square Césaro fractal",
                                Box::new(|iteration| LindenmayerSystemTurtleProgram::new(CesaroFractal::new(iteration))));
    tc.run(matches);
}

pub fn cesarotri_command() -> clap::App<'static, 'static> {
    let tc = TurtleCommand::new("cestarotri",
                                "Draws a triangle Césaro fractal",
                                Box::new(|iteration| LindenmayerSystemTurtleProgram::new(CesaroTriFractal::new(iteration))));
    tc.command()
}

pub fn cesarotri_run(matches: &clap::ArgMatches) {
    let tc = TurtleCommand::new("cestarotri",
                                "Draws a triangle Césaro fractal",
                                Box::new(|iteration| LindenmayerSystemTurtleProgram::new(CesaroTriFractal::new(iteration))));
    tc.run(matches);
}

pub fn dragon_command() -> clap::App<'static, 'static> {
    let tc = TurtleCommand::new("dragon",
                                "Draws a dragon curve fractal",
                                Box::new(|iteration| DragonFractal::new(iteration)));
    tc.command()
}

pub fn dragon_run(matches: &clap::ArgMatches) {
    let tc = TurtleCommand::new("dragon",
                                "Draws a dragon curve fractal",
                                Box::new(|iteration| DragonFractal::new(iteration)));
    tc.run(matches);
}

pub fn kochcurve_command() -> clap::App<'static, 'static> {
    let tc = TurtleCommand::new("kochcurve",
                                "Draws a Koch snowflake curve",
                                Box::new(|iteration| {
                                    LindenmayerSystemTurtleProgram::new(KochCurve::new(iteration))
                                }));
    tc.command()
}

pub fn kochcurve_run(matches: &clap::ArgMatches) {
    let tc = TurtleCommand::new("kochcurve",
                                "Draws a Koch snowflake curve",
                                Box::new(|iteration| {
                                    LindenmayerSystemTurtleProgram::new(KochCurve::new(iteration))
                                }));
    tc.run(matches);
}

pub fn levyccurve_command() -> clap::App<'static, 'static> {
    let tc = TurtleCommand::new("levyccurve",
                                "Draws a Levy C Curve",
                                Box::new(|iteration| {
                                    LindenmayerSystemTurtleProgram::new(LevyCCurve::new(iteration))
                                }));
    tc.command()
}

pub fn levyccurve_run(matches: &clap::ArgMatches) {
    let tc = TurtleCommand::new("levyccurve",
                                "Draws a Levy C Curve",
                                Box::new(|iteration| {
                                    LindenmayerSystemTurtleProgram::new(LevyCCurve::new(iteration))
                                }));
    tc.run(matches);
}

pub fn mandelbrot_command() -> clap::App<'static, 'static> {
    let ctor = Box::new(|max_iterations, power| Mandelbrot::new(max_iterations, power));
    let etc = EscapeTimeCommand::new("mandelbrot", "Draws the mandelbrot fractal", ctor);
    etc.command()
}

pub fn mandelbrot_run(matches: &clap::ArgMatches) {
    let ctor = Box::new(|max_iterations, power| Mandelbrot::new(max_iterations, power));
    let etc = EscapeTimeCommand::new("mandelbrot", "Draws the mandelbrot fractal", ctor);
    etc.run(matches);
}

pub fn roadrunner_command() -> clap::App<'static, 'static> {
    let ctor = Box::new(|max_iterations, power| RoadRunner::new(max_iterations, power));
    let etc = EscapeTimeCommand::new("roadrunner",
                                     "Draws a variation of the burning ship fractal",
                                     ctor);
    etc.command()
}

pub fn roadrunner_run(matches: &clap::ArgMatches) {
    let ctor = Box::new(|max_iterations, power| RoadRunner::new(max_iterations, power));
    let etc = EscapeTimeCommand::new("roadrunner",
                                     "Draws a variation of the burning ship fractal",
                                     ctor);
    etc.run(matches);
}

pub fn sierpinski_command() -> clap::App<'static, 'static> {
    let ctor = Box::new(||SierpinskiChaosGame::new());
    let cgc = ChaosGameCommand::new("sierpinski", "Draws a Sierpinski triangle using a chaos game and 3 randomly chosen points on the screen", ctor);
    cgc.command()
}

pub fn sierpinski_run(matches: &clap::ArgMatches) {
    let ctor = Box::new(||SierpinskiChaosGame::new());
    let cgc = ChaosGameCommand::new("sierpinski", "Draws a Sierpinski triangle using a chaos game and 3 randomly chosen points on the screen", ctor);
    cgc.run(matches);
}

pub fn terdragon_command() -> clap::App<'static, 'static> {
    let tc = TurtleCommand::new("terdragon",
                                "Draws a terdragon curve",
                                Box::new(|iteration| {
                                    LindenmayerSystemTurtleProgram::new(TerdragonFractal::new(iteration))
                                }));
    tc.command()
}

pub fn terdragon_run(matches: &clap::ArgMatches) {
    let tc = TurtleCommand::new("terdragon",
                                "Draws a terdragon curve",
                                Box::new(|iteration| {
                                    LindenmayerSystemTurtleProgram::new(TerdragonFractal::new(iteration))
                                }));
    tc.run(matches);
}
