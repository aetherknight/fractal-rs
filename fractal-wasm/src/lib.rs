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
use fractal_lib::curves::terdragon;
use fractal_lib::lindenmayer::LindenmayerSystemTurtleProgram;
use fractal_lib::turtle::{Turtle, TurtleProgram, TurtleState};
use js_sys::Array;
use paste;
use vecmath;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

mod turtle;

fn render_turtle(canvas: &HtmlCanvasElement, program: &dyn TurtleProgram) {
    // Extract the rendering context from the canvas.
    let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    // set up a turtle to do the work of drawing
    let mut turtle = turtle::CanvasTurtle::new(TurtleState::new(), &ctx);

    // run the init_turtle steps
    let init_turtle_steps = program.init_turtle();
    for action in init_turtle_steps {
        turtle.perform(action)
    }

    // run the program
    for action in program.turtle_program_iter() {
        turtle.perform(action)
    }
    turtle.up();
}

macro_rules! render_turtle {
    ($name:ident: $expr:expr) => {
        // Paste is needed to concatenate render_ and the name of the fractal. Rust's own macros
        // don't provide a good way to do this.
        paste::item! {
            // iteration needs to be a u32 for now.. As of 2019/04/28, Firefox 66.0.2 doesn't
            // support BigUint64Array/bigints.
            #[wasm_bindgen]
            pub fn [<render_ $name>] (
                canvas: &HtmlCanvasElement,
                iteration: u32
            ) -> Result<(), JsValue> {
                console::log_1(&format!("Rendering {}", stringify!($name)).into());
                let program = $expr;

                render_turtle(&canvas, &program);

                console::log_1(&"Done".into());
                Ok(())
            }
        }
    };
}

render_turtle!(dragon: dragon::DragonFractal::new(iteration as u64));
render_turtle!(
    terdragon: LindenmayerSystemTurtleProgram::new(
        terdragon::TerdragonFractal::new(iteration as u64)
    )
);
render_turtle!(
    cesaro: LindenmayerSystemTurtleProgram::new(cesaro::CesaroFractal::new(iteration as u64))
);
render_turtle!(
    cesarotri: LindenmayerSystemTurtleProgram::new(cesarotri::CesaroTriFractal::new(
        iteration as u64)
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
