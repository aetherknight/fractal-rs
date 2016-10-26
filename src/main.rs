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

extern crate clap;
// extern crate docopt;
extern crate rustc_serialize;

extern crate fractal;

use clap::App;
use clap::Arg;
use clap::SubCommand;
use fractal::fractaldata;
use fractal::pistonrendering;
use std::process;


fn parse_arg<T>(opt_name: &str, opt_val: &str) -> T
    where T: std::str::FromStr,
          <T as std::str::FromStr>::Err: std::fmt::Display
{
    match opt_val.parse::<T>() {
        Err(e) => panic!("Error parsing {}: {}", opt_name, e),
        Ok(v) => v,
    }
}

fn main() {
    let fds = fractaldata::get_all_fractal_data();
    // Command line arguments specification
    //
    // TODO: should drawrate and threads be moved into their respective
    // sub-commands? They aren't
    // really global options, but each one is used by several subcommands.
    let mut app = App::new("fractal")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Renders fractals in another window.")
        .arg(Arg::with_name("drawrate")
            .long("drawrate")
            .value_name("RATE")
            .help("The number of lines or dots of the fractal that should be drawn per frame. \
                   (Default: 1)")
            .takes_value(true))
        .arg(Arg::with_name("threads")
            .long("threads")
            .value_name("NUM")
            .help("The number of of worker threads, for fractals that support it. (Default: 1)")
            .takes_value(true));

    // Add the fractals as sub-commands
    app = fds.iter()
        .map(|fd| {
            let mut sc = SubCommand::with_name(fd.name.as_str()).about(fd.summary.as_str());
            // Add any sub-command specific command line arguments
            for (index, arg) in fd.args.iter().enumerate() {
                sc = sc.arg(Arg::with_name(arg).required(true).index(index as u64 + 1));
            }
            sc
        })
        .fold(app, |app, sc| app.subcommand(sc));

    // Parse the command line
    let matches = app.get_matches();

    // Which fractal renderer to run (or print the usage)
    let curve = matches.subcommand_name()
        .unwrap_or_else(|| {
            println!("{}", matches.usage());
            process::exit(1);
        });

    let drawrate = matches.value_of("drawrate")
        .and_then(|d| Some(parse_arg::<u64>("drawrate", d)))
        .unwrap_or(1);

    let threadcount =
        matches.value_of("threads").and_then(|d| Some(parse_arg::<u32>("threads", d))).unwrap_or(1);

    // TODO: move these arguments into the subcommand handlers
    let iterations: u64 = matches.subcommand_matches(curve)
        .and_then(|m| {
            m.value_of("ITERATION")
                .and_then(|d| Some(parse_arg::<u64>("ITERATION", d)))
                .or_else(|| {
                    m.value_of("MAX_ITERATIONS")
                        .and_then(|d| Some(parse_arg::<u64>("MAX_ITERATIONS", d)))
                })
        })
        .unwrap_or(0);
    let power = matches.subcommand_matches(curve)
        .and_then(|m| m.value_of("POWER").and_then(|d| Some(parse_arg::<u64>("POWER", d))))
        .unwrap_or(0);

    if let Ok(command_data) = fractaldata::get_fractal_data(curve) {
        let callback = command_data.with_window_handler;

        let args = fractaldata::Arguments {
            curve: curve.to_string(),
            drawrate: drawrate,
            threadcount: threadcount as u32,

            iterations: iterations,
            power: power,
        };

        callback(&args,
                 &|handler| {
                     pistonrendering::run(handler);
                 });
    } else {
        panic!("Unknown fractal: {}. Run `fractal --help` for more information.",
               curve);
    }
}
