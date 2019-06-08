// Copyright (c) 2019 William (B.J.) Snow Orvis
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

use fractal_lib::chaosgame::barnsleyfern;
use fractal_lib::chaosgame::sierpinski;
use fractal_lib::chaosgame::ChaosGameMoveIterator;
use fractal_lib::curves::cesaro;
use fractal_lib::curves::cesarotri;
use fractal_lib::curves::dragon;
use fractal_lib::curves::kochcurve;
use fractal_lib::curves::levyccurve;
use fractal_lib::curves::terdragon;
use fractal_lib::geometry;
use fractal_lib::lindenmayer::LindenmayerSystemTurtleProgram;
use fractal_lib::turtle::{Turtle, TurtleCollectToNextForwardIterator, TurtleProgram, TurtleState};
use js_sys::Array;
use paste;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

mod turtle;

/// Represents everything needed to render a turtle a piece at a time to a canvas.
///
/// It holds onto a turtle program, which is then used to eventually initialize an iterator over
/// that program.
#[wasm_bindgen]
pub struct TurtleAnimation {
    turtle: turtle::CanvasTurtle,
    iter: TurtleCollectToNextForwardIterator,
}

impl TurtleAnimation {
    /// Build a TurtleAnimation from a canvas element and a boxed turtle program.
    ///
    /// The TurtleProgram is copied and boxed (via its `turtle_program_iter`) to to avoid
    /// TurtleAnimation being generic.
    pub fn new(ctx: CanvasRenderingContext2d, program: &dyn TurtleProgram) -> TurtleAnimation {
        let mut turtle = turtle::CanvasTurtle::new(TurtleState::new(), ctx);

        let init_turtle_steps = program.init_turtle();
        for action in init_turtle_steps {
            turtle.perform(action)
        }

        let iter = program.turtle_program_iter().collect_to_next_forward();

        TurtleAnimation { turtle, iter }
    }
}

#[wasm_bindgen]
impl TurtleAnimation {
    /// Returns true if there are more moves to make, and false if it can no longer perform a move.
    pub fn draw_one_frame(&mut self) -> bool {
        if let Some(one_move) = self.iter.next() {
            console::log_1(&"Rendering one move".into());
            for action in one_move {
                self.turtle.perform(action);
            }
            true
        } else {
            console::log_1(&"No more moves".into());
            self.turtle.up();
            false
        }
    }
}

/// Macro that generates a function for constructing a TurtleAnimation for a particular kind of
/// turtle-based curve.
///
/// It takes a name identifier, a colon, and then expression that should evaluate to a
/// TurtleProgram. The expression may use `iteration` in order to configure the TurtleProgram.
///
/// For example:
/// ```rust,ignore
/// animated_turtle!(dragon: dragon::DragonFractal::new(iteration as u64));
/// ```
///
/// Will create a function with signature:
///
/// ```rust, ignore
/// #[wasm_bindgen]
/// pub fn animated_dragon(canvas: &HtmlCanvaselement, iteration: u32) -> TurtleAnimation;
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
            ) -> TurtleAnimation {
                console::log_1(&format!("Starting animation {}", stringify!($name)).into());
                let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                let program = $expr;
                ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                TurtleAnimation::new(ctx, &program)
            }
        }
    };
}

animated_turtle!(
    cesaro: LindenmayerSystemTurtleProgram::new(cesaro::CesaroFractal::new(iteration as u64))
);
animated_turtle!(
    cesarotri: LindenmayerSystemTurtleProgram::new(cesarotri::CesaroTriFractal::new(
        iteration as u64)
    )
);
animated_turtle!(dragon: dragon::DragonFractal::new(iteration as u64));
animated_turtle!(
    kochcurve: LindenmayerSystemTurtleProgram::new(kochcurve::KochCurve::new(iteration as u64))
);
animated_turtle!(
    levyccurve: LindenmayerSystemTurtleProgram::new(levyccurve::LevyCCurve::new(iteration as u64))
);
animated_turtle!(
    terdragon: LindenmayerSystemTurtleProgram::new(
        terdragon::TerdragonFractal::new(iteration as u64)
    )
);

/// Translates a pixel-coordinate on the Canvas into the coordinate system used by the turtle
/// curves. See turtle::turtle_vat for more information on the coordinate system for turtle curves.
#[wasm_bindgen]
pub fn screen_to_turtle(canvas: &HtmlCanvasElement, x: f64, y: f64) -> Array {
    let pos_point = turtle::turtle_vat(canvas).map_pixel_to_point([x,y]);
    Array::of2(&pos_point.x.into(), &pos_point.y.into())
}

/// Constructs a ViewAreaTransformer for converting between a canvas pixel-coordinate and the
/// coordinate system used by Chaos Games.
///
/// The ChaosGame fractals expect a view area that covers between -1.0 and 1.0 on the X axis, as
/// well as -1.0 to 1.0 on the Y axis, with positive values going up and right.
fn chaos_game_vat(canvas: &HtmlCanvasElement) -> geometry::ViewAreaTransformer {
    let screen_width = canvas.width() as f64;
    let screen_height = canvas.height() as f64;

    geometry::ViewAreaTransformer::new(
        [screen_width, screen_height],
        geometry::Point { x: -1.0, y: -1.0 },
        geometry::Point { x: 1.0, y: 1.0 },
    )
}

/// Translates a pixel-coordinate on the Canvas into the coordinate system used by a chaos game.
/// See chaos_game_vat for more information on the coordinate system for chaos games.
#[wasm_bindgen]
pub fn screen_to_chaos_game(canvas: &HtmlCanvasElement, x: f64, y: f64) -> Array {
    let pos_point = chaos_game_vat(canvas).map_pixel_to_point([x, y]);
    Array::of2(&pos_point.x.into(), &pos_point.y.into())
}

/// Represents everything needed to render a chaos game fractal as an animation.
#[wasm_bindgen]
pub struct ChaosGameAnimation {
    ctx: CanvasRenderingContext2d,
    iter: Box<dyn ChaosGameMoveIterator>,
}

impl ChaosGameAnimation {
    pub fn new(
        ctx: CanvasRenderingContext2d,
        chaos_game: Box<dyn ChaosGameMoveIterator>,
    ) -> ChaosGameAnimation {
        ChaosGameAnimation {
            ctx,
            iter: chaos_game,
        }
    }

    fn draw_point(&self, point: geometry::Point) {
        let canvas = self.ctx.canvas().unwrap();
        let pixel_pos = chaos_game_vat(&canvas).map_point_to_pixel(point);
        // console::log_1(&format!("pixels: {}, {}", pixel_pos[0], pixel_pos[1]).into());
        self.ctx.set_fill_style(&"black".into());
        self.ctx.fill_rect(pixel_pos[0], pixel_pos[1], 1.0, 1.0);
        // self.ctx.stroke();
    }
}

#[wasm_bindgen]
impl ChaosGameAnimation {
    /// Always returns true
    pub fn draw_one_frame(&mut self) -> bool {
        if let Some(next_point) = self.iter.next() {
            // console::log_1(&format!("{}", next_point).into());
            self.draw_point(next_point);
            true
        } else {
            console::log_1(&"No more points".into());
            false
        }
    }
}

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
            // iteration needs to be a u32 for now.. As of 2019/04/28, Firefox 66.0.2 doesn't
            // support BigUint64Array/bigints.
            #[wasm_bindgen]
            pub fn [<animated_ $name>] (canvas: &HtmlCanvasElement) -> ChaosGameAnimation {
                console::log_1(&format!("Starting animation {}", stringify!($name)).into());
                let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                ChaosGameAnimation::new(ctx, Box::new($expr))
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
