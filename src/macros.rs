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
/// This would use float-cmp and use ULPs instead of an epsilon, but ULPs and ratios (also provided
/// by float-cmp) do not work well for several cases. Checking ULPs does not work well when dealing
/// with calculations that use the approximation of PI, and that are themselves approximated
/// because the ULP distance can sometimes be huge. Ratios might work well, but they do not seem to
/// work well with values close to 0.0.
///
/// Not worrying about it, since this is for unit tests and not application logic.
macro_rules! assert_approx_eq {
    ( $lhs:expr, $rhs:expr, $epsilon:expr ) => {
        {
            let lhs = $lhs as f64;
            let rhs = $rhs as f64;
            let epsilon = ($epsilon as f64).abs();

            if ! ((lhs - rhs).abs() < epsilon) {
                panic!("assertion failed: {} does not approximately equal: {}", lhs, rhs);
            }
        }
    }
}

/// Macro to assert that two complex numbers are almost equal.
///
/// Complex numbers need to compare the modulus of their difference to the epsilon.
macro_rules! assert_complex_approx_eq {
    ( $lhs:expr, $rhs:expr, $epsilon:expr ) => {
        {
            use num::complex::Complex64;

            let lhs = ($lhs as Complex64);
            let rhs = ($rhs as Complex64);
            let epsilon = ($epsilon as f64).abs();

            if ! ((lhs - rhs).norm().abs() < epsilon) {
                panic!("assertion failed: {} does not approximately equal: {}", lhs, rhs);
            }
        }
    }
}

macro_rules! assert_complex_approx_in {
    ( $collection:expr, $rhs:expr, $epsilon:expr ) => {
        {
            use num::complex::Complex64;

            let collection = $collection as &[Complex64];
            let rhs = $rhs as Complex64;
            let epsilon = ($epsilon as f64).abs();

            let mut found = false;
            for item in collection {
                if (item - rhs).norm().abs() < epsilon {
                    found = true
                }
            }
            if ! found {
                panic!("assertion failed: {:?} does not approximately contain: {}", collection, rhs);
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
