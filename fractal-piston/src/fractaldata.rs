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

//! Code and data structures that glue together command line arguments to the code that draws them.

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
use fractal_lib::{FractalCategory, SelectedFractal};
use std::str::FromStr;
use std::sync::Arc;
use strum::IntoEnumIterator;

/// Helper to extract and parse a value from a clap command line argument.
macro_rules! extract {
    ($matches:expr, $name:expr) => {
        $matches
            .get_one::<String>($name)
            .map(|s| s.as_str())
            .map(|s| parse_arg($name, s))
            .unwrap_or_else(|| Err(format!("Missing {}", $name)))
    };
}

/// Function to help with parsing a command line argument into some other type.
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

fn run_chaos_game<E, F>(ctor: &F, matches: &clap::ArgMatches) -> Result<(), String>
where
    E: ChaosGameMoveIterator + 'static,
    F: Fn() -> E,
{
    let drawrate = extract!(matches, "drawrate")?;

    let game = Box::new((ctor)());
    let mut handler = pistonrendering::chaosgame::ChaosGameWindowHandler::new(game, drawrate);
    pistonrendering::run(&mut handler);

    Ok(())
}

fn run_escape_time<E, F>(ctor: &F, matches: &clap::ArgMatches) -> Result<(), String>
where
    E: EscapeTime + Send + Sync + 'static,
    F: Fn(u64, u64) -> E,
{
    let max_iterations = (extract!(matches, "MAX_ITERATIONS"))?;
    // .unwrap_or_else(|| return Err("Must specify a MAX_ITERATIONS of 1 or greater!"));
    let power = (extract!(matches, "POWER"))?;
    // .unwrap_or_else(|| return Err("Must specify a POWER of 1 or greater!"));

    let et = Arc::new((ctor)(max_iterations, power));
    // TODO: `et` when passed in here wants E to be constraint by `'static`. Why?
    let mut handler = pistonrendering::escapetime::EscapeTimeWindowHandler::new(et);
    pistonrendering::run(&mut handler);

    Ok(())
}

fn run_turtle<E, F>(ctor: &F, matches: &clap::ArgMatches) -> Result<(), String>
where
    E: TurtleProgram + 'static,
    F: Fn(u64) -> E,
{
    let drawrate = (extract!(matches, "drawrate"))?;
    let iteration = (extract!(matches, "ITERATION"))?;
    // .unwrap_or_else(|| Err("Must specify an ITERATION of 1 or greater!"));

    let program = (ctor)(iteration);
    let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&program, drawrate);
    pistonrendering::run(&mut *handler);

    Ok(())
}

trait SelectedFractalExt {
    fn clap_subcommand<'a>(&self) -> clap::builder::Command<'a>;
    fn run(&self, matches: &clap::ArgMatches) -> Result<(), String>;
}

impl SelectedFractalExt for SelectedFractal {
    /// Constructs a clap subcommand for a given `SelectedFractal` variant.
    ///
    /// It uses the fractal's category to determine which input arguments it supports.
    fn clap_subcommand<'a>(&self) -> clap::builder::Command<'a> {
        let subcommand = clap::Command::new::<&str>(self.into()).about(self.description());
        match self.category() {
            FractalCategory::ChaosGames => subcommand.arg(
                clap::Arg::new("drawrate")
                    .takes_value(true)
                    .help("The number of points to draw per frame")
                    .long("drawrate")
                    .value_name("MPF")
                    .default_value("1"),
            ),
            FractalCategory::EscapeTimeFractals => subcommand
                .arg(
                    clap::Arg::new("MAX_ITERATIONS")
                        .required(true)
                        .index(1)
                        .help(
                            "The maximum number of iterations of the escape time function before \
                         deciding the fractal has escaped",
                        ),
                )
                .arg(
                    clap::Arg::new("POWER")
                        .required(true)
                        .index(2)
                        .help("The exponent used in the escape time function (positive integer)"),
                ),
            FractalCategory::TurtleCurves => subcommand
                .arg(
                    clap::Arg::new("drawrate")
                        .takes_value(true)
                        .help("The number of points to draw per frame")
                        .long("drawrate")
                        .value_name("MPF")
                        .default_value("1"),
                )
                .arg(
                    clap::Arg::new("ITERATION")
                        .required(true)
                        .index(1)
                        .help(
                            "Which iteration of the underlying curve to draw. This usually \
                            causes an exponential growth in required computation",
                        ),
                ),
        }
    }

    /// Parses the clap arguments for the command launches an animated renderer for the specified
    /// fractal variant.
    fn run(&self, matches: &clap::ArgMatches) -> Result<(), String> {
        match self {
            SelectedFractal::BarnsleyFern => run_chaos_game(
                &|| {
                    barnsleyfern::BarnsleyFern::new(
                        &barnsleyfern::REFERENCE_TRANSFORMS,
                        &barnsleyfern::REFERENCE_WEIGHTS,
                    )
                },
                matches,
            ),
            SelectedFractal::BurningMandel => run_escape_time(&BurningMandel::new, matches),
            SelectedFractal::BurningShip => run_escape_time(&BurningShip::new, matches),
            SelectedFractal::Cesaro => run_turtle(
                &LindenmayerSystemTurtleProgram::build(CesaroFractal::new),
                matches,
            ),
            SelectedFractal::CesaroTri => run_turtle(
                &LindenmayerSystemTurtleProgram::build(CesaroTriFractal::new),
                matches,
            ),
            SelectedFractal::Dragon => run_turtle(&DragonFractal::new, matches),
            SelectedFractal::KochCurve => run_turtle(
                &|iteration| LindenmayerSystemTurtleProgram::new(KochCurve::new(iteration)),
                matches,
            ),
            SelectedFractal::LevyCCurve => run_turtle(
                &LindenmayerSystemTurtleProgram::build(LevyCCurve::new),
                matches,
            ),
            SelectedFractal::Mandelbrot => run_escape_time(&Mandelbrot::new, matches),
            SelectedFractal::RoadRunner => run_escape_time(&RoadRunner::new, matches),
            SelectedFractal::Sierpinski => run_chaos_game(&SierpinskiChaosGame::new, matches),
            SelectedFractal::TerDragon => run_turtle(
                &LindenmayerSystemTurtleProgram::build(TerdragonFractal::new),
                matches,
            ),
        }
    }
}

pub fn add_subcommands<'a>(app: clap::builder::Command<'a>) -> clap::builder::Command<'a> {
    let mut app = app;
    for fractal in SelectedFractal::iter() {
        app = app.subcommand(fractal.clap_subcommand());
    }
    app
}

pub fn run_subcommand(app_argmatches: &clap::ArgMatches) -> Result<(), String> {
    if let Some((name, args)) = app_argmatches.subcommand() {
        if let Ok(fractal) = SelectedFractal::from_str(name) {
            fractal.run(&args)
        } else {
            Err("Unknown subcommand".to_string())
        }
    } else {
        Err("Unknown subcommand".to_string())
    }
}
