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

//! Computations and abstractions needed for rendering a terdragon fractal.

use crate::geometry::deg2rad;
use crate::lindenmayer::{LindenmayerSystem, LindenmayerSystemDrawingParameters};
use crate::turtle::TurtleStep;

const SQRT_3: f64 = 1.7320508075;

#[derive(Copy, Clone, Debug)]
pub struct TerdragonFractal {
    iterations: u64,
}

/// Represents the computations needed to render a terdragon fractal of a particular iteration. It
/// uses a Lindenmeyer system under the hood
impl TerdragonFractal {
    /// Create a new TerdragonFractal. `iterations` is the number of generations to apply to the
    /// Lindenmeyer system used to generate the fractal's turns.
    pub fn new(iterations: u64) -> TerdragonFractal {
        TerdragonFractal {
            iterations: iterations,
        }
    }

    /// The number of lines that will be drawn.
    ///
    /// Essentially: 3^(iterations)
    pub fn number_of_steps(self) -> u64 {
        (3 as u64).pow((self.iterations) as u32)
    }

    /// How many line segments are between the starting and end points.
    ///
    /// This can be used to calculate how long each line segment should be to ensure that the
    /// drawing of the fractal ends at the desired endpoints. For example, if the starting
    /// point is (0, 0) and the endpoint should be (1, 0), then the size of each line segment
    /// would
    /// be:
    ///
    /// ```text
    /// (x_start - x_end).abs() / df.lines_between_endpoints()
    /// ```
    pub fn lines_between_endpoints(self) -> f64 {
        match self.iterations {
            0 => 1.0,
            _ => SQRT_3.powf(self.iterations as f64),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LSA {
    F,
    L,
    R,
}

impl LindenmayerSystem<LSA> for TerdragonFractal {
    fn initial(&self) -> Vec<LSA> {
        vec![LSA::F]
    }

    fn apply_rule(&self, lstr: LSA) -> Vec<LSA> {
        match lstr {
            LSA::F => vec![LSA::F, LSA::L, LSA::F, LSA::R, LSA::F],
            x => vec![x],
        }
    }
}

impl LindenmayerSystemDrawingParameters<LSA> for TerdragonFractal {
    fn iteration(&self) -> u64 {
        self.iterations
    }

    fn initial_rad(&self) -> f64 {
        use std::f64::consts::PI;
        PI / 6.0 * -(self.iterations as f64)
    }

    fn interpret_symbol(&self, symbol: LSA) -> TurtleStep {
        match symbol {
            LSA::F => TurtleStep::Forward(1.0 / (self.lines_between_endpoints() as f64)),
            LSA::L => TurtleStep::TurnRad(deg2rad(120.0)),
            LSA::R => TurtleStep::TurnRad(deg2rad(-120.0)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::SQRT_3;
    use super::{TerdragonFractal, LSA};
    use crate::geometry::Point;
    use crate::lindenmayer::LindenmayerSystemDrawingParameters;
    use std::f64::consts::PI;

    #[test]
    fn test_step_count() {
        assert_eq!(TerdragonFractal::new(0).number_of_steps(), 1);
        assert_eq!(TerdragonFractal::new(1).number_of_steps(), 3);
        assert_eq!(TerdragonFractal::new(2).number_of_steps(), 9);
        assert_eq!(TerdragonFractal::new(3).number_of_steps(), 27);
    }

    #[test]
    fn test_lines_between_endpoints() {
        assert_approx_eq!(
            TerdragonFractal::new(0).lines_between_endpoints(),
            1.0,
            0.000001
        );
        assert_approx_eq!(
            TerdragonFractal::new(1).lines_between_endpoints(),
            SQRT_3,
            0.000001
        );
        assert_approx_eq!(
            TerdragonFractal::new(2).lines_between_endpoints(),
            3.0,
            0.000001
        );
        assert_approx_eq!(
            TerdragonFractal::new(3).lines_between_endpoints(),
            3.0 * SQRT_3,
            0.000001
        );
        assert_approx_eq!(
            TerdragonFractal::new(4).lines_between_endpoints(),
            9.0,
            0.000001
        );
    }

    #[test]
    fn test_l_system() {
        use super::super::super::lindenmayer::LindenmayerSystem; // needed to pull in generate()
        let l_system = TerdragonFractal::new(0);
        assert_eq!(l_system.generate(0), [LSA::F]);
        assert_eq!(
            l_system.generate(1),
            [LSA::F, LSA::L, LSA::F, LSA::R, LSA::F]
        );
        assert_eq!(
            l_system.generate(2),
            [
                LSA::F,
                LSA::L,
                LSA::F,
                LSA::R,
                LSA::F,
                LSA::L,
                LSA::F,
                LSA::L,
                LSA::F,
                LSA::R,
                LSA::F,
                LSA::R,
                LSA::F,
                LSA::L,
                LSA::F,
                LSA::R,
                LSA::F
            ]
        );
    }

    #[test]
    fn test_initial_pos() {
        assert_point_eq!(
            TerdragonFractal::new(0).initial_pos(),
            Point { x: 0.0, y: 0.0 },
            0.000000001
        );
        assert_point_eq!(
            TerdragonFractal::new(1).initial_pos(),
            Point { x: 0.0, y: 0.0 },
            0.000000001
        );
        assert_point_eq!(
            TerdragonFractal::new(2).initial_pos(),
            Point { x: 0.0, y: 0.0 },
            0.000000001
        );
    }

    #[test]
    fn test_initial_angle() {
        assert_approx_eq!(TerdragonFractal::new(0).initial_rad(), 0.0, 0.00000001);
        assert_approx_eq!(
            TerdragonFractal::new(1).initial_rad(),
            -1.0 * PI / 6.0,
            0.00000001
        );
        assert_approx_eq!(
            TerdragonFractal::new(2).initial_rad(),
            -2.0 * PI / 6.0,
            0.00000001
        );
    }
}
