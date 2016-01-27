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

extern crate argparse;
extern crate graphics;
extern crate piston;
extern crate piston_window;

extern crate fractal;
mod glwindow;

use argparse::{ArgumentParser, Store, StoreTrue};

use fractal::curves::cesaro::CesaroFractal;
use fractal::curves::cesarotri::CesaroTriFractal;
use fractal::curves::dragon::DragonFractal;
use fractal::curves::kochcurve::KochCurve;
use fractal::curves::levyccurve::LevyCCurve;
use fractal::curves::terdragon::TerdragonFractal;
use fractal::lindenmayer::LindenmayerSystemTurtleProgram;
use fractal::turtle::TurtleProgram;

struct Arguments {
    curve_name: String,
    iterations: u64,
    animate: u64,
    version: bool,
}

fn parse_args() -> Arguments {
    let mut retargs = Arguments { curve_name: String::from(""), iterations: 0, animate: 0, version: false };
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Renders fractal curves.");
        parser.refer(&mut retargs.animate).add_option(&["--animate"], Store, "Animate the drawing of the fractal instead of drawing it all at once. ANIMATE specifies the number of moves to make per frame of animation. Set to 0 to explicitly disable.");
        parser.refer(&mut retargs.version).add_option(&["-v", "--version"], StoreTrue, "Display the version");

        parser.refer(&mut retargs.curve_name).add_argument("curve", Store, "Which curve to draw. Valid options are: cesaro, cesarotri, dragon, kochcurve, levyccurve, and terdragon.");
        parser.refer(&mut retargs.iterations).add_argument("iterations", Store, "The iteration of the specified curve to draw. should be a non-negative integer.");
        parser.parse_args_or_exit();
    }

    retargs
}

fn construct_program(program_name: &str, iterations: u64) -> Box<TurtleProgram> {
    match program_name {
        "cesaro"     => Box::new(LindenmayerSystemTurtleProgram::new(CesaroFractal::new(iterations))),
        "cesarotri"  => Box::new(LindenmayerSystemTurtleProgram::new(CesaroTriFractal::new(iterations))),
        "dragon"     => Box::new(DragonFractal::new(iterations)),
        "kochcurve"  => Box::new(LindenmayerSystemTurtleProgram::new(KochCurve::new(iterations))),
        "levyccurve" => Box::new(LindenmayerSystemTurtleProgram::new(LevyCCurve::new(iterations))),
        "terdragon"  => Box::new(LindenmayerSystemTurtleProgram::new(TerdragonFractal::new(iterations))),
        _            => panic!("Unknown program type")
    }
}

fn main() {
    let args = parse_args();

    let program = construct_program(args.curve_name.as_ref(), args.iterations);

    glwindow::run(&*program, args.animate);
}
