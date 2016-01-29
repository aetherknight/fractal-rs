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

pub mod cesaro;
pub mod cesarotri;
pub mod dragon;
pub mod kochcurve;
pub mod levyccurve;
pub mod terdragon;

use std::error::Error;
use std::fmt;

use self::cesaro::CesaroFractal;
use self::cesarotri::CesaroTriFractal;
use self::dragon::DragonFractal;
use self::kochcurve::KochCurve;
use self::levyccurve::LevyCCurve;
use self::terdragon::TerdragonFractal;
use super::lindenmayer::LindenmayerSystemTurtleProgram;
use super::turtle::TurtleProgram;

#[derive(Debug)]
pub struct CouldNotfindTurtleProgramError {
    requested_program_name: String,
}

impl fmt::Display for CouldNotfindTurtleProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not find the specified TurtleProgram: {}", self.requested_program_name)
    }
}

impl Error for CouldNotfindTurtleProgramError {
    fn description(&self) -> &str {
        "Could not find the specified TurtleProgram"
    }
}

/// Looks up and instantiates a TurtleProgram that will draw a given curve.
pub fn lookup_turtle_program(program_name: &str, iterations: u64) -> Result<Box<TurtleProgram>,CouldNotfindTurtleProgramError>{
    match program_name {
        "cesaro" =>     Ok(Box::new(LindenmayerSystemTurtleProgram::new(CesaroFractal::new(iterations)))),
        "cesarotri" =>  Ok(Box::new(LindenmayerSystemTurtleProgram::new(CesaroTriFractal::new(iterations)))),
        "dragon" =>     Ok(Box::new(DragonFractal::new(iterations))),
        "kochcurve" =>  Ok(Box::new(LindenmayerSystemTurtleProgram::new(KochCurve::new(iterations)))),
        "levyccurve" => Ok(Box::new(LindenmayerSystemTurtleProgram::new(LevyCCurve::new(iterations)))),
        "terdragon" =>  Ok(Box::new(LindenmayerSystemTurtleProgram::new(TerdragonFractal::new(iterations)))),
        _ =>            Err(CouldNotfindTurtleProgramError { requested_program_name: program_name.to_string() }),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lookup_turtle_program() {
        let maybe_cesaro = lookup_turtle_program("cesaro", 0);
        assert!(maybe_cesaro.is_ok());

        let bogus = lookup_turtle_program("foobar", 0);
        assert!(bogus.is_err());
    }
}
