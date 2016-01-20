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

//! Various types and functions the work within a 2-D cartesian coordinate
//! system.

use std::f64::consts::PI;
use std::fmt;

/// Represents a point in a 2-D cartesian coordinate system.
#[derive(Copy, Clone, Debug, PartialEq)]
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

/// Converts degrees into radians.
pub fn deg2rad(degrees: f64) -> f64 {
    degrees / 360.0 * 2.0 * PI
}

#[cfg(test)]
mod test {
    use std::f64::consts::PI;
    use std::f64::consts::SQRT_2;

    use super::{Point, Vector, deg2rad};

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

    #[test]
    fn test_deg2rad() {
        assert_approx_eq!(deg2rad(0.0), 0.0, 0.000000001);
        assert_approx_eq!(deg2rad(60.0), PI / 3.0, 0.000000001);
        assert_approx_eq!(deg2rad(90.0), PI / 2.0, 0.000000001);
        assert_approx_eq!(deg2rad(120.0), 2.0 * PI / 3.0, 0.000000001);
        assert_approx_eq!(deg2rad(180.0), PI, 0.000000001);
        assert_approx_eq!(deg2rad(360.0), 2.0 * PI, 0.000000001);
    }
}
