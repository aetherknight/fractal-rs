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

pub mod burningship;
pub mod mandelbrot;

pub use num::complex::Complex64;

pub trait EscapeTime {
    /// The maximum number of iterations to perform before accepting that the value being iterated
    /// will not diverge.
    fn max_iterations(&self) -> u64;

    /// The default view area of the complex number plane, specified as 2 complex numbers. Although
    /// a rectangle is usually specified with top-left and bottom-right points, the caller is
    /// expected in this case to figure that out themselves.
    fn default_view_area(&self) -> [Complex64; 2];

    /// A single iteration of the function that defines this particular fractal.
    fn iterate(&self, c: Complex64, z: Complex64) -> Complex64;

    /// Tests whether a given complex number is in the fractal's set or if it diverges.
    ///
    /// The default implementation implements the mandelbrot test, which uses the complex number
    /// being tested as the constant `c`, and starts iteration with an input of 0+0i (Julia/Fatou
    /// sets would have to reimplement this).
    ///
    /// The default implementation uses EscapeTime::max_iterations() and EscapeTime::iterate().
    fn test_point(&self, point: Complex64) -> (bool, u64) {
        let mut zp = Complex64::new(0.0, 0.0);
        for i in 0..self.max_iterations() {
            zp = self.iterate(point, zp);
            if zp.norm() >= 2.0 {
                return (false, i);
            }
        }
        (true, 0)
    }
}
