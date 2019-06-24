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

//! Exports the various fractal curves so that they can be called and animated/rendered from
//! JavaScript in a browser.
//!
//! Each fractal type has its own factory function that returns an object that implements the
//! following interface/protocol (although traits don't map to JS):
//!
//! ```
//! trait FractalAnimation {
//!     fn draw_one_frame(&mut self) -> bool;
//!     fn pixel_to_coordinate(&self, x: f64, y: f64) -> Array;
//! }
//! ```

use console_error_panic_hook;
use fractal_lib::chaosgame::barnsleyfern;
use fractal_lib::chaosgame::sierpinski;
use fractal_lib::curves::cesaro;
use fractal_lib::curves::cesarotri;
use fractal_lib::curves::dragon;
use fractal_lib::curves::kochcurve;
use fractal_lib::curves::levyccurve;
use fractal_lib::curves::terdragon;
use fractal_lib::escapetime::burningship::{BurningMandel, BurningShip, RoadRunner};
use fractal_lib::escapetime::mandelbrot::Mandelbrot;
use fractal_lib::lindenmayer::LindenmayerSystemTurtleProgram;
use paste;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

mod chaosgame;
mod escapetime;
mod turtle;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

/// Macro that generates a function for constructing a TurtleAnimation for a particular kind of
/// turtle-based curve.
///
/// It takes a name identifier, a colon, and then expression that should evaluate to a
/// TurtleProgram. The expression may use `iteration` in order to configure the TurtleProgram.
///
/// For example:
/// ```rust,ignore
/// animated_turtle!(dragon: dragon::DragonFractal::new(u64::from(iteration)));
/// ```
///
/// Will create a function with signature:
///
/// ```rust, ignore
/// #[wasm_bindgen]
/// pub fn animated_dragon(canvas: &HtmlCanvaselement, iteration: u32) -> turtle::TurtleAnimation;
/// ```
///
/// It will blank out the screen, start the TurtleAnimation, and then return it. The caller may
/// then call `draw_one_frame` on future frames/ticks to update/animate the canvas.
///
/// Note: `iteration` is a u32 in the function signature (and not a u64) because as of 2019/06/08,
/// wasm-bindgen uses BigUint64Array to help pass around 64-bit unsigned integers, but
/// BigUint64Array has not yet been standardized/ Firefox 67 does not yet support BigUint64Array or
/// bigints.
macro_rules! animated_turtle {
    ($name:ident: $expr:expr) => {
        // Paste is needed to concatenate render_ and the name of the fractal. Rust's own macros
        // don't provide a good way to do this.
        paste::item! {
            /// Blanks the canvas and constructs a `TurtleAnimation` that represents the given
            /// curve, and then returns the `TurtleAnimation`, allowing the call to render
            /// additional frames.
            ///
            /// The iteration specifies which iteration of the TurtleProgram it will draw.
            #[wasm_bindgen]
            pub fn [<animated_ $name>] (
                canvas: &HtmlCanvasElement,
                iteration: u32
            ) -> turtle::TurtleAnimation {
                console::log_1(&format!("Starting animation {}", stringify!($name)).into());
                let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                let program = $expr;
                ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                turtle::TurtleAnimation::new(ctx, &program)
            }
        }
    };
}

animated_turtle!(
    cesaro: LindenmayerSystemTurtleProgram::new(cesaro::CesaroFractal::new(u64::from(iteration)))
);
animated_turtle!(
    cesarotri:
        LindenmayerSystemTurtleProgram::new(cesarotri::CesaroTriFractal::new(u64::from(iteration)))
);
animated_turtle!(dragon: dragon::DragonFractal::new(u64::from(iteration)));
animated_turtle!(
    kochcurve: LindenmayerSystemTurtleProgram::new(kochcurve::KochCurve::new(u64::from(iteration)))
);
animated_turtle!(
    levyccurve:
        LindenmayerSystemTurtleProgram::new(levyccurve::LevyCCurve::new(u64::from(iteration)))
);
animated_turtle!(
    terdragon:
        LindenmayerSystemTurtleProgram::new(terdragon::TerdragonFractal::new(u64::from(iteration)))
);

/// Macro that generates a function for constructing (and starting) a ChaosGameAnimation for a
/// particular kind of ChaosGame.
///
/// It takes a name identifier, a colon, and then expression that should evaluate to a
/// a ChaosGame.
///
/// For example:
/// ```rust,ignore
/// animated_chaos_game!(sierpinski: sierpinski::SierpinskiChaosGame::new());
/// ```
///
/// Will create a function with signature:
///
/// ```rust,ignore
/// #[wasm_bindgen]
/// pub fn animated_sierpinski(canvas: &HtmlCanvaselement) -> ChaosGameAnimation;
/// ```
///
/// It will blank out the screen, start the ChaosGameAnimation, and then return it. The caller may
/// then call `draw_one_frame` on future frames/ticks to update/animate the canvas.
macro_rules! animated_chaos_game {
    ($name:ident: $expr:expr) => {
        // Paste is needed to concatenate render_ and the name of the fractal. Rust's own macros
        // don't provide a good way to do this.
        paste::item! {
            #[wasm_bindgen]
            pub fn [<animated_ $name>] (
                canvas: &HtmlCanvasElement
            ) -> chaosgame::ChaosGameAnimation {
                console::log_1(&format!("Starting animation {}", stringify!($name)).into());
                let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                chaosgame::ChaosGameAnimation::new(ctx, Box::new($expr))
            }
        }
    };
}

animated_chaos_game!(
    barnsleyfern:
        barnsleyfern::BarnsleyFern::new(
            &barnsleyfern::REFERENCE_TRANSFORMS,
            &barnsleyfern::REFERENCE_WEIGHTS,
        )
);

animated_chaos_game!(sierpinski: sierpinski::SierpinskiChaosGame::new());

macro_rules! animated_escape_time {
    ($name:ident: $expr:expr) => {
        // Paste is needed to concatenate render_ and the name of the fractal. Rust's own macros
        // don't provide a good way to do this.
        paste::item! {
            #[wasm_bindgen]
            pub fn [<animated_ $name>] (
                canvas: &HtmlCanvasElement, max_iterations: u32, power: u32
            ) -> escapetime::EscapeTimeAnimation {
                console::log_1(&format!("Starting animation {}", stringify!($name)).into());
                let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                escapetime::EscapeTimeAnimation::new(ctx, Box::new($expr))
            }
        }
    };
}

animated_escape_time!(
    burningmandel: BurningMandel::new(u64::from(max_iterations), u64::from(power))
);
animated_escape_time!(burningship: BurningShip::new(u64::from(max_iterations), u64::from(power)));
animated_escape_time!(mandelbrot: Mandelbrot::new(u64::from(max_iterations), u64::from(power)));
animated_escape_time!(roadrunner: RoadRunner::new(u64::from(max_iterations), u64::from(power)));
