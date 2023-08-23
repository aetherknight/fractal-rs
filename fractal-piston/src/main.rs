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

pub mod fractaldata;
pub mod pistonrendering;
pub mod work_multiplexer;

fn main() {
    // Command line arguments specification
    let mut app = clap::builder::Command::new("fractal")
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about("Renders fractals in another window.");
    app = fractaldata::add_subcommands(app);

    app = app.arg(
        clap::Arg::new("loglevel")
            .num_args(1)
            .help("Choose log level")
            .long("loglevel")
            .value_name("LEVEL")
            .default_value("INFO"),
    );

    let matches = app.get_matches();
    simple_logger::SimpleLogger::new()
        .with_level(
            matches
                .get_one::<String>("loglevel")
                .map(|s| s.as_str())
                .map(|s| fractaldata::parse_arg::<log::LevelFilter>("loglevel", s))
                .unwrap()
                .unwrap(),
        )
        .with_module_level("gfx_device_gl", log::LevelFilter::Warn)
        .init()
        .unwrap();

    let result = fractaldata::run_subcommand(&matches);

    match result {
        Ok(_) => {}
        Err(e) => {
            use std::io::{stderr, Write};
            writeln!(&mut stderr(), "{}", e).unwrap();
            std::process::exit(1);
        }
    }
}
