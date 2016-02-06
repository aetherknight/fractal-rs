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

extern crate docopt;
extern crate graphics;
extern crate piston;
extern crate piston_window;
extern crate rustc_serialize;

extern crate fractal;

use docopt::Docopt;
use std::env;

use fractal::pistonrendering;
use fractal::fractaldata;

#[cfg_attr(rustfmt, rustfmt_skip)]
const USAGE: &'static str = "
Renders fractals in a piston window.

Usage:
  fractal (-h|--help|--version)
  fractal [options] CURVE [ITERATION]

Arguments:
  CURVE         Which curve to draw.
  ITERATION     Parameter needed by some curves.

Options:
  -h --help      Show this screen.
  --version      Show version.
  --animate=<A>  Animate drawing some curves by drawing A lines or dots per
                 frame of animation. [default: 1]

Curves:
  barnsleyfern          Barnsley Fern.
  cesaro ITERATION      Césaro square fractal.
  cesarotri ITERATION   Césaro triangle fractal.
  dragon ITERATION      Dragon curve fractal.
  kochcurve ITERATION   Koch snowflake fractal.
  levyccurve ITERATION  Levy C Curve.
  sierpinski            Sierpinski triangle
  terdragon ITERATION   Terdragon fractal.
";

#[derive(Debug, RustcDecodable)]
#[allow(non_snake_case)]
struct Args {
    flag_version: bool,
    flag_animate: u64,
    arg_ITERATION: Option<u64>,
    arg_CURVE: String,
}

impl Into<fractaldata::Arguments> for Args {
    fn into(self) -> fractaldata::Arguments {
        let iterations = self.arg_ITERATION.unwrap_or(0);
        fractaldata::Arguments {
            curve: self.arg_CURVE,
            iterations: iterations,
            animate: self.flag_animate,
        }
    }
}

fn parse_args() -> Args {
    Docopt::new(USAGE)
        .and_then(|d| d.argv(env::args()).decode())
        .unwrap_or_else(|e| e.exit())
}

fn main() {
    let args: Args = parse_args();

    if args.flag_version {
        println!("{}", env!("CARGO_PKG_VERSION").to_string());
        std::process::exit(0);
    }

    if let Some(command_data) = fractaldata::get_chaos_data().get(args.arg_CURVE.as_ref() as &str) {
        let callback = command_data.with_window_handler;
        callback(&args.into(),
                 &|handler| {
                     pistonrendering::run(handler);
                 });
    } else {
        panic!("Unknown fractal: {}. Run `fractal --help` for more information.",
               args.arg_CURVE);
    }
}
