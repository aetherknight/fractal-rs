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

//! Computations and abstractions needed for rendering a Césaro fractal

use geometry::{Point, deg2rad};
use lindenmayer::{LindenmayerSystem, LindenmayerSystemDrawingParameters};
use turtle::TurtleStep;

#[derive(Copy, Clone, Debug)]
pub struct CesaroFractal {
    iterations: u64,
}

#[derive(Copy, Clone, Debug)]
pub enum LSA {
    F, // move forward
    Q, // corner of the square
    L, // turn left X degrees
    R, // turn right X degrees
}

impl CesaroFractal {
    pub fn new(iterations: u64) -> CesaroFractal {
        CesaroFractal { iterations: iterations }
    }

    fn distance_forward(self) -> f64 {
        // TODO: 2.2 just approximates the growth factor as more gaps are added to each side.
        1.0 / (2.2 as f64).powf((self.iterations) as f64)
    }
}

impl LindenmayerSystem<LSA> for CesaroFractal {
    fn initial(&self) -> Vec<LSA> {
        vec![LSA::F, LSA::Q, LSA::F, LSA::Q, LSA::F, LSA::Q, LSA::F, LSA::Q]
    }

    fn apply_rule(&self, lstr: LSA) -> Vec<LSA> {
        match lstr {
            LSA::F => vec![LSA::F, LSA::L, LSA::F, LSA::R, LSA::R, LSA::F, LSA::L, LSA::F],
            x => vec![x],
        }
    }
}

impl LindenmayerSystemDrawingParameters<LSA> for CesaroFractal {
    fn iteration(&self) -> u64 {
        self.iterations
    }

    /// Start at y of -0.5 so that the box is more centered.
    fn initial_pos(&self) -> Point {
        Point { x: 0.0, y: -0.5 }
    }

    fn interpret_symbol(&self, symbol: LSA) -> TurtleStep {
        match symbol {
            LSA::F => TurtleStep::Forward(self.distance_forward()),
            LSA::Q => TurtleStep::TurnRad(deg2rad(90.0)),
            LSA::L => TurtleStep::TurnRad(deg2rad(85.0)),
            LSA::R => TurtleStep::TurnRad(deg2rad(-85.0)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::CesaroFractal;
    use geometry::Point;
    use lindenmayer::LindenmayerSystemDrawingParameters;

    #[test]
    fn test_initial_pos() {
        assert_point_eq!(
            CesaroFractal::new(0).initial_pos(),
            Point { x: 0.0, y: -0.5 },
            0.000000001
        );
        assert_point_eq!(
            CesaroFractal::new(1).initial_pos(),
            Point { x: 0.0, y: -0.5 },
            0.000000001
        );
        assert_point_eq!(
            CesaroFractal::new(2).initial_pos(),
            Point { x: 0.0, y: -0.5 },
            0.000000001
        );
    }

    #[test]
    fn test_initial_angle() {
        assert_eq!(CesaroFractal::new(0).initial_rad(), 0.0);
        assert_eq!(CesaroFractal::new(1).initial_rad(), 0.0);
        assert_eq!(CesaroFractal::new(2).initial_rad(), 0.0);
    }
}
