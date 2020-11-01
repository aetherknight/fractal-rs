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
//
use super::FractalAnimation;
use fractal_lib::geometry::{Point, Vector, ViewAreaTransformer};
use fractal_lib::turtle::{Turtle, TurtleCollectToNextForwardIterator, TurtleProgram, TurtleState};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Constructs a ViewAreaTransformer for converting between a canvas pixel-coordinate and the
/// coordinate system used by Turtle curves.
///
/// The Turtle curves expect a view area that has positive values going up and to the right, and
/// that center on both (0.0, 0.0) and (1.0, 0.0). to achieve this, the we need a view area from
/// -0.5 to 1.5 on the X axis, and -0.75 and 0.75 on the Y axis.
fn turtle_vat(canvas: &HtmlCanvasElement) -> ViewAreaTransformer {
    let screen_width = f64::from(canvas.width());
    let screen_height = f64::from(canvas.height());

    ViewAreaTransformer::new(
        [screen_width, screen_height],
        Point { x: -0.5, y: -0.75 },
        Point { x: 1.5, y: 0.75 },
    )
}

/// A turtle that can draw to an HTML Canvas.
struct CanvasTurtle {
    state: TurtleState,
    pub ctx: CanvasRenderingContext2d,
}

impl CanvasTurtle {
    pub fn new(state: TurtleState, ctx: CanvasRenderingContext2d) -> CanvasTurtle {
        CanvasTurtle { state, ctx }
    }
}

impl Turtle for CanvasTurtle {
    fn forward(&mut self, distance: f64) {
        let old_pos = self.state.position;
        let new_pos = self.state.position.point_at(Vector {
            direction: self.state.angle,
            magnitude: distance,
        });

        if self.state.down {
            let turtle_vat = turtle_vat(&self.ctx.canvas().unwrap());

            let old_coords = turtle_vat.map_point_to_pixel(old_pos);
            let new_coords = turtle_vat.map_point_to_pixel(new_pos);
            // log::debug!("Line to {}, {}", new_pos.x, new_pos.y);
            // log::debug!(
            //     "old coords",
            //     old_coords[0],
            //     old_coords[1],
            // );
            // log::debug!(
            //     "new coords",
            //     &new_coords[0],
            //     &new_coords[1],
            // );

            self.ctx.set_line_width(1.0f64);
            self.ctx.begin_path();
            self.ctx.move_to(old_coords[0], old_coords[1]);
            self.ctx.line_to(new_coords[0], new_coords[1]);
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

/// Represents everything needed to render a turtle a piece at a time to a canvas.
///
/// It holds onto a turtle program, which is then used to eventually initialize an iterator over
/// that program.
pub struct TurtleAnimation {
    turtle: CanvasTurtle,
    iter: TurtleCollectToNextForwardIterator,
}

impl TurtleAnimation {
    /// Build a TurtleAnimation from a canvas element and a boxed turtle program.
    ///
    /// The TurtleProgram is copied and boxed (via its `turtle_program_iter`) to to avoid
    /// TurtleAnimation being generic.
    pub fn new(ctx: CanvasRenderingContext2d, program: &dyn TurtleProgram) -> TurtleAnimation {
        let mut turtle = CanvasTurtle::new(TurtleState::new(), ctx);

        let init_turtle_steps = program.init_turtle();
        for action in init_turtle_steps {
            turtle.perform(action)
        }

        let iter = program.turtle_program_iter().collect_to_next_forward();

        TurtleAnimation { turtle, iter }
    }
}

impl FractalAnimation for TurtleAnimation {
    /// Returns true if there are more moves to make, and false if it can no longer perform a move.
    fn draw_one_frame(&mut self) -> bool {
        if let Some(one_move) = self.iter.next() {
            log::debug!("Rendering one move");
            for action in one_move {
                self.turtle.perform(action);
            }
            true
        } else {
            log::debug!("No more moves");
            self.turtle.up();
            false
        }
    }

    /// Translates a pixel-coordinate on the Canvas into the coordinate system used by the turtle
    /// curves.
    ///
    /// See turtle::turtle_vat for more information on the coordinate system for turtle curves.
    fn pixel_to_coordinate(&self, x: f64, y: f64) -> [f64; 2] {
        let canvas = self.turtle.ctx.canvas().unwrap();
        let pos_point = turtle_vat(&canvas).map_pixel_to_point([x, y]);
        pos_point.into()
    }

    fn zoom(&mut self, _x1: f64, _y1: f64, _x2: f64, _y2: f64) -> bool {
        false
    }
}
