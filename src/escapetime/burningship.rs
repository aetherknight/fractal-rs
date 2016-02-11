// Copyright (c) 2016 William (B.J.) Snow Orvis
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

//! The burning ship fractal is an iterated function system similar to
//! Mandelbrot, but it is over the complex number function:
//! ```
//! f(z) = (abs(Re(z)) - i*abs(Im(z)))^2 + c
//! ```
//! It differs from the Mandelbrot equation in that the real and imaginary
//! components of z are always set to their absolute value before squaring
//! them. Also, the absolute value of the imaginary component of z is sometimes
//! added instead of subtracted, which makes the fractal flip upside down.

use super::*;

pub struct BurningShip {
    max_iters: u64,
}

impl BurningShip {
    pub fn new(max_iterations: u64) -> BurningShip {
        BurningShip { max_iters: max_iterations }
    }
}

impl EscapeTime for BurningShip {
    fn max_iterations(&self) -> u64 {
        self.max_iters
    }

    fn default_view_area(&self) -> [Complex64; 2] {
        [Complex64::new(-2.5, 2.0), Complex64::new(1.5, -1.0)]
    }


    fn iterate(&self, c: Complex64, z: Complex64) -> Complex64 {
        let absz = Complex64::new(z.re.abs(), -z.im.abs());
        absz * absz + c
    }
}

// #[cfg(test)]
// mod test {
//     use super::*;
//     use num::complex::Complex64;

//     #[test]
//     fn test_test_point() {
//         let mb = BurningShip::new(100);
//         assert!(mb.test_point(Complex64::new(0.0, 0.0)));
//         assert!(mb.test_point(Complex64::new(-1.0, 0.0)));
//         assert!(!mb.test_point(Complex64::new(1.0, 0.0)));
//         assert!(!mb.test_point(Complex64::new(-0.8, 0.35)));
//     }
// }
