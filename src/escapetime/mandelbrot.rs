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

//! The Mandelbrot fractal is an iterated function system defined over the
//! complex number function:
//! ```
//! f(z) = z^2 + c
//! ```
//! A given point, which corresponds to `c`, belongs to the Mandelbrot set if
//! iterating on f(0) converges. That is, f(0), f(f(0)), f(f(f(0))), ...
//! converges. Alternately, if it diverges (trends towards a value of ∞ ), then
//! a point `c` is not in the Mandelbrot set.

use super::*;

pub struct Mandelbrot {
    max_iters: u64,
}

impl Mandelbrot {
    pub fn new(max_iterations: u64) -> Mandelbrot {
        Mandelbrot { max_iters: max_iterations }
    }
}

impl EscapeTime for Mandelbrot {
    fn max_iterations(&self) -> u64 {
        self.max_iters
    }

    fn default_view_area(&self) -> [Complex64; 2] {
        [Complex64::new(-2.0, 1.0), Complex64::new(1.0, -1.0)]
    }

    fn iterate(&self, c: Complex64, z: Complex64) -> Complex64 {
        z * z + c
    }
}

#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;

    #[test]
    fn test_test_point() {
        let mb = Mandelbrot::new(100);
        assert!(mb.test_point(Complex64::new(0.0, 0.0)));
        assert!(mb.test_point(Complex64::new(-1.0, 0.0)));
        assert!(!mb.test_point(Complex64::new(1.0, 0.0)));
        assert!(!mb.test_point(Complex64::new(-0.8, 0.35)));
    }
}