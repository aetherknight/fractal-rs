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

use graphics;
use piston_window::*;

use geometry::{Point, Vector};
use turtle::{Turtle, TurtleProgram, TurtleStep};

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

#[derive(Debug)]
struct WindowHandler {
    redraw: [bool; 2],
}

/// Renders a TurtleProgram in a PistonWindow.
pub fn run(program: &TurtleProgram) {

    let opengl = OpenGL::V3_2;
    let window: PistonWindow = WindowSettings::new("Fractal", [800, 600])
                                   .opengl(opengl)
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

    let mut window_handler = WindowHandler { redraw: [true; 2] };

    let mut frame_num: u32 = 0;
    // event loop
    for e in window {
        e.draw_2d(|context, gfx| {
            let size = e.size();
            frame_num += 1;
            println!("Render frame {}, window: {:?}", frame_num, size);
            window_handler.render_frame(size, context, gfx, program, frame_num);
        });
        //     // Some(Event::Input(i)) => {
        //     //     match i {
        //     //         Input::Press(Button::Keyboard(k)) => {
        //     //             match k {
        //     //                 Key::Up => {}
        //     //                 Key::Down => {}
        //     //                 _ => {}
        //     //             }
        //     //         }
        //     //         _ => {}
        //     //     }
        //     // }
        //     _ => {}
        // }
    }
}

impl WindowHandler {
    /// TODO: resizing the window does should trigger a re-render.
    pub fn render_frame<G, T>(&mut self,
                              window_size: Size,
                              context: graphics::context::Context,
                              gfx: &mut G,
                              program: &TurtleProgram,
                              frame_num: u32)
        where T: ImageSize,
              G: Graphics<Texture = T>
    {
        use graphics::*;
        let redraw = self.redraw[(frame_num % 2) as usize];
        if redraw {
            println!("Redrawing frame {}", frame_num % 2);
            clear(WHITE, gfx);

            let mut turtle = GlTurtle::new(gfx, window_size, context);
            WindowHandler::turtledraw(program, &mut turtle);

            println!("Done redrawing frame");
            self.redraw[(frame_num % 2) as usize] = false;
        }
    }

    fn turtledraw(program: &TurtleProgram, turtle: &mut Turtle) {
        program.init_turtle(turtle);

        for action in program.turtle_program_iter() {
            match action {
                TurtleStep::Forward(dist) => turtle.forward(dist),
                TurtleStep::TurnRad(angle) => turtle.turn_rad(angle),
                _ => {}
            }
        }
        turtle.up();
    }
}

/// An implementation of a Turtle within an OpenGL (rather, a gfx) context.
pub struct GlTurtle<'a, G, T>
    where T: ImageSize,
          G: Graphics<Texture = T> + 'a
{
    gfx: &'a mut G,
    window_size: Size,
    context: graphics::context::Context,

    position: Point,
    angle: f64,
    down: bool,
}

impl<'a, G, T> GlTurtle<'a, G, T>
    where T: ImageSize,
          G: Graphics<Texture = T> + 'a
{
    pub fn new(gfx: &'a mut G,
               window_size: Size,
               context: graphics::context::Context)
               -> GlTurtle<'a, G, T> {
        GlTurtle {
            gfx: gfx,
            window_size: window_size,
            context: context,
            position: Point { x: 0.0, y: 0.0 },
            angle: 0.0,
            down: true,
        }
    }
}

impl<'a, G, T> Turtle for GlTurtle<'a, G, T>
    where T: ImageSize,
          G: Graphics<Texture = T> + 'a
{
    fn forward(&mut self, distance: f64) {
        use graphics::*;

        let old_pos = self.position;
        let new_pos = self.position.point_at(Vector {
            direction: self.angle,
            magnitude: distance,
        });

        if self.down {
            let screen_width = self.window_size.width;
            let screen_height = self.window_size.height;

            let startx = (screen_width / 4) as f64;
            let starty = (screen_height / 2) as f64;
            let endx = (3 * screen_width / 4) as f64;
            // let endy = (screen_height / 2) as f64;

            let linesize = (startx - endx).abs() as f64;

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
                                                  self.gfx);
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
