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

//! Computations and abstractions needed for rendering a dragon fractal.

use std::f64::consts::PI;
use std::f64::consts::SQRT_2;

use crate::geometry::Point;
use crate::turtle::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Copy, Clone, Debug)]
pub struct DragonFractal {
    iterations: u64,
}

/// Represents the computations needed to render a dragon fractal of a particular iteration.
impl DragonFractal {
    /// Create a new DragonFractal. `iterations` is the number of times you would "fold" the
    /// curve in half. Eg:
    ///
    /// ```text
    /// 0:
    ///     --------
    ///
    /// 1:
    ///     \      /
    ///      \    /
    ///       \  /
    ///        \/
    ///
    /// 2:
    ///         ___
    ///     |  |
    ///     |__|
    /// ```
    pub fn new(iterations: u64) -> DragonFractal {
        DragonFractal { iterations }
    }

    /// The number of lines that will be drawn.
    ///
    /// Essentially: 2^(iterations)
    pub fn number_of_steps(self) -> u64 {
        match self.iterations {
            0 => 1,
            1 => 2,
            2 => 4,
            _ => 2 << (self.iterations - 1),
        }
    }

    /// Whether to turn left or right after a move forward.
    pub fn turn_after_step(step: u64) -> Turn {
        let mut step_without_twos = step;
        while step_without_twos != 0 && (step_without_twos % 2) == 0 {
            step_without_twos /= 2;
        }
        if step_without_twos % 4 == 1 {
            Turn::Left
        } else {
            Turn::Right
        }
    }

    /// How many line segments are between the starting and end points.
    ///
    /// This can be used to calculate how long each line segment should be to ensure that the
    /// drawing of the fractal ends at the desired endpoints. For example, if the starting
    /// point is (0, 0) and the endpoint should be (1, 0), then the size of each line segment
    /// would be:
    ///
    /// ```text
    /// (x_start - x_end).abs() / df.lines_between_endpoints()
    /// ```
    pub fn lines_between_endpoints(self) -> f64 {
        match self.iterations {
            0 => 1.0,
            _ => SQRT_2.powf(self.iterations as f64),
        }
    }
}

impl TurtleProgram for DragonFractal {
    /// Starts at (0.0, 0.0) and facing 0 degrees along the X axis. Tries to end at (1.0, 0.0).
    fn init_turtle(&self) -> Vec<TurtleStep> {
        vec![
            TurtleStep::SetPos(Point { x: 0.0, y: 0.0 }),
            TurtleStep::SetRad(PI / 4.0 * -(self.iterations as f64)),
            TurtleStep::Down,
        ]
    }

    fn turtle_program_iter(&self) -> TurtleProgramIterator {
        TurtleProgramIterator::new(Box::new(DragonFractalTurtleProgramIterator {
            dragon: *self,
            curr_step: 1,
            move_next: true,
        }))
    }
}

/// Iterator that emits the dragon fractal's turtle program as `TurtleStep`s.
pub struct DragonFractalTurtleProgramIterator {
    dragon: DragonFractal,
    curr_step: u64,
    move_next: bool,
}

impl Iterator for DragonFractalTurtleProgramIterator {
    type Item = TurtleStep;

    fn next(&mut self) -> Option<TurtleStep> {
        if self.curr_step > self.dragon.number_of_steps() {
            return None;
        }
        if self.move_next {
            self.move_next = false;
            log::debug!("curr_step: {}, Forward", self.curr_step);
            Some(TurtleStep::Forward(
                1.0 / (self.dragon.lines_between_endpoints() as f64),
            ))
        } else {
            let turn = DragonFractal::turn_after_step(self.curr_step);
            log::debug!("curr_step :{}, {:?}", self.curr_step, turn);
            self.move_next = true;
            self.curr_step += 1;
            match turn {
                Turn::Left => Some(TurtleStep::TurnRad(PI / 2.0_f64)),
                Turn::Right => Some(TurtleStep::TurnRad(-PI / 2.0_f64)),
            }
        }
    }
}

#[cfg(test)]
mod test {

    use super::DragonFractal;
    use super::Turn::{Left, Right};
    use crate::geometry::Point;
    use crate::turtle::{TurtleProgram, TurtleStep};
    use std::f64::consts::PI;
    use std::f64::consts::SQRT_2;

    #[test]
    fn test_step_count() {
        assert_eq!(DragonFractal::new(0).number_of_steps(), 1);
        assert_eq!(DragonFractal::new(1).number_of_steps(), 2);
        assert_eq!(DragonFractal::new(2).number_of_steps(), 4);
        assert_eq!(DragonFractal::new(3).number_of_steps(), 8);
        assert_eq!(DragonFractal::new(4).number_of_steps(), 16);
    }

    #[test]
    fn test_step_turn() {
        assert_eq!(DragonFractal::turn_after_step(1), Left);
        assert_eq!(DragonFractal::turn_after_step(2), Left);
        assert_eq!(DragonFractal::turn_after_step(3), Right);
        assert_eq!(DragonFractal::turn_after_step(4), Left);
        assert_eq!(DragonFractal::turn_after_step(5), Left);
        assert_eq!(DragonFractal::turn_after_step(6), Right);
        assert_eq!(DragonFractal::turn_after_step(7), Right);
        assert_eq!(DragonFractal::turn_after_step(8), Left);
        assert_eq!(DragonFractal::turn_after_step(9), Left);
        assert_eq!(DragonFractal::turn_after_step(10), Left);
        assert_eq!(DragonFractal::turn_after_step(11), Right);
        assert_eq!(DragonFractal::turn_after_step(12), Right);
        assert_eq!(DragonFractal::turn_after_step(13), Left);
        assert_eq!(DragonFractal::turn_after_step(14), Right);
        assert_eq!(DragonFractal::turn_after_step(15), Right);
    }

    #[test]
    fn test_lines_between_endpoints() {
        assert_approx_eq!(
            DragonFractal::new(0).lines_between_endpoints(),
            1.0,
            0.000001
        );
        assert_approx_eq!(
            DragonFractal::new(1).lines_between_endpoints(),
            SQRT_2,
            0.000001
        );
        assert_approx_eq!(
            DragonFractal::new(2).lines_between_endpoints(),
            2.0,
            0.000001
        );
        assert_approx_eq!(
            DragonFractal::new(3).lines_between_endpoints(),
            2.0 * SQRT_2,
            0.000001
        );
        assert_approx_eq!(
            DragonFractal::new(4).lines_between_endpoints(),
            4.0,
            0.000001
        );
    }

    #[test]
    fn test_init_turtle() {
        fn check_init_turtle(iteration: u64, expected_angle: f64) {
            let initial_steps = DragonFractal::new(iteration).init_turtle();
            assert_eq!(initial_steps.len(), 3);
            match initial_steps.get(0) {
                Some(&TurtleStep::SetPos(point)) => {
                    assert_point_eq!(point, Point { x: 0.0, y: 0.0 }, 0.000000001)
                }
                _ => panic!("Iteration {} did not return a SetPos first", iteration),
            }
            match initial_steps.get(1) {
                Some(&TurtleStep::SetRad(angle)) => {
                    assert_approx_eq!(angle, expected_angle, 0.000000001)
                }
                _ => panic!("Iteration {} did not return not a SetRad second", iteration),
            }
            match initial_steps.get(2) {
                Some(&TurtleStep::Down) => {}
                _ => panic!("Iteration {} did not return a Down third", iteration),
            }
        }
        check_init_turtle(0, 0.0);
        check_init_turtle(1, -PI / 4.0);
        check_init_turtle(2, -PI / 2.0);
        check_init_turtle(3, -3.0 * PI / 4.0);
        check_init_turtle(4, -PI);
    }
}
