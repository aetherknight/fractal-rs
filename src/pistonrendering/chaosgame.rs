// Copyright (c) 2016 William (B.J.) Snow Orvis
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

//! Window handlers for drawing points as part of playing a ChaosGame.

use std::sync::Arc;
use graphics;
use piston_window::*;

use super::super::chaosgame::{ChaosGame, ChaosGameMoveIterator};
use super::{BLACK, WHITE, WindowHandler, WhichFrame};
use super::super::geometry::Point;

const DOTS_PER_FRAME: usize = 100;

pub struct ChaosGameWindowHandler {
    game: Arc<ChaosGame>,
    which_frame: WhichFrame,
    iter: Option<ChaosGameMoveIterator>,
    // last_move: Point,
    last_moves: [Point; DOTS_PER_FRAME],
}

impl ChaosGameWindowHandler {
    pub fn new(game: Arc<ChaosGame>) -> ChaosGameWindowHandler {
        ChaosGameWindowHandler {
            game: game,
            which_frame: WhichFrame::FirstFrame,
            iter: None,
            last_moves: [Point { x: 0.0, y: 0.0 }; DOTS_PER_FRAME],
        }
    }
}

/// Draw a dot at the given point. (0.0,0.0) is the center of the screen, (1.0,1.0) is near the top
/// right, and (-1.0,-1.0) is near the bottom left.
fn draw_dot(gfx: &mut G2d, window_size: Size, context: graphics::context::Context, point: Point) {
    let screen_width = window_size.width;
    let screen_height = window_size.height;

    let originx = (screen_width / 2) as f64;
    let originy = (screen_height / 2) as f64;

    let taller = (screen_height as i64 - screen_width as i64) > 0;

    // use the smaller direction to determine how many pixels are in one unit of
    // distance here.
    let one_unit_to_pixels = if taller {
        (screen_width / 2) as f64
    } else {
        (screen_height / 2) as f64
    };

    let transform = context.transform
                           .trans(originx, originy)
                           .zoom(one_unit_to_pixels)
                           .flip_v()
                           .trans(0.0, 0.0);

    let delta = 0.5 / one_unit_to_pixels as f64;

    // println!("Drawing {}", point);
    Rectangle::new(BLACK).draw([point.x - delta, point.y - delta, 2.0 * delta, 2.0 * delta],
                               default_draw_state(),
                               transform,
                               gfx);
}

impl WindowHandler for ChaosGameWindowHandler {
    fn window_resized(&mut self) {
        self.which_frame = WhichFrame::FirstFrame;
        self.iter = None;
        // self.last_move = Point { x: 0.0, y: 0.0 };
        self.last_moves = [Point { x: 0.0, y: 0.0 }; DOTS_PER_FRAME];
    }

    // First frame clears, sets params, draws.
    // second frame clears, sets params, draws, draws.
    //
    // the RNG and seed must match for each buffer. otherwise, the double buffering
    // will flicker.
    fn render_frame(&mut self,
                    window_size: Size,
                    context: graphics::context::Context,
                    gfx: &mut G2d,
                    frame_num: u32) {
        match self.which_frame {
            WhichFrame::FirstFrame => {
                // The first frame clears its screen and draws a point.
                clear(WHITE, gfx);
                self.iter = Some(ChaosGameMoveIterator::new(self.game.clone()));
                for i in 0..DOTS_PER_FRAME {
                    if let Some(next_point) = self.iter.as_mut().unwrap().next() {
                        draw_dot(gfx, window_size, context, next_point);
                        self.last_moves[i] = next_point;
                    }
                }
                self.which_frame = WhichFrame::SecondFrame;
            }
            WhichFrame::SecondFrame => {
                // The second frame is on the second buffer, so it needs to clear the screen,
                // draw the first point, and then draw the next point.
                clear(WHITE, gfx);
                for i in 0..DOTS_PER_FRAME {
                    draw_dot(gfx, window_size, context, self.last_moves[i]);
                }
                for i in 0..DOTS_PER_FRAME {
                    if let Some(next_point) = self.iter.as_mut().unwrap().next() {
                        draw_dot(gfx, window_size, context, next_point);
                        self.last_moves[i] = next_point;
                    }
                }
                self.which_frame = WhichFrame::AllOtherFrames;
            }
            _ => {
                // All other frames need to draw the last point (already drawn on the other
                // buffer) and then add a point to the current buffer.
                for i in 0..DOTS_PER_FRAME {
                    draw_dot(gfx, window_size, context, self.last_moves[i]);
                }
                for i in 0..DOTS_PER_FRAME {
                    if let Some(next_point) = self.iter.as_mut().unwrap().next() {
                        draw_dot(gfx, window_size, context, next_point);
                        self.last_moves[i] = next_point;
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
}
