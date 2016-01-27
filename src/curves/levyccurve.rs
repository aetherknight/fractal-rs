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

//! Computations and abstractions needed for rendering a Lévy C Curve.

use std::f64::consts::SQRT_2;

use geometry::deg2rad;
use lindenmayer::{LindenmayerSystem, LindenmayerSystemDrawingParameters};
use turtle::TurtleStep;

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
    pub fn new(iterations: u64) -> LevyCCurve {
        LevyCCurve { iterations: iterations }
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
    fn initial(&self) -> Vec<LSA> {
        vec![LSA::F]
    }

    fn apply_rule(&self, lstr: LSA) -> Vec<LSA> {
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

    fn interpret_symbol(&self, symbol: LSA) -> TurtleStep {
        match symbol {
            LSA::F => TurtleStep::Forward(self.distance_forward()),
            LSA::L => TurtleStep::TurnRad(deg2rad(45.0)),
            LSA::R => TurtleStep::TurnRad(deg2rad(-45.0)),
        }
    }
}

#[cfg(test)]
mod test {
    use geometry::Point;
    use lindenmayer::LindenmayerSystemDrawingParameters;
    use super::LevyCCurve;

    #[test]
    fn test_initial_pos() {
        assert_point_eq!(LevyCCurve::new(0).initial_pos(),
                         Point { x: 0.0, y: 0.0 },
                         0.000000001);
        assert_point_eq!(LevyCCurve::new(1).initial_pos(),
                         Point { x: 0.0, y: 0.0 },
                         0.000000001);
        assert_point_eq!(LevyCCurve::new(2).initial_pos(),
                         Point { x: 0.0, y: 0.0 },
                         0.000000001);
    }

    #[test]
    fn test_initial_angle() {
        assert_eq!(LevyCCurve::new(0).initial_rad(), 0.0);
        assert_eq!(LevyCCurve::new(1).initial_rad(), 0.0);
        assert_eq!(LevyCCurve::new(2).initial_rad(), 0.0);
    }
}
