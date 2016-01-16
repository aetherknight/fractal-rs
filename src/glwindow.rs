// Copyright (c) 2015 William (B.J.) Snow Orvis
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

use graphics;
use opengl_graphics::GlGraphics;
use piston_window::*;

use common::{Turtle, TurtleApp, Point, Vector};

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct WindowHandler {
    opengl: OpenGL,
    window: PistonWindow,
    redraw: bool,
}

impl WindowHandler {
    pub fn new() -> WindowHandler {
        let opengl = OpenGL::V3_2;

        let window: PistonWindow = WindowSettings::new("Fractal", [800, 600])
                                       .opengl(opengl)
                                       .exit_on_esc(true)
                                       .build()
                                       .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

        WindowHandler {
            opengl: opengl,
            window: window,
            redraw: true,
        }
    }

    pub fn run(mut self, app: &TurtleApp) {
        // event loop
        for e in self.window {
            match e.event {
                Some(Event::Render(r)) => {
                    self.redraw = false;
                    let gl = &mut GlGraphics::new(self.opengl);

                    gl.draw(r.viewport(), |context, gl2| {
                        use graphics::*;
                        clear(WHITE, gl2);

                        let turtle = &mut GlTurtle::new(gl2, r, context);
                        app.draw(turtle);

                    });
                }
                // Some(Event::Update(u)) => {
                //     self.app.update(&u);
                // }
                // Some(Event::Input(i)) => {}
                _ => {}
            }
        }
    }
}

/// An implementation of a Turtle within an OpenGL context.
pub struct GlTurtle<'a> {
    gl: &'a mut GlGraphics,
    args: RenderArgs,
    context: graphics::context::Context,

    position: Point,
    angle: f64,
    down: bool,
}

impl<'a> GlTurtle<'a> {
    pub fn new(gl: &'a mut GlGraphics,
               args: RenderArgs,
               context: graphics::context::Context)
               -> GlTurtle {
        GlTurtle {
            gl: gl,
            args: args,
            context: context,
            position: Point { x: 0.0, y: 0.0 },
            angle: 0.0,
            down: true,
        }
    }
}

impl<'a> Turtle for GlTurtle<'a> {
    fn forward(&mut self, distance: f64) {
        use graphics::*;

        let old_pos = self.position;
        let new_pos = self.position.point_at(Vector {
            direction: self.angle,
            magnitude: distance,
        });

        if self.down {
            // let rotation = 0.0;
            let startx = (self.args.width / 4) as f64;
            let starty = (self.args.height / 2) as f64;
            let endx = (3 * self.args.width / 4) as f64;
            // let endy = (self.args.height / 2) as f64;

            let linesize = (startx - endx).abs() as f64;

            // println!("{}, {}", self.args.width, self.args.height);

            let transform = self.context
                                .transform
                                .trans(startx, starty)
                                .zoom(linesize)
                                .flip_v()
                                .trans(0.0, 0.0);

            // Line::new(BLACK, 1.0).draw([old_pos.x*linesize, old_pos.y*linesize,
            // new_pos.x*linesize, new_pos.y*linesize],
            Line::new(BLACK, 0.5 / linesize).draw([old_pos.x, old_pos.y, new_pos.x, new_pos.y],
                                                  default_draw_state(),
                                                  transform,
                                                  self.gl);
        }

        self.position = new_pos;
    }

    fn set_pos(&mut self, new_pos: Point) {
        self.position = new_pos;
    }

    fn set_rad(&mut self, new_rad: f64) {
        self.angle = new_rad;
    }

    fn turn_rad(&mut self, radians: f64) {
        use std::f64::consts::PI;
        self.angle = (self.angle + radians) % (2.0 * PI);
    }

    fn down(&mut self) {
        self.down = true;
    }

    fn up(&mut self) {
        self.down = false;
    }
}
