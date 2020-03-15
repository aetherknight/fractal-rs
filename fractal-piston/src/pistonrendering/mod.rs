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

//! A piston window-based renderer and event loop.

pub mod chaosgame;
pub mod escapetime;
pub mod turtle;

use graphics;
use graphics::math::Vec2d;
use piston_window::{
    Button, G2d, Key, MouseButton, MouseCursorEvent, PistonWindow, PressEvent, ReleaseEvent,
    RenderEvent, WindowSettings,
};

/// State machine for `WindowHandlers` that want to animate across the double buffered frames.
#[derive(Debug, PartialEq)]
pub enum WhichFrame {
    FirstFrame,
    SecondFrame,
    AllOtherFrames,
}

/// Information about the viewport and graphical backend needed by `WindowHandler::render_frame`.
pub struct RenderContext<'a, 'b: 'a> {
    /// Graphics context, describing the viewport, base transform, etc.
    pub context: graphics::context::Context,
    /// Graphics backend
    pub gfx: &'a mut G2d<'b>,
    // We can't include the gfx Factory here because it would cause the pison window to be borrowed
    // twice (since Piston changed draw_2d from being on the event to being on the window object,
    // and the lambda passed to draw_2d is where the RenderContext is constructed)
}

/// An object that can render frames of a drawing/animation/program/game.
pub trait WindowHandler {
    /// When the window is resized (including the first time it is created/sized), do or gather any
    /// information needed in order to start the render/re-render. (Eg, to extract the gfx context,
    /// texture context, etc.)
    fn window_resized(&mut self, new_size: Vec2d, window: &mut PistonWindow);

    /// Render a frame.
    fn render_frame(&mut self, context: &mut RenderContext, frame_num: u32);

    /// Optional: used to indicate that the user selected an area of the window to zoom in on.
    fn zoom(&mut self, rect: [Vec2d; 2]) {
        println!("Selected: {:?}, {:?}", rect[0], rect[1]);
    }

    /// Optional: used to indicate that the user wants to revert to the default view.
    fn reset_view(&mut self) {
        println!("Reset zoom");
    }
}

/// Runs a `WindowHandler` in a `PistonWindow`.
pub fn run(window_handler: &mut dyn WindowHandler) {
    println!("Use the mouse to select an area to zoom in on");
    println!("Press backspace to reset the view back to the initial view");
    println!("Press esc to exit");

    let mut window: PistonWindow = WindowSettings::new("Fractal", [800, 600])
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

    let mut frame_num: u32 = 0;
    let mut old_size: Vec2d = [0.0, 0.0];

    let mut mouse_pos: Vec2d = [0.0, 0.0];
    let mut mouse_down_pos = None;

    while let Some(e) = window.next() {
        if let Some(args) = e.render_args() {
            let uvec = args.viewport().window_size;
            #[allow(clippy::identity_conversion)]
            let size: Vec2d = [f64::from(uvec[0]), f64::from(uvec[1])];
            if size != old_size {
                println!("resized");
                old_size = size;
                window_handler.window_resized(size, &mut window);
            }
        }
        window.draw_2d(&e, |context, gfx, _device| {
            frame_num += 1;
            // println!("Render frame {}, window: {:?}", frame_num, size);
            let mut render_context = RenderContext { context, gfx };
            window_handler.render_frame(&mut render_context, frame_num);
        });
        e.mouse_cursor(|coords| {
            // mouse moved
            mouse_pos = coords;
        });
        e.press(|button| {
            match button {
                Button::Mouse(mbutton) => {
                    if let MouseButton::Left = mbutton {
                        // mouse down
                        mouse_down_pos = Some(mouse_pos);
                        println!("Pressed mouse left: {:?}", mouse_down_pos.as_ref().unwrap());
                    }
                }
                Button::Keyboard(key) => {
                    match key {
                        Key::Backspace => {
                            // "backspace" key down
                            println!("reset zoom");
                            window_handler.reset_view();
                        }
                        Key::Minus => {
                            println!("zoom out");
                            // make the current screen's dimension become 50% the new view
                            let halves = [old_size[0] / 2.0, old_size[1] / 2.0];
                            let new_top_left = [-halves[0], -halves[1]];
                            let new_bot_right = [old_size[0] + halves[0], old_size[1] + halves[1]];

                            window_handler.zoom([new_top_left, new_bot_right]);
                        }
                        Key::Equals => {
                            println!("zoom in");
                            // zoom in so that 50% of the current view is the new view
                            let quarters = [old_size[0] / 4.0, old_size[1] / 4.0];
                            let new_top_left = [quarters[0], quarters[1]];
                            let new_bot_right =
                                [old_size[0] - quarters[0], old_size[1] - quarters[1]];

                            window_handler.zoom([new_top_left, new_bot_right]);
                        }
                        Key::Up => {
                            println!("up");
                            // scroll up by 25%
                            let move_up = old_size[1] * 0.25;
                            let new_top_left = [0.0, -move_up];
                            let new_bot_right = [old_size[0], old_size[1] - move_up];

                            window_handler.zoom([new_top_left, new_bot_right]);
                        }
                        Key::Down => {
                            let move_down = old_size[1] * 0.25;
                            let new_top_left = [0.0, move_down];
                            let new_bot_right = [old_size[0], old_size[1] + move_down];

                            window_handler.zoom([new_top_left, new_bot_right]);
                        }
                        Key::Right => {
                            let move_right = old_size[0] * 0.25;
                            let new_top_left = [move_right, 0.0];
                            let new_bot_right = [old_size[0] + move_right, old_size[1]];

                            window_handler.zoom([new_top_left, new_bot_right]);
                        }
                        Key::Left => {
                            let move_left = old_size[0] * 0.25;
                            let new_top_left = [-move_left, 0.0];
                            let new_bot_right = [old_size[0] - move_left, old_size[1]];

                            window_handler.zoom([new_top_left, new_bot_right]);
                        }
                        _ => {}
                    }
                }
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
