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
        let rads = new_deg / 360.0 * 2.0 * PI;
        self.set_rad(rads);
    }

    fn set_rad(&mut self, new_rad: f64);

    /// Rotate the turtle, in degrees.
    ///
    /// Positive values turn the turtle "left" or counter-clockwise. Negative values turn the
    /// turtle "right" or clockwise.
    fn turn_deg(&mut self, degrees: f64) {
        let rads = degrees / 360.0 * 2.0 * PI;
        self.turn_rad(rads);
    }

    /// Convenience method for rotating the turtle in radians instead of degrees.
    ///
    /// 2*PI radians = 360 degrees.
    fn turn_rad(&mut self, radians: f64);

    /// Touch the turtle's pen to the drawing surface.
    fn down(&mut self);

    /// Lift the turtle's pen off of the drawing surface.
    fn up(&mut self);
}

/// An object that knows how to draw something using a Turtle.
pub trait TurtleApp {
    /// A method that uses a Turtle to draw something. Within an implementation, this method should
    /// manipulate the Turtle in order to draw something.
    fn draw(&self, turtle: &mut Turtle);
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
