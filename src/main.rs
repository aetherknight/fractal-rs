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
extern crate rustc_serialize;

extern crate fractal;

use fractal::fractaldata;


fn main() {
    // Command line arguments specification
    let app = clap::App::new("fractal")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Renders fractals in another window.")
        .subcommand(fractaldata::barnsleyfern_command())
        .subcommand(fractaldata::burningship_command())
        .subcommand(fractaldata::burningmandel_command())
        .subcommand(fractaldata::cesaro_command())
        .subcommand(fractaldata::cesarotri_command())
        .subcommand(fractaldata::dragon_command())
        .subcommand(fractaldata::kochcurve_command())
        .subcommand(fractaldata::levyccurve_command())
        .subcommand(fractaldata::mandelbrot_command())
        .subcommand(fractaldata::roadrunner_command())
        .subcommand(fractaldata::sierpinski_command())
        .subcommand(fractaldata::terdragon_command());

    let matches = app.get_matches();
    match matches.subcommand() {
        ("barnsleyfern", Some(args)) => fractaldata::barnsleyfern_run(args),
        ("burningship", Some(args)) => fractaldata::burningship_run(args),
        ("burningmandel", Some(args)) => fractaldata::burningmandel_run(args),
        ("cesaro", Some(args)) => fractaldata::cesaro_run(args),
        ("cesarotri", Some(args)) => fractaldata::cesarotri_run(args),
        ("dragon", Some(args)) => fractaldata::dragon_run(args),
        ("kochcurve", Some(args)) => fractaldata::kochcurve_run(args),
        ("levyccurve", Some(args)) => fractaldata::levyccurve_run(args),
        ("mandelbrot", Some(args)) => fractaldata::mandelbrot_run(args),
        ("roadrunner", Some(args)) => fractaldata::roadrunner_run(args),
        ("sierpinski", Some(args)) => fractaldata::sierpinski_run(args),
        ("terdragon", Some(args)) => fractaldata::terdragon_run(args),
        _ => panic!("Unknown subcommand. Run `fractal --help` for more information."),
    }
}
