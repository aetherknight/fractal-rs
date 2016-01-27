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
use std::fmt;

use fractal::geometry::{Point, Vector};
use fractal::turtle::{Turtle, TurtleProgram, TurtleCollectToNextForwardIterator};

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub trait WindowHandler<'a> {
    /// When the window is resized, we may need to plan to re-render.
    fn window_resized(&mut self);

    /// Render a frame.
    fn render_frame(&mut self,
                    window_size: Size,
                    context: graphics::context::Context,
                    gfx: &mut G2d,
                    program: &'a TurtleProgram,
                    frame_num: u32);
}

/// Renders a TurtleProgram in a PistonWindow.
pub fn run(program: &TurtleProgram, animate: u64) {

    let opengl = OpenGL::V3_2;
    let window: PistonWindow = WindowSettings::new("Fractal", [800, 600])
                                   .opengl(opengl)
                                   .exit_on_esc(true)
                                   .build()
                                   .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

    let mut window_handler: Box<WindowHandler> = match animate {
        0 => Box::new(DoubleBufferedWindowHandler::new()),
        _ => Box::new(DoubleBufferedAnimatedWindowHandler::new(animate)),
    };

    let mut frame_num: u32 = 0;
    let mut old_size: Size = Size {
        width: 0,
        height: 0,
    };
    for e in window {
        e.draw_2d(|context, gfx| {
            let size = e.size();
            // Size doesn't implement PartialEq, so we have to check ourselves.
            if size.width != old_size.width || size.height != old_size.height {
                println!("resized");
                old_size = size;
                window_handler.window_resized();
            }
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

/// WindowHandler that renders an entire turtle program per-frame, and optimizes re-renders
/// by only rendering twice (once for each buffer).
#[derive(Debug)]
struct DoubleBufferedWindowHandler {
    /// Whether we need to re-render for double-buffered frames.
    redraw: [bool; 2],
}

impl DoubleBufferedWindowHandler {
    pub fn new() -> DoubleBufferedWindowHandler {
        DoubleBufferedWindowHandler { redraw: [true; 2] }
    }

    fn turtledraw(program: &TurtleProgram, turtle: &mut Turtle) {
        let init_turtle_steps = program.init_turtle();

        for action in init_turtle_steps {
            turtle.perform(action)
        }

        for action in program.turtle_program_iter() {
            turtle.perform(action)
        }
        turtle.up();
    }
}

impl<'a> WindowHandler<'a> for DoubleBufferedWindowHandler {
    fn window_resized(&mut self) {
        self.redraw[0] = true;
        self.redraw[1] = true;
    }

    fn render_frame(&mut self,
                    window_size: Size,
                    context: graphics::context::Context,
                    gfx: &mut G2d,
                    program: &'a TurtleProgram,
                    frame_num: u32) {
        let redraw = self.redraw[(frame_num % 2) as usize];
        if redraw {
            println!("Redrawing frame {}", frame_num % 2);
            clear(WHITE, gfx);

            let mut state = GlTurtleState::new();
            let mut turtle = GlTurtle::new(&mut state, gfx, window_size, context);
            DoubleBufferedWindowHandler::turtledraw(program, &mut turtle);

            println!("Done redrawing frame");
            self.redraw[(frame_num % 2) as usize] = false;
        }
    }
}

/// Internal state of the turtle.
#[derive(Clone, Debug)]
pub struct GlTurtleState {
    position: Point,
    angle: f64,
    down: bool,
}

impl GlTurtleState {
    pub fn new() -> GlTurtleState {
        GlTurtleState {
            position: Point { x: 0.0, y: 0.0 },
            angle: 0.0,
            down: true,
        }
    }
}

/// An implementation of a Turtle within an OpenGL (rather, a gfx) context.
pub struct GlTurtle<'a, G>
    where G: Graphics + 'a
{
    gfx: &'a mut G,
    window_size: Size,
    context: graphics::context::Context,

    state: &'a mut GlTurtleState,
}

impl<'a, G> GlTurtle<'a, G> where G: Graphics + 'a
{
    pub fn new(state: &'a mut GlTurtleState,
               gfx: &'a mut G,
               window_size: Size,
               context: graphics::context::Context)
               -> GlTurtle<'a, G> {
        GlTurtle {
            gfx: gfx,
            window_size: window_size,
            context: context,
            state: state,
        }
    }
}

impl<'a, G> Turtle for GlTurtle<'a, G> where G: Graphics + 'a
{
    fn forward(&mut self, distance: f64) {
        let old_pos = self.state.position;
        let new_pos = self.state.position.point_at(Vector {
            direction: self.state.angle,
            magnitude: distance,
        });

        if self.state.down {
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

            Line::new(BLACK, 0.5 / linesize).draw([old_pos.x, old_pos.y, new_pos.x, new_pos.y],
                                                  default_draw_state(),
                                                  transform,
                                                  self.gfx);
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

#[derive(Debug,PartialEq)]
enum WhichFrame {
    FirstFrame,
    SecondFrame,
    AllOtherFrames,
}

/// WindowHandler that animates the rendering of the curve.
struct DoubleBufferedAnimatedWindowHandler<'a> {
    /// stored turtle state for each turtle. double-buffered means we need to animate the curve
    /// "twice".
    turtles: [GlTurtleState; 2],
    /// Two iterators.
    iters: [TurtleCollectToNextForwardIterator<'a>; 2],
    forwards_per_frame: u64,
    /// Whether we need to re-render for double-buffered frames.
    first_draw: [bool; 2],
    /// Which frame we are rendering. We need to perform the initial steps for the first frame, and
    /// we need perform the initial steps and do one extra move forward for the second frame (to
    /// stagger the double buffer). The rest of the frames then just move forward.
    which_frame: WhichFrame,
}

impl<'a> fmt::Debug for DoubleBufferedAnimatedWindowHandler<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "DoubleBufferedAnimatedWindowHandler(turtles:{:?}, iters:<iters>, first_draw:{:?}, \
                which_frame:{:?})",
               self.turtles,
               self.first_draw,
               self.which_frame)
    }
}

impl<'a> DoubleBufferedAnimatedWindowHandler<'a> {
    pub fn new(animate: u64) -> DoubleBufferedAnimatedWindowHandler<'a> {
        DoubleBufferedAnimatedWindowHandler {
            turtles: [GlTurtleState::new(), GlTurtleState::new()],
            iters: [TurtleCollectToNextForwardIterator::new_null_iter(),
                    TurtleCollectToNextForwardIterator::new_null_iter()],
            forwards_per_frame: animate,
            first_draw: [true, true],
            which_frame: WhichFrame::FirstFrame,
        }
    }
}

impl<'a> WindowHandler<'a> for DoubleBufferedAnimatedWindowHandler<'a> {
    fn window_resized(&mut self) {
        self.first_draw[0] = true;
        self.first_draw[1] = true;
        self.which_frame = WhichFrame::FirstFrame;
        self.turtles[0] = GlTurtleState::new();
        self.turtles[1] = GlTurtleState::new();
    }

    fn render_frame(&mut self,
                    window_size: Size,
                    context: graphics::context::Context,
                    gfx: &mut G2d,
                    program: &'a TurtleProgram,
                    frame_num: u32) {
        let bufnum = (frame_num % 2) as usize;

        match self.which_frame {
            WhichFrame::FirstFrame => {
                // gfx can only be &mut borrowed by one thing at a time. If we loan it to the
                // turtle and also use it elsewhere, this would trigger the static analysis.
                // This could be worked around by placing gfx into a RefCell.
                clear(WHITE, gfx);
                let mut turtle = GlTurtle::new(&mut self.turtles[bufnum],
                                               gfx,
                                               window_size,
                                               context);
                for action in program.init_turtle() {
                    turtle.perform(action)
                }
                self.iters[bufnum] = program.turtle_program_iter().collect_to_next_forward();
                self.which_frame = WhichFrame::SecondFrame;
            }
            WhichFrame::SecondFrame => {
                clear(WHITE, gfx);
                let mut turtle = GlTurtle::new(&mut self.turtles[bufnum],
                                               gfx,
                                               window_size,
                                               context);
                for action in program.init_turtle() {
                    turtle.perform(action)
                }
                self.iters[bufnum] = program.turtle_program_iter().collect_to_next_forward();
                // if we are the second frame, then we need to stagger our buffer from
                // the first buffer.
                let one_move = self.iters[bufnum].next();
                if !one_move.is_none() {
                    for action in one_move.unwrap() {
                        turtle.perform(action)
                    }
                }
                self.which_frame = WhichFrame::AllOtherFrames;
            }
            _ => {
                let mut turtle = GlTurtle::new(&mut self.turtles[bufnum],
                                               gfx,
                                               window_size,
                                               context);
                for _ in 0..self.forwards_per_frame {
                    // Make 2 moves per frame since we are double buffered.
                    DoubleBufferedAnimatedWindowHandler::draw_one_move(&mut turtle,
                                                                       &mut self.iters[bufnum]);
                    DoubleBufferedAnimatedWindowHandler::draw_one_move(&mut turtle,
                                                                       &mut self.iters[bufnum]);
                }
            }
        }
    }
}

impl<'a> DoubleBufferedAnimatedWindowHandler<'a> {
    fn draw_one_move<G>(turtle: &mut GlTurtle<G>,
                        program_iter: &mut TurtleCollectToNextForwardIterator)
        where G: Graphics
    {
        let one_move = program_iter.next();
        if !one_move.is_none() {
            for action in one_move.unwrap() {
                turtle.perform(action)
            }
        }
    }
}
