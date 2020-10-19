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

use strum_macros::{EnumIter, EnumString, EnumVariantNames, IntoStaticStr};

// must be before any local modules that use the macros
#[macro_use]
pub mod macros;

pub mod chaosgame;
pub mod color;
pub mod curves;
pub mod escapetime;
pub mod geometry;
pub mod lindenmayer;
pub mod turtle;

/// Mainly used to categorize the fractals in a UI or menu.
#[derive(Copy, Clone)]
pub enum FractalCategory {
    ChaosGames,
    EscapeTimeFractals,
    TurtleCurves,
}

/// All of the supported fractals, with associated data for using them.
///
/// This type is meant to be extended or used by the various UI implementations to provide their
/// own code for constructing implementations for each kind of Fractal.
///
/// You can list all of them by using a derived iterator:
///
/// ```rust
/// use strum::IntoEnumIterator;
///
/// SelectedFractal::iter()
/// ```
///
/// You can parse a string token into one of these enums using something like:
///
/// ```rust
/// use std::str::FromStr;
///
/// SelectedFractal::from_str("dragon").unwrap()
/// ```
///
/// You can generate a static str representation using:
///
/// ```rust.ignore
/// <&'static str>::from(SelectedFractal::Dragon)
/// ```
///
/// Or:
///
/// ```rust.ignore
/// let slug: &'static str = SelectedFractal::Dragon.into()
/// ```
#[derive(Copy, Clone, EnumString, EnumIter, IntoStaticStr, EnumVariantNames)]
#[strum(serialize_all = "lowercase")]
pub enum SelectedFractal {
    BarnsleyFern,
    BurningMandel,
    BurningShip,
    Cesaro,
    CesaroTri,
    Dragon,
    KochCurve,
    LevyCCurve,
    Mandelbrot,
    RoadRunner,
    Sierpinski,
    TerDragon,
}

impl SelectedFractal {
    /// The full display name for each fractal variant.
    ///
    /// If you want a simpler ASCII name, use the `IntoStaticStr` derived definition, which lets
    /// you turn the Enum `into()` a `&'static str`.
    pub fn name(self) -> &'static str {
        match self {
            SelectedFractal::BarnsleyFern => "Barnsley Fern",
            SelectedFractal::BurningMandel => "Burning Mandel",
            SelectedFractal::BurningShip => "Burning Ship",
            SelectedFractal::Cesaro => "Cesàro",
            SelectedFractal::CesaroTri => "Cesàro Triangle",
            SelectedFractal::Dragon => "Dragon",
            SelectedFractal::KochCurve => "Koch Curve",
            SelectedFractal::LevyCCurve => "Lévy C Curve",
            SelectedFractal::Mandelbrot => "Mandelbrot",
            SelectedFractal::RoadRunner => "Roadrunner",
            SelectedFractal::Sierpinski => "Sierpiński Triangle",
            SelectedFractal::TerDragon => "Terdragon",
        }
    }

    /// A short description of each fractal variant.
    pub fn description(self) -> &'static str {
        match self {
            SelectedFractal::BarnsleyFern => "Draws the Barnsley Fern fractal using a chaos game with affine transforms.",
            SelectedFractal::BurningMandel => "Draws a variation of the burning ship fractal",
            SelectedFractal::BurningShip => "Draws the burning ship fractal",
            SelectedFractal::Cesaro => "Draws a square Cesàro fractal",
            SelectedFractal::CesaroTri => "Draws a triangle Cesàro fractal",
            SelectedFractal::Dragon => "Draws a dragon curve fractal",
            SelectedFractal::KochCurve => "Draws a Koch snowflake curve",
            SelectedFractal::LevyCCurve => "Draws a Lévy C Curve",
            SelectedFractal::Mandelbrot => "Draws the mandelbrot fractal",
            SelectedFractal::RoadRunner => "Draws a variation of the burning ship fractal",
            SelectedFractal::Sierpinski => "Draws a Sierpiński triangle using a chaos game and 3 randomly chosen points on the screen",
            SelectedFractal::TerDragon => "Draws a terdragon curve",
        }
    }

    /// Returns the category for the given variant.
    ///
    /// The categories relate to the kind of configuration that the given fractal needs.
    pub fn category(self) -> FractalCategory {
        match self {
            SelectedFractal::BarnsleyFern => FractalCategory::ChaosGames,
            SelectedFractal::BurningMandel => FractalCategory::EscapeTimeFractals,
            SelectedFractal::BurningShip => FractalCategory::EscapeTimeFractals,
            SelectedFractal::Cesaro => FractalCategory::TurtleCurves,
            SelectedFractal::CesaroTri => FractalCategory::TurtleCurves,
            SelectedFractal::Dragon => FractalCategory::TurtleCurves,
            SelectedFractal::KochCurve => FractalCategory::TurtleCurves,
            SelectedFractal::LevyCCurve => FractalCategory::TurtleCurves,
            SelectedFractal::Mandelbrot => FractalCategory::EscapeTimeFractals,
            SelectedFractal::RoadRunner => FractalCategory::EscapeTimeFractals,
            SelectedFractal::Sierpinski => FractalCategory::ChaosGames,
            SelectedFractal::TerDragon => FractalCategory::TurtleCurves,
        }
    }
}
