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

//! Window handlers for drawing points as part of playing a `ChaosGame`.

use super::{RenderContext, WhichFrame, WindowHandler};
use fractal_lib::chaosgame::ChaosGameMoveIterator;
use fractal_lib::color;
use fractal_lib::geometry::{Point, ViewAreaTransformer};
use gfx_device_gl;
use graphics;
use graphics::math::Vec2d;
use piston_window;

/// Draw a dot at the given point. (0.0,0.0) is the center of the screen, (1.0,1.0) is near the top
/// right, and (-1.0,-1.0) is near the bottom left.
fn draw_dot(context: graphics::context::Context, gfx: &mut piston_window::G2d, point: Point) {
    let view_size = context.get_view_size();
    let screen_width = view_size[0];
    let screen_height = view_size[1];

    let chaos_vat = ViewAreaTransformer::new(
        [screen_width, screen_height],
        Point { x: -1.0, y: -1.0 },
        Point { x: 1.0, y: 1.0 },
    );
    let screen_point = chaos_vat.map_point_to_pixel(point);

    piston_window::Rectangle::new(color::BLACK_F32.0).draw(
        [screen_point[0], screen_point[1], 1.0, 1.0],
        &graphics::draw_state::DrawState::default(),
        context.transform,
        gfx,
    );
}

pub struct ChaosGameWindowHandler {
    iter: Box<dyn ChaosGameMoveIterator>,
    which_frame: WhichFrame,
    dots_per_frame: u64,
    last_moves: Vec<Point>,
}

impl ChaosGameWindowHandler {
    pub fn new(
        game: Box<dyn ChaosGameMoveIterator>,
        dots_per_frame: u64,
    ) -> ChaosGameWindowHandler {
        ChaosGameWindowHandler {
            iter: game,
            which_frame: WhichFrame::FirstFrame,
            dots_per_frame,
            last_moves: Vec::with_capacity(dots_per_frame as usize),
        }
    }
}

impl WindowHandler for ChaosGameWindowHandler {
    fn window_resized(&mut self, _: Vec2d, _: &mut gfx_device_gl::Factory) {
        self.which_frame = WhichFrame::FirstFrame;
        self.iter.reset_game();
        self.last_moves = Vec::with_capacity(self.dots_per_frame as usize);
    }

    fn render_frame(&mut self, render_context: &mut RenderContext, _: u32) {
        match self.which_frame {
            WhichFrame::FirstFrame => {
                // The first frame clears its screen and starts drawing.
                piston_window::clear(color::WHITE_F32.0, render_context.gfx);
                // draw up to dots_per_frame dots, and store them for the next frame to also
                // draw
                for _ in 0..self.dots_per_frame {
                    if let Some(next_point) = self.iter.as_mut().next() {
                        draw_dot(render_context.context, render_context.gfx, next_point);
                        self.last_moves.push(next_point);
                    }
                }
                self.which_frame = WhichFrame::SecondFrame;
            }
            WhichFrame::SecondFrame => {
                // The second frame is on the second buffer, so it needs to clear the screen,
                // draw the first frame's dots, and then draw some more dots.
                piston_window::clear(color::WHITE_F32.0, render_context.gfx);
                // catch up to the first frame by draining last_moves
                for oldmove in self.last_moves.drain(..) {
                    draw_dot(render_context.context, render_context.gfx, oldmove);
                }
                // draw up to dots_per_frame dots, and refill last_moves.
                for _ in 0..self.dots_per_frame {
                    if let Some(next_point) = self.iter.as_mut().next() {
                        draw_dot(render_context.context, render_context.gfx, next_point);
                        self.last_moves.push(next_point);
                    }
                }
                self.which_frame = WhichFrame::AllOtherFrames;
            }
            _ => {
                // All remaining frames need to catch up to the last frame, and then move
                // forward.
                for oldmove in self.last_moves.drain(..) {
                    draw_dot(render_context.context, render_context.gfx, oldmove);
                }
                for _ in 0..self.dots_per_frame {
                    if let Some(next_point) = self.iter.as_mut().next() {
                        draw_dot(render_context.context, render_context.gfx, next_point);
                        self.last_moves.push(next_point);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {}
