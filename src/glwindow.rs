use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };
use graphics;

use common::{Turtle, TurtleApp, Point, Vector};

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub struct WindowHandler {
    opengl: OpenGL,
    window: Window,
    redraw: bool,
}

impl WindowHandler {
    pub fn new() -> WindowHandler {
        let opengl = OpenGL::V3_2;

        let window: Window = WindowSettings::new("Fractal", [800, 600])
            .opengl(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap_or_else(|e| { panic!("Failed to build Window: {}", e) });

        WindowHandler { opengl: opengl, window: window, redraw: true }
    }

    pub fn run(mut self, app: &TurtleApp) {
        // event loop
        for event in self.window.events() {
            if let Some(r) = event.render_args() {
                self.redraw = false;
                let gl = &mut GlGraphics::new(self.opengl);

                gl.draw(r.viewport(), |context, gl2| {
                    use graphics::*;
                    clear(WHITE, gl2);

                    let turtle = &mut GlTurtle::new(gl2, r, context);
                    app.draw(turtle);

                });
            }
            // if let Some(u) = event.update_args() {
            //     self.app.update(&u);
            // }
            // if let Some(i) = event.input
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
    pub fn new(gl: &'a mut GlGraphics, args: RenderArgs, context: graphics::context::Context) -> GlTurtle {
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
        let new_pos = self.position.point_at(Vector { direction: self.angle, magnitude: distance });

        if self.down {
            // let rotation = 0.0;
            let startx = (self.args.width / 4) as f64;
            let starty = (self.args.height / 2) as f64;
            let endx = (3 * self.args.width / 4) as f64;
            // let endy = (self.args.height / 2) as f64;

            let linesize = (startx - endx).abs() as f64;

            // println!("{}, {}", self.args.width, self.args.height);

            let transform = self.context.transform
                .trans(startx, starty)
                .zoom(linesize)
                .flip_v()
                // .rot_rad(rotation)
                .trans(0.0, 0.0);

            // Line::new(BLACK, 1.0).draw([old_pos.x*linesize, old_pos.y*linesize, new_pos.x*linesize, new_pos.y*linesize],
            Line::new(BLACK, 0.5/linesize).draw([old_pos.x, old_pos.y, new_pos.x, new_pos.y],
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
        self.angle = (self.angle + radians) % (2.0*PI);
    }

    fn down(&mut self) {
        self.down = true;
    }

    fn up(&mut self) {
        self.down = false;
    }
}

// pub struct FractalApp;

// impl FractalApp {
//     fn render(&mut self, args: &RenderArgs) {
//         use graphics::*;

//         let df = DragonFractal::new(1);

//         self.gl.draw(args.viewport(), |context, gl| {
//             // clear the screen
//             clear(WHITE, gl);

//             // df.draw(self);
//             let rotation = 0.0;
//             let (x, y) = ((args.width / 2) as f64, (args.height / 2) as f64);

//             let transform = context.transform
//                                    .trans(x, y)
//                                    .rot_rad(rotation)
//                                    .trans(0.0, 0.0);

//             graphics::Line::new(BLACK, 1.0).draw([0.0, 0.0, 25.0, 25.0],
//                                                  graphics::default_draw_state(),
//                                                  transform,
//                                                  gl);

//         });
//     }

//     fn update(&mut self, args: &UpdateArgs) {}

//     fn draw_dimensions(&mut self, args: &RenderArgs) {}
// }

// impl LineDrawer for FractalApp {
//     fn draw_line(&mut self, from: Point, to: Point) {}
// }
