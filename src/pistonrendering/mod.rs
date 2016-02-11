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

//! A piston window-based renderer and event loop.

pub mod chaosgame;
pub mod escapetime;
pub mod turtle;

use graphics;
use gfx_device_gl;
use piston_window::*;

pub use graphics::math::Vec2d;

pub const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const GREY:  [f32; 4] = [0.5, 0.5, 0.5, 1.0];

/// State machine for WindowHandlers that want to animate across the double buffered frames.
#[derive(Debug,PartialEq)]
pub enum WhichFrame {
    FirstFrame,
    SecondFrame,
    AllOtherFrames,
}

/// Information about the viewport and graphical backend needed by WindowHandler::render_frame.
pub struct RenderContext<'a, 'b: 'a> {
    /// Graphics context, describing the viewport, base transform, etc.
    pub context: graphics::context::Context,
    /// Graphics backend
    pub gfx: &'a mut G2d<'b>,
    /// graphics backend factory
    pub factory: &'a mut gfx_device_gl::Factory,
}

/// An object that can render frames of a drawing/animation/program/game.
pub trait WindowHandler {
    /// When the window is resized, we may need to plan to re-render.
    fn window_resized(&mut self, new_size: Vec2d);

    /// Render a frame.
    fn render_frame(&mut self,
                    context: &mut RenderContext,
                    frame_num: u32);

    /// Optional: used to indicate that the user selected an area of the window to zoom in on.
    fn zoom(&mut self, rect: [ Vec2d; 2]) {
        println!("Selected: {:?}, {:?}", rect[0], rect[1]);
    }

    /// Optional: used to indicate that the user wants to revert to the default view.
    fn reset_view(&mut self) {
        println!("Reset zoom");
    }
}

/// Runs a WindowHandler in a PistonWindow.
pub fn run(window_handler: &mut WindowHandler) {
    println!("Use the mouse to select an area to zoom in on");
    println!("Press backspace to reset the view back to the initial view");
    println!("Press esc to exit");

    let opengl = OpenGL::V3_2;
    let window: PistonWindow = WindowSettings::new("Fractal", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

    let mut frame_num: u32 = 0;
    let mut old_size: Vec2d = [0.0,0.0];

    let mut mouse_pos: Vec2d = [0.0,0.0];
    let mut mouse_down_pos = None;

    for e in window {
        e.draw_2d(|context, gfx| {
            let size = context.get_view_size();
            if size != old_size {
                println!("resized");
                old_size = size;
                window_handler.window_resized(size);
            }
            frame_num += 1;
            // println!("Render frame {}, window: {:?}", frame_num, size);
            let mut factory = e.factory.borrow_mut();
            let mut render_context = RenderContext {
                context: context,
                gfx: gfx,
                factory: &mut *factory,
            };
            window_handler.render_frame(&mut render_context, frame_num);
        });
        e.mouse_cursor(|x,y| {
            // mouse moved
            mouse_pos = [x,y];
        });
        e.press(|button| {
            match button {
                Button::Mouse(mbutton) => {
                    match mbutton {
                        MouseButton::Left => {
                            // mouse down
                            mouse_down_pos = Some(mouse_pos);
                            println!("Pressed mouse left: {:?}", mouse_down_pos.as_ref().unwrap());
                        },
                        _ => {}
                    }
                },
                Button::Keyboard(key) => {
                    match key {
                        Key::Backspace => {
                            // "backspace" key down
                            println!("reset zoom");
                            window_handler.reset_view();
                        },
                        Key::Minus => {
                            println!("zoom out");
                            let halves = [old_size[0]/2.0,old_size[1]/2.0];
                            let top_left = [-halves[0],-halves[1]];
                            let bot_right = [old_size[0]+halves[0], old_size[1]+halves[1]];

                            window_handler.zoom([top_left, bot_right]);
                        }
                        Key::Equals => {
                            println!("zoom in");
                            let quarters = [old_size[0]/4.0,old_size[1]/4.0];
                            let top_left = [quarters[0],quarters[1]];
                            let bot_right = [old_size[0]- quarters[0], old_size[1]-quarters[1]];
                            window_handler.zoom([top_left, bot_right]);
                        }
                        _ => {}
                    }
                },
                _ => {}
            }
        });
        e.release(|button| {
            // mouse up
            if button == Button::Mouse(MouseButton::Left) {
                let p2 = mouse_pos;
                println!("Released mouse left: {:?}", p2);
                if let Some(p1) = mouse_down_pos {
                    let top_left = [p1[0].min(p2[0]), p1[1].min(p2[1])];
                    let bot_right = [p1[0].max(p2[0]), p1[1].max(p2[1])];
                    window_handler.zoom([top_left, bot_right]);
                }
                mouse_down_pos = None;
            }
        });
    }
}
