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

//! Computations and abstractions needed for rendering a Koch Curve.

use common::{Point, Turtle, deg2rad, TurtleStep};
use lindenmayer::{LindenmayerSystem, LindenmayerSystemDrawingParameters};

#[derive(Copy, Clone, Debug)]
pub struct KochCurve {
    iterations: u64,
}

#[derive(Copy, Clone, Debug)]
pub enum LSA {
    F, // move forward
    L, // turn left X degrees
    R, // turn right X degrees
}

impl KochCurve {
    pub fn new(iterations: u64) -> Result<KochCurve, &'static str> {
        let lcc = KochCurve { iterations: iterations };
        Ok(lcc)
    }

    /// 0 -> 1
    /// 1 -> 1/3
    /// 2 -> 1/9
    fn distance_forward(self) -> f64 {
        1.0 / 2.0 / (3.0 as f64).powf((self.iterations) as f64)
    }
}

impl LindenmayerSystem<LSA> for KochCurve {
    fn initial() -> Vec<LSA> {
        vec![LSA::F, LSA::L, LSA::L, LSA::F, LSA::L, LSA::L, LSA::F]
    }

    fn apply_rule(lstr: LSA) -> Vec<LSA> {
        match lstr {
            LSA::F => vec![LSA::F, LSA::R, LSA::F, LSA::L, LSA::L, LSA::F, LSA::R, LSA::F],
            x => vec![x],
        }
    }
}

impl LindenmayerSystemDrawingParameters<LSA> for KochCurve {
    fn iteration(&self) -> u64 {
        self.iterations
    }

    fn initialize_turtle(&self, turtle: &mut Turtle) {
        turtle.set_pos(Point { x: 0.0, y: 0.0 });
        turtle.set_rad(0.0);
    }

    fn interpret_symbol(&self, symbol: LSA) -> TurtleStep {
        match symbol {
            LSA::F => TurtleStep::Forward(self.distance_forward()),
            LSA::L => TurtleStep::TurnRad(deg2rad(60.0)),
            LSA::R => TurtleStep::TurnRad(deg2rad(-60.0)),
        }
    }
}

#[cfg(test)]
mod test {
}
