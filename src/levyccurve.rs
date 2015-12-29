// Copyright (c) 2015 William (B.J.) Snow Orvis
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

//! Computations and abstractions needed for rendering a Lévy C Curve.

use common::{Point, Turtle};
use lindenmayer::{LindenmayerSystem, LindenmayerSystemDrawingParameters};
use std::f64::consts::SQRT_2;

#[derive(Copy, Clone, Debug)]
pub struct LevyCCurve {
    iterations: u64,
}

#[derive(Copy, Clone, Debug)]
pub enum LSA {
    F, // move forward
    L, // turn left 45 degrees
    R, // turn right 45 degrees
}

impl LevyCCurve {
    pub fn new(iterations: u64) -> Result<LevyCCurve, &'static str> {
        let lcc = LevyCCurve { iterations: iterations };
        Ok(lcc)
    }

    fn lines_between_endpoints(self) -> f64 {
        match self.iterations {
            0 => 1.0,
            x => SQRT_2.powf(x as f64),
        }
    }

    fn distance_forward(self) -> f64 {
        1.0 / (self.lines_between_endpoints() as f64) / 2.0
    }
}

impl LindenmayerSystem<LSA> for LevyCCurve {
    fn initial() -> Vec<LSA> {
        vec![LSA::F]
    }

    fn apply_rule(lstr: LSA) -> Vec<LSA> {
        match lstr {
            LSA::F => vec![LSA::L, LSA::F, LSA::R, LSA::R, LSA::F, LSA::L],
            x => vec![x],
        }
    }
}

impl LindenmayerSystemDrawingParameters<LSA> for LevyCCurve {
    fn iteration(&self) -> u64 {
        self.iterations
    }

    fn initialize_turtle(&self, turtle: &mut Turtle) {
        // use std::f64::consts::PI;
        turtle.set_pos(Point { x: 0.0, y: 0.0 });
        // turtle.set_rad(PI / 4.0 * -(self.iterations as f64));
        turtle.set_rad(0.0);
    }

    fn interpret_symbol(&self, symbol: LSA, turtle: &mut Turtle) {
        match symbol {
            LSA::F => turtle.forward(self.distance_forward()),
            LSA::L => turtle.turn_deg(45.0),
            LSA::R => turtle.turn_deg(-45.0),
        }
    }
}

#[cfg(test)]
mod test {
}
