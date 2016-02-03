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

//! Library of things to explore and draw various fractal curves.
extern crate argparse;
extern crate graphics;
extern crate piston;
extern crate piston_window;
extern crate rand;

// must be before any local modules that use the macros
#[macro_use]
mod macros;

pub mod chaosgame;
pub mod curves;
pub mod geometry;
pub mod lindenmayer;
pub mod pistonrendering;
pub mod turtle;
