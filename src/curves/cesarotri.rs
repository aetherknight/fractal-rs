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

use geometry::deg2rad;
use lindenmayer::{LindenmayerSystem, LindenmayerSystemDrawingParameters};
use turtle::TurtleStep;

#[derive(Copy, Clone, Debug)]
pub struct CesaroTriFractal {
    iterations: u64,
}

#[derive(Copy, Clone, Debug)]
pub enum LSA {
    F1, // side 1
    F2, // side 2
    F3, // side 3
    Q1, // corner 1
    Q2, // corner 2
    Q3, // corner 3
    L, // turn left X degrees
    R, // turn right X degrees
}

impl CesaroTriFractal {
    pub fn new(iterations: u64) -> CesaroTriFractal {
        CesaroTriFractal { iterations: iterations }
    }
}

impl LindenmayerSystem<LSA> for CesaroTriFractal {
    fn initial(&self) -> Vec<LSA> {
        vec![LSA::F1, LSA::Q1, LSA::F2, LSA::Q2, LSA::F3, LSA::Q3]
    }

    fn apply_rule(&self, lstr: LSA) -> Vec<LSA> {
        match lstr {
            LSA::F1 => vec![LSA::F1, LSA::L, LSA::F1, LSA::R, LSA::R, LSA::F1, LSA::L, LSA::F1],
            LSA::F2 => vec![LSA::F2, LSA::L, LSA::F2, LSA::R, LSA::R, LSA::F2, LSA::L, LSA::F2],
            LSA::F3 => vec![LSA::F3, LSA::L, LSA::F3, LSA::R, LSA::R, LSA::F3, LSA::L, LSA::F3],
            x => vec![x],
        }
    }
}

impl LindenmayerSystemDrawingParameters<LSA> for CesaroTriFractal {
    fn iteration(&self) -> u64 {
        self.iterations
    }

    // It is cleaner to read the Lyndenmayer System Alphabet matcher arms
    // separately.
    #[cfg_attr(feature = "cargo-clippy", allow(match_same_arms))]
    fn interpret_symbol(&self, symbol: LSA) -> TurtleStep {
        // use std::f64::consts::SQRT_2;
        use std::f64::consts::PI;

        let base_angle = 85.0_f64;
        let base_angle_rads = base_angle / 180.0_f64 * PI;

        let hyp = 1.0_f64; // length of the whole hypotenuse side
        // the length of a line segment on the hypotenuse.
        //
        // For iteration 0, this is the hypotenuse.
        // For iteration 1, this is H / (2*(1 + sin(base_angle/2)))
        // For iteration 2,         H / (2*(1 + sin(base_angle/2)))^2
        // and so forth.
        let hyp_unit = hyp /
                       ((2.0_f64 * (1.0_f64 + (PI - (2.0_f64 * base_angle_rads)).sin()))
            .powf(self.iterations as f64));

        // cos(a) = A/H. A = hyp/2. H = A/cos(a)
        // let side_len = (hyp / 2.0_f64) / (base_angle_rads / 2.0_f64).cos();
        let side_unit = hyp_unit / 2.0_f64 / (base_angle_rads / 2.0_f64).cos();

        let side_angle = base_angle / 2.0_f64;
        let top_angle = 180.0_f64 - (2.0_f64 * side_angle);

        match symbol {
            LSA::F1 => TurtleStep::Forward(hyp_unit),
            LSA::F2 => TurtleStep::Forward(side_unit),
            LSA::F3 => TurtleStep::Forward(side_unit),
            LSA::Q1 => TurtleStep::TurnRad(deg2rad(180.0_f64 - side_angle)),
            LSA::Q2 => TurtleStep::TurnRad(deg2rad(180.0_f64 - top_angle)),
            LSA::Q3 => TurtleStep::TurnRad(deg2rad(180.0_f64 - side_angle)),
            LSA::L => TurtleStep::TurnRad(deg2rad(base_angle)),
            LSA::R => TurtleStep::TurnRad(deg2rad(-base_angle)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::CesaroTriFractal;
    use geometry::Point;
    use lindenmayer::LindenmayerSystemDrawingParameters;

    #[test]
    fn test_initial_pos() {
        assert_point_eq!(CesaroTriFractal::new(0).initial_pos(),
                         Point { x: 0.0, y: 0.0 },
                         0.000000001);
        assert_point_eq!(CesaroTriFractal::new(1).initial_pos(),
                         Point { x: 0.0, y: 0.0 },
                         0.000000001);
        assert_point_eq!(CesaroTriFractal::new(2).initial_pos(),
                         Point { x: 0.0, y: 0.0 },
                         0.000000001);
    }

    #[test]
    fn test_initial_angle() {
        assert_eq!(CesaroTriFractal::new(0).initial_rad(), 0.0);
        assert_eq!(CesaroTriFractal::new(1).initial_rad(), 0.0);
        assert_eq!(CesaroTriFractal::new(2).initial_rad(), 0.0);
    }
}
