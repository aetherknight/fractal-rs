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

use fractal_lib::curves::dragon;
use fractal_lib::geometry::{Point, Vector};
use fractal_lib::turtle::{Turtle, TurtleProgram, TurtleState};
use js_sys::Array;
use vecmath;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

pub fn turtle_to_screen_transform(screen_width: f64, screen_height: f64) -> [[f64; 3]; 2] {
    let startx = screen_width / 4f64;
    let starty = screen_height / 2f64;
    let endx = 3f64 * screen_width / 4f64;
    // let endy = (screen_height / 2) as f64;

    let linesize = (startx - endx).abs() as f64;

    // identity transform.
    let mut transform: vecmath::Matrix2x3<f64> = [[1f64, 0f64, 0f64], [0f64, 1f64, 0f64]];
    // translate to startx and starty
    transform = vecmath::row_mat2x3_mul(transform, [[1f64, 0f64, startx], [0f64, 1f64, starty]]);
    // zoom in by the linesize
    transform =
        vecmath::row_mat2x3_mul(transform, [[linesize, 0f64, 0f64], [0f64, linesize, 0f64]]);
    // flip vertical
    transform = vecmath::row_mat2x3_mul(transform, [[1f64, 0f64, 0f64], [0f64, -1f64, 0f64]]);
    transform
}

/// A turtle that can draw to an HTML Canvas.
pub struct CanvasTurtle<'a> {
    state: TurtleState,
    ctx: &'a CanvasRenderingContext2d,
}

impl<'a> CanvasTurtle<'a> {
    pub fn new(state: TurtleState, ctx: &'a CanvasRenderingContext2d) -> CanvasTurtle {
        CanvasTurtle { state, ctx }
    }
}

impl<'a> Turtle for CanvasTurtle<'a> {
    fn forward(&mut self, distance: f64) {
        let old_pos = self.state.position;
        let new_pos = self.state.position.point_at(Vector {
            direction: self.state.angle,
            magnitude: distance,
        });

        if self.state.down {
            let screen_width = self.ctx.canvas().unwrap().width() as f64;
            let screen_height = self.ctx.canvas().unwrap().height() as f64;
            let transform = turtle_to_screen_transform(screen_width, screen_height);

            console::log_3(
                &transform[0][0].into(),
                &transform[0][1].into(),
                &transform[0][2].into(),
            );
            console::log_3(
                &transform[1][0].into(),
                &transform[1][1].into(),
                &transform[1][2].into(),
            );

            // // [[ a, c, e ]]
            // // [[ b, d, f ]]
            // // [[ 0, 0, 1 ]]
            // //
            // // a - horizontal scaling (1 does nothing)
            // // b - vertical skewing
            // // c - horizontal skewing
            // // d - vertical scaling
            // // e - horizontal translation
            // // f - vertical translation
            // self.ctx
            //     .set_transform(
            //         transform[0][0],
            //         transform[1][0],
            //         transform[0][1],
            //         transform[1][1],
            //         transform[0][2],
            //         transform[1][2],
            //     )
            //     .unwrap();
            //
            let old_coords = vecmath::row_mat2x3_transform_pos2(transform, [old_pos.x, old_pos.y]);
            let new_coords = vecmath::row_mat2x3_transform_pos2(transform, [new_pos.x, new_pos.y]);
            console::log_3(
                &"old coords".into(),
                &old_coords[0].into(),
                &old_coords[1].into(),
            );
            console::log_3(
                &"new coords".into(),
                &new_coords[0].into(),
                &new_coords[1].into(),
            );

            // self.ctx.set_line_width(0.5f64 / linesize);
            self.ctx.set_line_width(1.0f64);
            self.ctx.begin_path();
            // self.ctx.move_to(old_pos.x, old_pos.y);
            // self.ctx.line_to(new_pos.x, new_pos.y);
            self.ctx.move_to(old_coords[0], old_coords[1]);
            self.ctx.line_to(new_coords[0], new_coords[1]);
            console::log_3(&"Line to".into(), &new_pos.x.into(), &new_pos.y.into());
            self.ctx.stroke();
        }

        self.state.position = new_pos;
    }

    fn set_pos(&mut self, new_pos: Point) {
        self.state.position = new_pos;
    }

    fn set_rad(&mut self, new_rad: f64) {
        self.state.angle = new_rad;
    }

    fn turn_rad(&mut self, radians: f64) {
        use std::f64::consts::PI;
        self.state.angle = (self.state.angle + radians) % (2.0 * PI);
    }

    fn down(&mut self) {
        self.state.down = true;
    }

    fn up(&mut self) {
        self.state.down = false;
    }
}

#[wasm_bindgen]
pub fn render(canvas: &HtmlCanvasElement) -> Result<(), JsValue> {
    console::log_1(&"Begin render".into());
    console::log_1(&"Extracting the rendering context".into());
    // Extract the rendering context from the canvas.
    let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    // get the program we want to run
    console::log_1(&"Create the turtle program".into());
    let program = dragon::DragonFractal::new(3);

    // set up a turtle to do the work of drawing
    console::log_1(&"Create the turtle".into());
    let mut turtle = CanvasTurtle::new(TurtleState::new(), &ctx);

    // run the init_turtle steps
    console::log_1(&"Run the turtle".into());
    let init_turtle_steps = program.init_turtle();
    for action in init_turtle_steps {
        turtle.perform(action)
    }

    // run the program
    for action in program.turtle_program_iter() {
        turtle.perform(action)
    }
    turtle.up();
    console::log_1(&"Done".into());

    Ok(())
}

#[wasm_bindgen]
pub fn screen_to_turtle(canvas: &HtmlCanvasElement, x: f64, y: f64) -> Array {
    let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();
    let screen_width = ctx.canvas().unwrap().width() as f64;
    let screen_height = ctx.canvas().unwrap().height() as f64;
    let inv_transform =
        vecmath::mat2x3_inv(turtle_to_screen_transform(screen_width, screen_height));
    let coords = vecmath::row_mat2x3_transform_pos2(inv_transform, [x, y]);
    Array::of2(&coords[0].into(), &coords[1].into())
}
