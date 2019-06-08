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
//
use fractal_lib::geometry::{Point, Vector, ViewAreaTransformer};
use fractal_lib::turtle::{Turtle, TurtleState};
use web_sys::{console, CanvasRenderingContext2d};

/// A turtle that can draw to an HTML Canvas.
pub struct CanvasTurtle {
    state: TurtleState,
    ctx: CanvasRenderingContext2d,
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
            let screen_width = self.ctx.canvas().unwrap().width() as f64;
            let screen_height = self.ctx.canvas().unwrap().height() as f64;

            let turtle_vat = ViewAreaTransformer::new(
                [screen_width, screen_height],
                Point { x: -0.5, y: -0.75 },
                Point { x: 1.5, y: 0.75 },
            );

            let old_coords = turtle_vat.map_point_to_pixel(old_pos);
            let new_coords = turtle_vat.map_point_to_pixel(new_pos);
            // console::log_3(&"Line to".into(), &new_pos.x.into(), &new_pos.y.into());
            // console::log_3(
            //     &"old coords".into(),
            //     &old_coords[0].into(),
            //     &old_coords[1].into(),
            // );
            // console::log_3(
            //     &"new coords".into(),
            //     &new_coords[0].into(),
            //     &new_coords[1].into(),
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
