use common::{Point, Turtle, TurtleApp};
use std::f64::consts::SQRT_2;

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
    /// Create a new DragonFractal. `iterations` is the number of times you would "fold" the curve
    /// in half. Eg:
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
    pub fn new(iterations: u64) -> Result<DragonFractal, &'static str> {
        let df = DragonFractal{ iterations: iterations };
        Ok(df)
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
            step_without_twos = step_without_twos / 2;
        }
        if step_without_twos % 4 == 1 {
            return Turn::Left;
        } else {
            return Turn::Right;
        }
    }

    /// How many line segments are between the starting and end points.
    ///
    /// This can be used to calculate how long each line segment should be to ensure that the
    /// drawing of the fractal ends at the desired endpoints. For example, if the starting point is
    /// (0, 0) and the endpoint should be (1, 0), then the size of each line segment would be:
    ///
    ///     (x_start - x_end).abs() / df.lines_between_endpoints()
    ///
    pub fn lines_between_endpoints(self) -> f64 {
        match self.iterations {
            0 => 1.0,
            _ => SQRT_2.powf(self.iterations as f64)
        }
    }
}

impl TurtleApp for DragonFractal {
    /// Draw each of the lines that make up the iterations of the Dragon fractal.
    ///
    /// Starts at (0.0, 0.0) and facing 0 degrees along the X axis. Tries to end at (1.0, 0.0).
    fn draw(&self, turtle: &mut Turtle) {
        use std::f64::consts::PI;

        turtle.set_pos(Point{x:0.0,y:0.0});
        turtle.set_rad(PI/4.0 * -(self.iterations as f64));
        turtle.down();

        // move forward, then change directions.
        for turn in (1..(self.number_of_steps()+1)).map(Self::turn_after_step) {
            turtle.forward(1.0/(self.lines_between_endpoints() as f64));
            match turn {
                Turn::Left => turtle.turn_deg(90.0),
                Turn::Right => turtle.turn_deg(-90.0)
            }
        }

        turtle.up();
    }
}

mod test {
    #[cfg(test)]
    use std::f64::consts::SQRT_2;

    #[cfg(test)]
    use super::DragonFractal;
    #[cfg(test)]
    use super::Turn::{Left, Right};

    #[test]
    fn test_step_count() {
        assert_eq!(DragonFractal::new(0).unwrap().number_of_steps(), 1);
        assert_eq!(DragonFractal::new(1).unwrap().number_of_steps(), 2);
        assert_eq!(DragonFractal::new(2).unwrap().number_of_steps(), 4);
        assert_eq!(DragonFractal::new(3).unwrap().number_of_steps(), 8);
        assert_eq!(DragonFractal::new(4).unwrap().number_of_steps(), 16);
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
        assert_approx_eq!(DragonFractal::new(0).unwrap().lines_between_endpoints(), 1.0, 0.000001);
        assert_approx_eq!(DragonFractal::new(1).unwrap().lines_between_endpoints(), SQRT_2, 0.000001);
        assert_approx_eq!(DragonFractal::new(2).unwrap().lines_between_endpoints(), 2.0, 0.000001);
        assert_approx_eq!(DragonFractal::new(3).unwrap().lines_between_endpoints(), 2.0*SQRT_2, 0.000001);
        assert_approx_eq!(DragonFractal::new(4).unwrap().lines_between_endpoints(), 4.0, 0.000001);
    }
}
