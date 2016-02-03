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
pub mod turtle;

use graphics;
use piston_window::*;

const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

/// State machine for WindowHandlers that want to animate across the double buffered frames.
#[derive(Debug,PartialEq)]
pub enum WhichFrame {
    FirstFrame,
    SecondFrame,
    AllOtherFrames,
}

/// An object that can render frames of a drawing/animation/program/game.
pub trait WindowHandler {
    /// When the window is resized, we may need to plan to re-render.
    fn window_resized(&mut self);

    /// Render a frame.
    fn render_frame(&mut self,
                    window_size: Size,
                    context: graphics::context::Context,
                    gfx: &mut G2d,
                    frame_num: u32);
}

/// Runs a WindowHandler in a PistonWindow.
pub fn run(window_handler: &mut WindowHandler) {

    let opengl = OpenGL::V3_2;
    let window: PistonWindow = WindowSettings::new("Fractal", [800, 600])
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build Window: {}", e));

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
            window_handler.render_frame(size, context, gfx, frame_num);
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
