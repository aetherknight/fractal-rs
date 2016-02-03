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

use argparse::{ArgumentParser, Store, Print};

use fractal::chaosgame;
use fractal::curves;
use fractal::pistonrendering;

struct Arguments {
    curve_name: String,
    iterations: u64,
    animate: u64,
}

fn parse_args() -> Arguments {
    let mut retargs = Arguments {
        curve_name: String::from(""),
        iterations: 0,
        animate: 0,
    };
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Renders fractal curves.");
        parser.refer(&mut retargs.animate)
              .add_option(&["--animate"],
                          Store,
                          "Animate the drawing of the fractal instead of drawing it all at once. \
                           ANIMATE specifies the number of moves to make per frame of animation. \
                           Set to 0 to explicitly disable.");
        parser.add_option(&["-v", "--version"],
                          Print(env!("CARGO_PKG_VERSION").to_string()),
                          "Display the version and exit");

        parser.refer(&mut retargs.curve_name)
              .add_argument("curve",
                            Store,
                            "Which curve to draw. Valid options are: cesaro, cesarotri, dragon, \
                             kochcurve, levyccurve, and terdragon.")
              .required();
        parser.refer(&mut retargs.iterations)
              .add_argument("iterations",
                            Store,
                            "The iteration of the specified curve to draw. should be a \
                             non-negative integer.")
              .required();
        parser.parse_args_or_exit();
    }

    retargs
}

fn main() {
    let args = parse_args();

    if let Ok(chaosgame) = chaosgame::construct_chaos_game(args.curve_name
                                                               .as_ref()) {
        let mut handler =
            Box::new(pistonrendering::chaosgame::ChaosGameWindowHandler::new(chaosgame));
        pistonrendering::run(&mut *handler);
    } else if let Ok(program) = curves::lookup_turtle_program(args.curve_name
                                                           .as_ref(),
                                                       args.iterations) {
        let mut handler = pistonrendering::turtle::construct_turtle_window_handler(&*program,
                                                                                   args.animate);
        pistonrendering::run(&mut *handler);
    } else {
        panic!("Unknown curve/program");
    }
}
