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

use num::complex::Complex64;
use std::f64::consts::PI;
use std::fmt;

pub type Vec2d = [f64; 2];

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

impl Default for Point {
    fn default() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
}

impl Into<Complex64> for Point {
    fn into(self) -> Complex64 {
        Complex64::new(self.x, self.y)
    }
}

impl From<Complex64> for Point {
    fn from(c: Complex64) -> Point {
        Point { x: c.re, y: c.im }
    }
}

impl From<Vec2d> for Point {
    fn from(p: Vec2d) -> Point {
        Point { x: p[0], y: p[1] }
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

pub trait AffineTransform<T> {
    /// Apply the affine transform to the specified type (usually a vector,
    /// point, or similar object)
    fn transform(&self, v: T) -> T;
}

/// Row-major matrix for applying affine transforms to Cartesian points. Affine transformations on
/// the Cartesian plane are usually defined by 6 parameters that correspond to a 2x2 matrix and a
/// vector:
///
/// ```text
///     ⎡ x' ⎤ = ⎡ a b ⎤ * ⎡ x⎤ + ⎡ e ⎤
///     ⎣ y' ⎦   ⎣ c d ⎦   ⎣ y⎦   ⎣ f ⎦
/// ```
///
/// Or a 3x3 augmented matrix:
///
/// ```text
///     ⎡ x' ⎤   ⎡ a b e ⎤   ⎡ x ⎤
///     ⎢ y' ⎥ = ⎢ c d f ⎥ * ⎢ y ⎥
///     ⎣  1 ⎦   ⎣ 0 0 1 ⎦   ⎣ 1 ⎦
/// ```
///
/// For this implementation, we use a 3x2 matrix (we omit the third row).
pub type CartesianAffineTransform = [[f64; 3]; 2];

impl AffineTransform<Point> for CartesianAffineTransform {
    fn transform(&self, p: Point) -> Point {
        Point {
            x: self[0][0] * p.x + self[0][1] * p.y + self[0][2],
            y: self[1][0] * p.x + self[1][1] * p.y + self[1][2],
        }
    }
}

/// Ensures that the cartesian area specified by `top_left` and `bot_right` fit into the
/// `view_area` of a window/viewport without distortion.
///
/// It essentially implements a set of affine transforms from one coordinate space to another.
/// However, it limits these transforms in a few specialized ways:
///
/// * It flips the Y axis. The positive direction for the screen is down and right, but in graphing
///   the positive direction is usually up and right.
/// * It ensures that the view area is not stretched or squished, limiting the transforms to
///   zooming and shifting.
pub struct ViewAreaTransformer {
    // view_area_size: Vec2d,
    top_left: Point,
    // bot_right: Point,
    scale: f64,
    offset_factor_x: f64,
    offset_factor_y: f64,
}

impl ViewAreaTransformer {
    /// Initializes a ViewAreaTranformer using the size of the view area, and two points that
    /// define a rectangle in the cartesian plane that should be visible in the view area.
    pub fn new(view_area_size: Vec2d, a: Point, b: Point) -> ViewAreaTransformer {
        let window_width = view_area_size[0];
        let window_height = view_area_size[1];
        let cart_width = (a.x - b.x).abs();
        let cart_height = (a.y - b.y).abs();

        let scale = Self::compute_scale(window_width, window_height, cart_width, cart_height);
        let (offset_factor_x, offset_factor_y) = Self::compute_offset_factors(
            window_width,
            window_height,
            cart_width,
            cart_height,
            scale,
        );

        ViewAreaTransformer {
            // view_area_size: view_area_size,
            top_left: Point {
                x: a.x.min(b.x),
                y: a.y.max(b.y),
            },
            // bot_right: Point {
            //     x: a.x.max(b.x),
            //     y: a.y.min(b.y),
            // },
            scale,
            offset_factor_x,
            offset_factor_y,
        }
    }

    fn compute_scale(
        window_width: f64,
        window_height: f64,
        cart_width: f64,
        cart_height: f64,
    ) -> f64 {
        if (cart_height / cart_width) > (window_height / window_width) {
            cart_height / window_height
        } else {
            (cart_width / window_width)
        }
    }

    fn compute_offset_factors(
        window_width: f64,
        window_height: f64,
        cart_width: f64,
        cart_height: f64,
        scale: f64,
    ) -> (f64, f64) {
        if (cart_height / cart_width) > (window_height / window_width) {
            (((window_width * scale - cart_width) / 2.0), 0.0)
        } else {
            (0.0, ((window_height * scale - cart_height) / 2.0))
        }
    }

    /// Calculates the cartesian point that exists at a given pixel-location on the
    /// window/viewport.
    pub fn map_pixel_to_point(&self, screen_coord: Vec2d) -> Point {
        Point {
            x: screen_coord[0] * self.scale + self.top_left.x - (self.offset_factor_x),
            y: -(screen_coord[1] * self.scale) + self.top_left.y + (self.offset_factor_y),
        }
    }

    pub fn map_point_to_pixel(&self, point: Point) -> Vec2d {
        [
            (point.x - self.top_left.x + self.offset_factor_x) / self.scale,
            -(point.y - self.top_left.y - self.offset_factor_y) / self.scale,
        ]
    }
}

/// Implements pow for complex numbers.
pub fn cpow(c: Complex64, exponent: u64) -> Complex64 {
    match exponent {
        0 => Complex64::new(1.0, 0.0),
        1 => c,
        2 => c * c,
        _ => {
            let mut accum: Complex64 = c;
            for _ in 1..exponent {
                accum *= c;
            }
            accum
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use num::complex::Complex64;
    use std::f64::consts::PI;
    use std::f64::consts::SQRT_2;

    #[test]
    fn test_distance_to() {
        assert_approx_eq!(
            Point { x: 0.0, y: 0.0 }.distance_to(Point { x: 0.0, y: 0.0 }),
            0.0,
            0.000001
        );

        assert_approx_eq!(
            Point { x: 0.0, y: 0.0 }.distance_to(Point { x: 0.0, y: 1.0 }),
            1.0,
            0.000001
        );
        assert_approx_eq!(
            Point { x: 0.0, y: 0.0 }.distance_to(Point { x: 1.0, y: 0.0 }),
            1.0,
            0.000001
        );
        assert_approx_eq!(
            Point { x: 0.0, y: 0.0 }.distance_to(Point { x: 1.0, y: 1.0 }),
            SQRT_2,
            0.000000001
        );
        assert_approx_eq!(
            Point { x: 1.0, y: 1.0 }.distance_to(Point { x: 2.0, y: 2.0 }),
            SQRT_2,
            0.000000001
        );
        assert_approx_eq!(
            Point { x: 1.0, y: 1.0 }.distance_to(Point { x: 4.0, y: 5.0 }),
            5.0,
            0.000000001
        );

        assert_approx_eq!(
            Point { x: 4.0, y: 5.0 }.distance_to(Point { x: 1.0, y: 1.0 }),
            5.0,
            0.000000001
        );
    }

    #[test]
    fn test_point_from_complex() {
        let c = Complex64::new(-23.2, 45.9);
        assert_eq!(Point::from(c).x, -23.2);
        assert_eq!(Point::from(c).y, 45.9);
    }

    #[test]
    fn test_point_from_vec2d() {
        let vec2d: Vec2d = [123.0, 456.5];
        assert_eq!(Point::from(vec2d).x, 123.0);
        assert_eq!(Point::from(vec2d).y, 456.5);
    }

    #[test]
    fn test_vector_delta_x() {
        assert_approx_eq!(
            Vector {
                direction: 0.0,
                magnitude: 1.0,
            }
            .delta_x(),
            1.0,
            0.0000001
        );
        assert_approx_eq!(
            Vector {
                direction: PI / 2.0,
                magnitude: 1.0,
            }
            .delta_x(),
            0.0,
            0.0000001
        );
        assert_approx_eq!(
            Vector {
                direction: PI / 4.0,
                magnitude: 1.0,
            }
            .delta_x(),
            (PI / 4.0).cos(),
            0.0000001
        );

        assert_approx_eq!(
            Vector {
                direction: PI / 4.0,
                magnitude: 5.0,
            }
            .delta_x(),
            (PI / 4.0).cos() * 5.0,
            0.0000001
        );
        assert_approx_eq!(
            Vector {
                direction: 3.0 * PI / 4.0,
                magnitude: 5.0,
            }
            .delta_x(),
            (PI / 4.0).cos() * -5.0,
            0.0000001
        );

        assert_approx_eq!(
            Vector {
                direction: PI,
                magnitude: 1.0,
            }
            .delta_x(),
            -1.0,
            0.0000001
        );
        assert_approx_eq!(
            Vector {
                direction: PI,
                magnitude: 5.0,
            }
            .delta_x(),
            -5.0,
            0.0000001
        );

        assert_approx_eq!(
            Vector {
                direction: 3.0 * PI / 2.0,
                magnitude: 1.0,
            }
            .delta_x(),
            0.0,
            0.0000001
        );
        assert_approx_eq!(
            Vector {
                direction: 3.0 * PI / 2.0,
                magnitude: 5.0,
            }
            .delta_x(),
            0.0,
            0.0000001
        );
    }

    #[test]
    fn test_vector_delta_y() {
        assert_approx_eq!(
            Vector {
                direction: 0.0,
                magnitude: 1.0,
            }
            .delta_y(),
            0.0,
            0.0000001
        );
        assert_approx_eq!(
            Vector {
                direction: PI / 2.0,
                magnitude: 1.0,
            }
            .delta_y(),
            1.0,
            0.0000001
        );
        assert_approx_eq!(
            Vector {
                direction: PI / 4.0,
                magnitude: 1.0,
            }
            .delta_y(),
            (PI / 4.0).sin(),
            0.0000001
        );

        assert_approx_eq!(
            Vector {
                direction: PI / 4.0,
                magnitude: 5.0,
            }
            .delta_y(),
            (PI / 4.0).sin() * 5.0,
            0.0000001
        );
        assert_approx_eq!(
            Vector {
                direction: 5.0 * PI / 4.0,
                magnitude: 5.0,
            }
            .delta_y(),
            (5.0 * PI / 4.0).sin() * 5.0,
            0.0000001
        );

        assert_approx_eq!(
            Vector {
                direction: PI,
                magnitude: 1.0,
            }
            .delta_y(),
            0.0,
            0.0000001
        );
        assert_approx_eq!(
            Vector {
                direction: PI,
                magnitude: 5.0,
            }
            .delta_y(),
            0.0,
            0.0000001
        );
    }

    #[test]
    fn test_point_at() {
        assert_point_eq!(
            Point { x: 0.0, y: 0.0 }.point_at(Vector {
                direction: 0.0,
                magnitude: 1.0,
            }),
            Point { x: 1.0, y: 0.0 },
            0.000000001
        );
        assert_point_eq!(
            Point { x: 0.0, y: 0.0 }.point_at(Vector {
                direction: PI,
                magnitude: 1.0,
            }),
            Point { x: -1.0, y: 0.0 },
            0.000000001
        );
        assert_point_eq!(
            Point { x: 1.0, y: 0.0 }.point_at(Vector {
                direction: PI,
                magnitude: 1.0,
            }),
            Point { x: -0.0, y: 0.0 },
            0.000000001
        );
        assert_point_eq!(
            Point { x: 1.0, y: 0.0 }.point_at(Vector {
                direction: PI / 2.0,
                magnitude: 1.0,
            }),
            Point { x: 1.0, y: 1.0 },
            0.000000001
        );
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

    #[test]
    fn test_cartesian_affine_transform() {
        let test_point = Point { x: 1.45, y: 6.78 };

        let identity: CartesianAffineTransform = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        assert_point_eq!(identity.transform(test_point), test_point, 0.0000000001);

        let move_right: CartesianAffineTransform = [[1.0, 0.0, 1.0], [0.0, 1.0, 0.0]];
        assert_point_eq!(
            move_right.transform(test_point),
            Point { x: 2.45, y: 6.78 },
            0.0000000001
        );

        let mirror_x: CartesianAffineTransform = [[-1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        assert_point_eq!(
            mirror_x.transform(test_point),
            Point { x: -1.45, y: 6.78 },
            0.0000000001
        );

        let shrink_and_move: CartesianAffineTransform = [[0.5, 0.0, 1.2], [0.0, 0.5, -5.0]];
        assert_point_eq!(
            shrink_and_move.transform(Point { x: 5.0, y: 4.9 }),
            Point {
                x: 0.5 * 5.0 + 0.0 + 1.2,
                y: 0.0 + 4.9 * 0.5 - 5.0,
            },
            0.0000000001
        );
    }

    /// 800x600 -> [(-1,1),(1,-1)], flip y
    #[test]
    fn test_view_area_transformer_map_pixel1() {
        // |--100--|--600--|--100--|
        //         -1  0   1
        let screen_size = [800.0, 600.0];
        let top_left = Point { x: -1.0, y: 1.0 };
        let bot_right = Point { x: 1.0, y: -1.0 };
        let vat = ViewAreaTransformer::new(screen_size, top_left, bot_right);

        assert_approx_eq!(vat.map_pixel_to_point([0.0, 0.0]).y, 1.0, 0.0000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([0.0, 300.0]).y, 0.0, 0.0000000000001);
        assert_approx_eq!(
            vat.map_pixel_to_point([0.0, 600.0]).y,
            -1.0,
            0.0000000000001
        );

        assert_approx_eq!(vat.map_pixel_to_point([100.0, 0.0]).x, -1.0, 0.000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([400.0, 0.0]).x, 0.0, 0.000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([700.0, 0.0]).x, 1.0, 0.000000000001);
        assert_approx_eq!(
            vat.map_pixel_to_point([0.0, 0.0]).x,
            -1.0 - (1.0 / 3.0),
            0.0000000000001
        );
        assert_approx_eq!(
            vat.map_pixel_to_point([800.0, 0.0]).x,
            1.0 + (1.0 / 3.0),
            0.0000000000001
        );

        // the screen is wider than it is tall, point 1.0 is at 600 (+ 100 offset to center)
        assert_approx_eq!(vat.map_point_to_pixel(Point{x:1.0, y: 1.0})[0], 700.0, 0.000000000001);
        assert_approx_eq!(vat.map_point_to_pixel(Point{x:1.0, y: 1.0})[1], 0.0, 0.000000000001);

        assert_approx_eq!(vat.map_point_to_pixel(Point{x:-1.0, y: -1.0})[0], 100.0, 0.000000000001);
        assert_approx_eq!(vat.map_point_to_pixel(Point{x:-1.0, y: -1.0})[1], 600.0, 0.000000000001);
    }

    /// 600x800 -> [(-1,1),(1,-1)], flip y
    #[test]
    fn test_view_area_transformer_map_pixel2() {
        let screen_size = [600.0, 800.0];
        let top_left = Point { x: -1.0, y: 1.0 };
        let bot_right = Point { x: 1.0, y: -1.0 };
        let vat = ViewAreaTransformer::new(screen_size, top_left, bot_right);

        assert_approx_eq!(
            vat.map_pixel_to_point([0.0, 0.0]).y,
            1.0 + (1.0 / 3.0),
            0.0000000000001
        );
        assert_approx_eq!(vat.map_pixel_to_point([0.0, 100.0]).y, 1.0, 0.0000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([0.0, 400.0]).y, 0.0, 0.0000000000001);
        assert_approx_eq!(
            vat.map_pixel_to_point([0.0, 700.0]).y,
            -1.0,
            0.0000000000001
        );
        assert_approx_eq!(
            vat.map_pixel_to_point([0.0, 800.0]).y,
            -1.0 - (1.0 / 3.0),
            0.0000000000001
        );

        assert_approx_eq!(vat.map_pixel_to_point([0.0, 0.0]).x, -1.0, 0.000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([300.0, 0.0]).x, 0.0, 0.000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([600.0, 0.0]).x, 1.0, 0.000000000001);

        assert_approx_eq!(vat.map_point_to_pixel(Point{x:1.0, y: 1.0})[0], 600.0, 0.000000000001);
        assert_approx_eq!(vat.map_point_to_pixel(Point{x:1.0, y: 1.0})[1], 100.0, 0.000000000001);

        assert_approx_eq!(vat.map_point_to_pixel(Point{x:-1.0, y: -1.0})[0], 000.0, 0.000000000001);
        assert_approx_eq!(vat.map_point_to_pixel(Point{x:-1.0, y: -1.0})[1], 700.0, 0.000000000001);
    }

    /// 3x4 window, and [(3,12),12,3)]
    #[test]
    fn test_view_area_transformer_map_pixel3() {
        let screen_size = [3.0, 4.0];
        let top_left = Point { x: 3.0, y: 12.0 };
        let bot_right = Point { x: 12.0, y: 3.0 };
        let vat = ViewAreaTransformer::new(screen_size, top_left, bot_right);

        assert_approx_eq!(vat.map_pixel_to_point([0.0, 0.5]).y, 12.0, 0.0000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([0.0, 3.5]).y, 3.0, 0.0000000000001);

        assert_approx_eq!(vat.map_pixel_to_point([0.0, 0.0]).x, 3.0, 0.000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([3.0, 0.0]).x, 12.0, 0.000000000001);

        assert_approx_eq!(vat.map_point_to_pixel(Point{x:3.0, y: 3.0})[0], 0.0, 0.000000000001);
        assert_approx_eq!(vat.map_point_to_pixel(Point{x:3.0, y: 3.0})[1], 3.5, 0.000000000001);

        assert_approx_eq!(vat.map_point_to_pixel(Point{x:12.0, y: 12.0})[0], 3.0, 0.000000000001);
        assert_approx_eq!(vat.map_point_to_pixel(Point{x:12.0, y: 12.0})[1], 0.5, 0.000000000001);
    }

    /// 3x4 window, and [(3,3),12,12)] can handle rectangle that is not
    /// top-left and bottom-right
    #[test]
    fn test_view_area_transformer_map_pixel4() {
        let screen_size = [3.0, 4.0];
        let top_left = Point { x: 3.0, y: 3.0 };
        let bot_right = Point { x: 12.0, y: 12.0 };
        let vat = ViewAreaTransformer::new(screen_size, top_left, bot_right);

        assert_approx_eq!(vat.map_pixel_to_point([0.0, 0.5]).y, 12.0, 0.0000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([0.0, 3.5]).y, 3.0, 0.0000000000001);

        assert_approx_eq!(vat.map_pixel_to_point([0.0, 0.0]).x, 3.0, 0.000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([3.0, 0.0]).x, 12.0, 0.000000000001);

        assert_approx_eq!(vat.map_point_to_pixel(Point{x:3.0, y: 3.0})[0], 0.0, 0.000000000001);
        assert_approx_eq!(vat.map_point_to_pixel(Point{x:3.0, y: 3.0})[1], 3.5, 0.000000000001);

        assert_approx_eq!(vat.map_point_to_pixel(Point{x:12.0, y: 12.0})[0], 3.0, 0.000000000001);
        assert_approx_eq!(vat.map_point_to_pixel(Point{x:12.0, y: 12.0})[1], 0.5, 0.000000000001);
    }

    /// 800x600 window, and [(-2,1),1,-1)]
    #[test]
    fn test_view_area_transformer_map_pixel5() {
        // window: 4:3 -> window is narrower
        // view area: 3:2
        //
        let screen_size = [800.0, 600.0];
        let top_left = Point { x: -2.0, y: 1.0 };
        let bot_right = Point { x: 1.0, y: -1.0 };
        let vat = ViewAreaTransformer::new(screen_size, top_left, bot_right);

        // 0,0 maps to
        assert_approx_eq!(
            vat.map_pixel_to_point([0.0, 0.0]).y,
            1.0 + 1.0 / 8.0,
            0.0000000000001
        );
        assert_approx_eq!(
            vat.map_pixel_to_point([0.0, 600.0]).y,
            -1.0 - 1.0 / 8.0,
            0.0000000000001
        );

        assert_approx_eq!(vat.map_pixel_to_point([0.0, 0.0]).x, -2.0, 0.000000000001);
        assert_approx_eq!(vat.map_pixel_to_point([800.0, 0.0]).x, 1.0, 0.000000000001);
    }

    #[test]
    fn test_cpow() {
        assert_eq!(cpow(Complex64::new(5.5, 0.0), 0), Complex64::new(1.0, 0.0));
        assert_eq!(cpow(Complex64::new(5.5, 0.0), 1), Complex64::new(5.5, 0.0));
        assert_eq!(
            cpow(Complex64::new(5.5, 0.0), 2),
            Complex64::new(5.5f64 * 5.5f64, 0.0)
        );
        assert_eq!(
            cpow(Complex64::new(5.5, 0.0), 3),
            Complex64::new(5.5f64 * 5.5f64 * 5.5f64, 0.0)
        );
        assert_eq!(
            cpow(Complex64::new(5.5, 0.0), 4),
            Complex64::new(5.5f64 * 5.5f64 * 5.5f64 * 5.5f64, 0.0)
        );

        assert_eq!(cpow(Complex64::new(5.5, 1.0), 0), Complex64::new(1.0, 0.0));
        assert_eq!(cpow(Complex64::new(5.5, 1.0), 1), Complex64::new(5.5, 1.0));
        assert_eq!(
            cpow(Complex64::new(5.5, 1.0), 2),
            Complex64::new(5.5 * 5.5 - 1.0 * 1.0, 2.0 * 5.5 * 1.0)
        );
        assert_eq!(
            cpow(Complex64::new(5.5, 1.0), 3),
            Complex64::new(
                5.5 * 5.5 * 5.5 - 3.0 * 5.5 * 1.0 * 1.0,
                3.0 * 5.5 * 5.5 * 1.0 - 1.0 * 1.0 * 1.0
            )
        );
    }
}
