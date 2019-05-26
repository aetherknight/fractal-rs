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

use fractal_lib::curves::cesaro;
use fractal_lib::curves::cesarotri;
use fractal_lib::curves::dragon;
use fractal_lib::curves::kochcurve;
use fractal_lib::curves::levyccurve;
use fractal_lib::curves::terdragon;
use fractal_lib::lindenmayer::LindenmayerSystemTurtleProgram;
use fractal_lib::turtle::{Turtle, TurtleCollectToNextForwardIterator, TurtleProgram, TurtleState};
use js_sys::Array;
use paste;
use vecmath;
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
    /// The TurtleProgram is boxed to to avoid generics here.
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
    pub fn draw_one_move(&mut self) -> bool {
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

macro_rules! animated_turtle {
    ($name:ident: $expr:expr) => {
        // Paste is needed to concatenate render_ and the name of the fractal. Rust's own macros
        // don't provide a good way to do this.
        paste::item! {
            // iteration needs to be a u32 for now.. As of 2019/04/28, Firefox 66.0.2 doesn't
            // support BigUint64Array/bigints.
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

#[wasm_bindgen]
pub fn screen_to_turtle(canvas: &HtmlCanvasElement, x: f64, y: f64) -> Array {
    let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let screen_width = ctx.canvas().unwrap().width() as f64;
    let screen_height = ctx.canvas().unwrap().height() as f64;
    let inv_transform = vecmath::mat2x3_inv(turtle::turtle_to_screen_transform(
        screen_width,
        screen_height,
    ));
    let coords = vecmath::row_mat2x3_transform_pos2(inv_transform, [x, y]);
    Array::of2(&coords[0].into(), &coords[1].into())
}
