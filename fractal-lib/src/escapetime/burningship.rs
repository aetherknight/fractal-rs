// Copyright (c) 2015-2019 William (B.J.) Snow Orvis
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

//! The burning ship fractal is an iterated function system similar to Mandelbrot, but it is over
//! the complex number function:
//!
//! ```text
//! f(z) = (abs(Re(z)) - i*abs(Im(z)))^2 + c
//! ```
//!
//! It differs from the Mandelbrot equation in that the real and imaginary components of z are
//! always set to their absolute value before squaring them. Also, the absolute value of the
//! imaginary component of z is sometimes added instead of subtracted, which makes the fractal flip
//! upside down.
//!
//! This module also contains a few other variations of the burning ship fractal that differ by
//! only taking the absolute value of one of the components of z (taking no absolute value would be
//! the mandelbrot set)

use super::super::geometry;
use super::*;

pub struct BurningShip {
    max_iters: u64,
    power: u64,
}

impl BurningShip {
    /// Creates a new escape time specification for the burning ship family of fractals.
    ///
    /// `max_iterations` specifies the cutoff iteration for deciding whether a complex number
    /// escapes or has converged.
    ///
    /// `power` specifies the exponent used in the burning ship equation. The burning ship
    /// fractal has an exponent of 2, but this allows for an exponent of 3, 4, etc. to explore
    /// these related fractals. See <https://theory.org/fracdyn/burningship/symmetry.html> for
    /// examples of what these may look like.
    pub fn new(max_iterations: u64, power: u64) -> BurningShip {
        BurningShip {
            max_iters: max_iterations,
            power,
        }
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
        geometry::cpow(absz, self.power) + c
    }
}

/// Variation of the burning ship and mandelbrot fractals.
///
/// I made the name up, since I could not find a name online for this variation. Where the
/// `BurningShip` is defined as:
///
/// ```text
/// f(z) = (abs(Re(z)) - i*abs(Im(z)))^2 + c
/// ```
///
/// This fractal is defined as:
///
/// ```text
/// f(z) = (abs(Re(z)) - i*Im(z))^2 + c
/// ```
///
/// Where only the Real part of z is converted to its absolute value.
pub struct BurningMandel {
    max_iters: u64,
    power: u64,
}

impl BurningMandel {
    /// Creates a new escape time specification for the burning ship family of fractals.
    ///
    /// `max_iterations` specifies the cutoff iteration for deciding whether a complex number
    /// escapes or has converged.
    ///
    /// `power` specifies the exponent used in the burning ship equation. The burning ship
    /// fractal has an exponent of 2, but this allows for an exponent of 3, 4, etc. to explore
    /// these related fractals. See <https://theory.org/fracdyn/burningship/symmetry.html> for
    /// examples of what these may look like.
    pub fn new(max_iterations: u64, power: u64) -> BurningMandel {
        BurningMandel {
            max_iters: max_iterations,
            power,
        }
    }
}

impl EscapeTime for BurningMandel {
    fn max_iterations(&self) -> u64 {
        self.max_iters
    }

    fn default_view_area(&self) -> [Complex64; 2] {
        [Complex64::new(-2.5, 1.0), Complex64::new(1.5, -1.0)]
    }

    fn iterate(&self, c: Complex64, z: Complex64) -> Complex64 {
        let absz = Complex64::new(z.re.abs(), -z.im);
        geometry::cpow(absz, self.power) + c
    }
}

/// Variation of the burning ship and mandelbrot fractals.
///
/// I made the name up, since I could not find a name online for this variation. Where the
/// `BurningShip` is defined as:
///
/// ```text
/// f(z) = (abs(Re(z)) - i*abs(Im(z)))^2 + c
/// ```
///
/// This fractal is defined as:
///
/// ```text
/// f(z) = (Re(z) - i*abs(Im(z)))^2 + c
/// ```
///
/// Where only the Imaginary part of z is converted to its absolute value.
pub struct RoadRunner {
    max_iters: u64,
    power: u64,
}

impl RoadRunner {
    /// Creates a new escape time specification for the burning ship family of fractals.
    ///
    /// `max_iterations` specifies the cutoff iteration for deciding whether a complex number
    /// escapes or has converged.
    ///
    /// `power` specifies the exponent used in the burning ship equation. The burning ship
    /// fractal has an exponent of 2, but this allows for an exponent of 3, 4, etc. to explore
    /// these related fractals. See <https://theory.org/fracdyn/burningship/symmetry.html> for
    /// examples of what these may look like.
    pub fn new(max_iterations: u64, power: u64) -> RoadRunner {
        RoadRunner {
            max_iters: max_iterations,
            power,
        }
    }
}

impl EscapeTime for RoadRunner {
    fn max_iterations(&self) -> u64 {
        self.max_iters
    }

    fn default_view_area(&self) -> [Complex64; 2] {
        [Complex64::new(-2.5, 1.5), Complex64::new(1.5, -1.5)]
    }

    fn iterate(&self, c: Complex64, z: Complex64) -> Complex64 {
        let absz = Complex64::new(z.re, -z.im.abs());
        geometry::cpow(absz, self.power) + c
    }
}
