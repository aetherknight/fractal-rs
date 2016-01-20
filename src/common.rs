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

//! A set of types, traits, and macros that provide things like points in a 2-d
//! coordinate system, the definition of a Turtle for turtle drawings, etc.

use std::f64::consts::PI;
use std::fmt;

/// Represents a point in a 2-D cartesian coordinate system.
#[derive(Copy, Clone, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    /// Computes the distance between two points using the Pythagorean formula.
    pub fn distance_to(self, other: Point) -> f64 {
        let x_delta = (self.x - other.x).abs();
        let y_delta = (self.y - other.y).abs();

        (x_delta * x_delta + y_delta * y_delta).sqrt()
    }

    /// Computes the point found at Vector from self.
    pub fn point_at(self, vector: Vector) -> Point {
        Point {
            x: self.x + vector.delta_x(),
            y: self.y + vector.delta_y(),
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/// Represents a Vector on a 2-D cartesian coordinate system.
#[derive(Copy, Clone, Debug)]
pub struct Vector {
    /// The direction, in radians.
    pub direction: f64,
    /// The magnitude, in spatial units.
    pub magnitude: f64,
}

impl Vector {
    pub fn delta_x(self) -> f64 {
        self.direction.cos() * self.magnitude
    }

    pub fn delta_y(self) -> f64 {
        self.direction.sin() * self.magnitude
    }
}

pub fn deg2rad(degrees: f64) -> f64 {
    degrees / 360.0 * 2.0 * PI
}

/// A Turtle is an abstraction for drawing lines in a space. It has a position and it faces a
/// particular direction. A program usually tells a turtle to move forward based upon its facing,
/// to change direction, and to start or stop drawing.
///
/// The implementation must implement `set_rad()` and `turn_rad()` itself, while it gets
/// `set_deg()` and `turn_deg()` for free.
pub trait Turtle {
    /// How far the turtle should move forward.
    fn forward(&mut self, distance: f64);

    /// Move the turtle to the specified coordinates.
    fn set_pos(&mut self, new_pos: Point);

    /// Set the turtle's direction.
    fn set_deg(&mut self, new_deg: f64) {
        self.set_rad(deg2rad(new_deg));
    }

    fn set_rad(&mut self, new_rad: f64);

    /// Rotate the turtle, in degrees.
    ///
    /// Positive values turn the turtle "left" or counter-clockwise. Negative values turn the
    /// turtle "right" or clockwise.
    fn turn_deg(&mut self, degrees: f64) {
        self.turn_rad(deg2rad(degrees));
    }

    /// Convenience method for rotating the turtle in radians instead of degrees.
    ///
    /// 2*PI radians = 360 degrees.
    fn turn_rad(&mut self, radians: f64);

    /// Touch the turtle's pen to the drawing surface.
    fn down(&mut self);

    /// Lift the turtle's pen off of the drawing surface.
    fn up(&mut self);

    /// Perform the action represented by `step`.
    fn perform(&mut self, step: TurtleStep) {
        match step {
            TurtleStep::Forward(dist) => self.forward(dist),
            TurtleStep::SetPos(point) => self.set_pos(point),
            TurtleStep::SetRad(angle) => self.set_rad(angle),
            TurtleStep::TurnRad(angle) => self.turn_rad(angle),
            TurtleStep::Down => self.down(),
            TurtleStep::Up => self.up(),
        }
    }
}

/// Represents the possible actions that a TurtleProgram can perform.
pub enum TurtleStep {
    /// Make the turtle move forward some distance in the coordinate system.
    Forward(f64),
    /// Move the turtle to the specified Point in the coordinate system.
    SetPos(Point),
    /// Set the turtle's angle. 0 and 2π are facing towards the positive X direction.
    SetRad(f64),
    /// Rotate the turtle the specified amount in radians.
    TurnRad(f64),
    /// Touch the turtle's pen to the drawing surface.
    Down,
    /// Lift the turtle's pen off of the drawing surface.
    Up,
}

/// An object that knows how to draw someting using a Turtle. Turtle programs are broken up into
/// two parts: an initializer method that should place the Turtle into its initial state, and a
/// method that returns a TurtleProgramIterator (which should wrap a Boxed internal iterator
/// implementation) that yields the sequence of steps for the main turtle program.
///
/// This approach adds some extra complexity and scaffolding by requiring an iterator (Rust doesn't
/// provide something equivalent to a generator function or coroutine yet), but it grants the
/// renderer renderer a huge amount of flexibility about how to draw/animate the turtle program.
pub trait TurtleProgram {
    /// This method is executed by various TurtleProgram runners before using the iterator. It
    /// should be used to initialize the turtle to a starting position and orientation.
    fn init_turtle(&self, turtle: &mut Turtle);

    /// Should return an iterator object that yields TurtleSteps representing each command the
    /// turtle will take.
    fn turtle_program_iter<'a>(&'a self) -> TurtleProgramIterator;
}

/// The return type for a TurtleProgram's `turtle_program_iter()`. Since Rust does not yet support
/// abstract return types (eg, a trait return type), the next best thing is a wrapper around a
/// boxed type.
pub struct TurtleProgramIterator<'a> {
    iter: Box<Iterator<Item = TurtleStep> + 'a>,
}

impl<'a> TurtleProgramIterator<'a> {
    pub fn new(iter: Box<Iterator<Item = TurtleStep> + 'a>) -> TurtleProgramIterator {
        TurtleProgramIterator { iter: iter }
    }
}

impl<'a> Iterator for TurtleProgramIterator<'a> {
    type Item = TurtleStep;

    fn next(&mut self) -> Option<TurtleStep> {
        self.iter.next()
    }
}

/// Macro to assert that two floating point values are almost equal.
///
/// This would use float-cmp and use ULPs instead of an epsilon, but float-cmp relies upon a Rust
/// language/stdlib feature that is not yet in the stable release, as of 2015/12/12.
macro_rules! assert_approx_eq {
    ( $lhs:expr, $rhs:expr, $epsilon:expr ) => {
        {
            let lhs = ($lhs as f64).abs();
            let rhs = ($rhs as f64).abs();
            let epsilon = ($epsilon as f64).abs();

            if ! ((lhs - rhs).abs() < epsilon) {
                panic!("assertion failed: {} does not approximately equal: {}", lhs, rhs);
            }
        }
    }
}

/// Macro to assert that two Points are almost equal.
///
/// This would use float-cmp and use ULPs instead of an epsilon, but float-cmp relies upon a Rust
/// language/stdlib feature that is not yet in the stable release, as of 2015/12/12.
macro_rules! assert_point_eq {
    ( $lhs:expr, $rhs:expr, $epsilon:expr ) => {
        {
            let lhs = $lhs as Point;
            let rhs = $rhs as Point;
            let epsilon = $epsilon as f64;

            if ! ((lhs.x - rhs.x).abs() < epsilon) {
                panic!("assertion failed: {}.x does not approximately equal: {}.x", lhs, rhs);
            }
            if ! ((lhs.y - rhs.y).abs() < epsilon) {
                panic!("assertion failed: {}.y does not approximately equal: {}.y", lhs, rhs);
            }
        }
    }
}


#[cfg(test)]
mod test {
    use std::f64::consts::PI;
    use std::f64::consts::SQRT_2;

    use super::{Point, Vector};

    #[test]
    fn test_distance_to() {
        assert_approx_eq!(Point { x: 0.0, y: 0.0 }.distance_to(Point { x: 0.0, y: 0.0 }),
                          0.0,
                          0.000001);

        assert_approx_eq!(Point { x: 0.0, y: 0.0 }.distance_to(Point { x: 0.0, y: 1.0 }),
                          1.0,
                          0.000001);
        assert_approx_eq!(Point { x: 0.0, y: 0.0 }.distance_to(Point { x: 1.0, y: 0.0 }),
                          1.0,
                          0.000001);
        assert_approx_eq!(Point { x: 0.0, y: 0.0 }.distance_to(Point { x: 1.0, y: 1.0 }),
                          SQRT_2,
                          0.000000001);
        assert_approx_eq!(Point { x: 1.0, y: 1.0 }.distance_to(Point { x: 2.0, y: 2.0 }),
                          SQRT_2,
                          0.000000001);
        assert_approx_eq!(Point { x: 1.0, y: 1.0 }.distance_to(Point { x: 4.0, y: 5.0 }),
                          5.0,
                          0.000000001);

        assert_approx_eq!(Point { x: 4.0, y: 5.0 }.distance_to(Point { x: 1.0, y: 1.0 }),
                          5.0,
                          0.000000001);
    }

    #[test]
    fn test_vector_delta_x() {
        assert_approx_eq!(Vector {
                              direction: 0.0,
                              magnitude: 1.0,
                          }
                          .delta_x(),
                          1.0,
                          0.0000001);
        assert_approx_eq!(Vector {
                              direction: PI / 2.0,
                              magnitude: 1.0,
                          }
                          .delta_x(),
                          0.0,
                          0.0000001);
        assert_approx_eq!(Vector {
                              direction: PI / 4.0,
                              magnitude: 1.0,
                          }
                          .delta_x(),
                          (PI / 4.0).cos(),
                          0.0000001);

        assert_approx_eq!(Vector {
                              direction: PI / 4.0,
                              magnitude: 5.0,
                          }
                          .delta_x(),
                          (PI / 4.0).cos() * 5.0,
                          0.0000001);
        assert_approx_eq!(Vector {
                              direction: 3.0 * PI / 4.0,
                              magnitude: 5.0,
                          }
                          .delta_x(),
                          (PI / 4.0).cos() * -5.0,
                          0.0000001);

        assert_approx_eq!(Vector {
                              direction: PI,
                              magnitude: 1.0,
                          }
                          .delta_x(),
                          -1.0,
                          0.0000001);
        assert_approx_eq!(Vector {
                              direction: PI,
                              magnitude: 5.0,
                          }
                          .delta_x(),
                          -5.0,
                          0.0000001);

        assert_approx_eq!(Vector {
                              direction: 3.0 * PI / 2.0,
                              magnitude: 1.0,
                          }
                          .delta_x(),
                          0.0,
                          0.0000001);
        assert_approx_eq!(Vector {
                              direction: 3.0 * PI / 2.0,
                              magnitude: 5.0,
                          }
                          .delta_x(),
                          0.0,
                          0.0000001);
    }

    #[test]
    fn test_vector_delta_y() {
        assert_approx_eq!(Vector {
                              direction: 0.0,
                              magnitude: 1.0,
                          }
                          .delta_y(),
                          0.0,
                          0.0000001);
        assert_approx_eq!(Vector {
                              direction: PI / 2.0,
                              magnitude: 1.0,
                          }
                          .delta_y(),
                          1.0,
                          0.0000001);
        assert_approx_eq!(Vector {
                              direction: PI / 4.0,
                              magnitude: 1.0,
                          }
                          .delta_y(),
                          (PI / 4.0).sin(),
                          0.0000001);

        assert_approx_eq!(Vector {
                              direction: PI / 4.0,
                              magnitude: 5.0,
                          }
                          .delta_y(),
                          (PI / 4.0).sin() * 5.0,
                          0.0000001);
        assert_approx_eq!(Vector {
                              direction: 5.0 * PI / 4.0,
                              magnitude: 5.0,
                          }
                          .delta_y(),
                          (5.0 * PI / 4.0).sin() * 5.0,
                          0.0000001);

        assert_approx_eq!(Vector {
                              direction: PI,
                              magnitude: 1.0,
                          }
                          .delta_y(),
                          0.0,
                          0.0000001);
        assert_approx_eq!(Vector {
                              direction: PI,
                              magnitude: 5.0,
                          }
                          .delta_y(),
                          0.0,
                          0.0000001);
    }

    #[test]
    fn test_point_at() {
        assert_point_eq!(Point { x: 0.0, y: 0.0 }.point_at(Vector {
                             direction: 0.0,
                             magnitude: 1.0,
                         }),
                         Point { x: 1.0, y: 0.0 },
                         0.000000001);
        assert_point_eq!(Point { x: 0.0, y: 0.0 }.point_at(Vector {
                             direction: PI,
                             magnitude: 1.0,
                         }),
                         Point { x: -1.0, y: 0.0 },
                         0.000000001);
        assert_point_eq!(Point { x: 1.0, y: 0.0 }.point_at(Vector {
                             direction: PI,
                             magnitude: 1.0,
                         }),
                         Point { x: -0.0, y: 0.0 },
                         0.000000001);
        assert_point_eq!(Point { x: 1.0, y: 0.0 }.point_at(Vector {
                             direction: PI / 2.0,
                             magnitude: 1.0,
                         }),
                         Point { x: 1.0, y: 1.0 },
                         0.000000001);
    }
}
