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
    fn max_iterations(&self) -> u64;

    fn iterate(&self, c: Complex64, z: Complex64) -> Complex64;

    fn test_point(&self, point: Complex64) -> bool {
        let mut zp = Complex64::new(0.0, 0.0);
        for _ in 0..self.max_iterations() {
            zp = self.iterate(point, zp);
            if zp.norm() >= 2.0 {
                return false;
            }
        }
        true
    }
}
