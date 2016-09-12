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
extern crate rustc_serialize;

extern crate fractal;

use docopt::Docopt;
use fractal::fractaldata;
use fractal::pistonrendering;
use std::env;

#[cfg_attr(rustfmt, rustfmt_skip)]
const USAGE: &'static str = "
Renders fractals in a piston window.

Usage:
  fractal (-h|--help|--version)
  fractal [options] CURVE [ITERATION] [POWER]

Arguments:
  CURVE         Which curve to draw.
  ITERATION     Parameter needed by some fractals. (sometimes called MAX_ITERATIONS)
  POWER         Optional exponent used by some curves. (default: 2)

Options:
  -h --help       Show this screen.
  --version       Show version.
  --drawrate=<r>  The number of lines or dots of the fractal that should be
                  drawn per frame. [default: 1]
  --threads=<t>   The number of worker threads, for modes that support it.
                  [default: 1]

Fractals:
";

#[derive(Debug, RustcDecodable)]
#[allow(non_snake_case)]
struct Args {
    flag_version: bool,
    flag_drawrate: u64,
    flag_threads: u32,
    arg_CURVE: String,
    arg_ITERATION: Option<u64>,
    arg_POWER: Option<u64>,
}

impl Into<fractaldata::Arguments> for Args {
    fn into(self) -> fractaldata::Arguments {
        let iterations = self.arg_ITERATION.unwrap_or(0);
        let power = self.arg_POWER.unwrap_or(2);
        fractaldata::Arguments {
            curve: self.arg_CURVE,
            drawrate: self.flag_drawrate,
            iterations: iterations,
            power: power,
            threadcount: self.flag_threads,
        }
    }
}

fn parse_args() -> Args {
    // Construct a summary of all the supported fractals to append to the basic
    // program usage.
    let all_fds = fractaldata::get_all_fractal_data();
    let longest_usage: usize = all_fds.iter().map(|fd| fd.usage().len()).max().unwrap();
    let help_summaries: Vec<String> = all_fds.iter()
        .map(|fd| {
            // construct a single line of the help summary. It involves the fractal
            // command, its args, some whitespace to pad, and then a short description. The
            // padding ensures that the short descriptions all line up.
            let fd_usage = fd.usage();
            let mut components = vec!["  ", &fd_usage];
            let spaces = longest_usage - fd_usage.len();
            for _ in 0..spaces {
                components.push(" ");
            }
            components.push("  ");
            components.push(&fd.summary);
            components.concat()
        })
        .collect();
    let help_summary = help_summaries.join("\n");

    Docopt::new([USAGE, &help_summary].concat())
        .and_then(|d| d.argv(env::args()).decode())
        .unwrap_or_else(|e| e.exit())
}

fn main() {
    let args: Args = parse_args();

    if args.flag_version {
        println!("{}", env!("CARGO_PKG_VERSION").to_string());
        std::process::exit(0);
    }

    if let Ok(command_data) = fractaldata::get_fractal_data(args.arg_CURVE.as_ref()) {
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
