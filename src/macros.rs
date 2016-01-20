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
