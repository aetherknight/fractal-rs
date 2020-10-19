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
use super::FractalAnimation;
use fractal_lib::chaosgame::ChaosGameMoveIterator;
use fractal_lib::geometry;
use js_sys::Array;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

/// Constructs a ViewAreaTransformer for converting between a canvas pixel-coordinate and the
/// coordinate system used by Chaos Games.
///
/// The ChaosGame fractals expect a view area that covers between -1.0 and 1.0 on the X axis, as
/// well as -1.0 to 1.0 on the Y axis, with positive values going up and right.
fn chaos_game_vat(canvas: &HtmlCanvasElement) -> geometry::ViewAreaTransformer {
    let screen_width = f64::from(canvas.width());
    let screen_height = f64::from(canvas.height());

    geometry::ViewAreaTransformer::new(
        [screen_width, screen_height],
        geometry::Point { x: -1.0, y: -1.0 },
        geometry::Point { x: 1.0, y: 1.0 },
    )
}

/// Represents everything needed to render a chaos game fractal as an animation.
#[wasm_bindgen]
pub struct ChaosGameAnimation {
    ctx: CanvasRenderingContext2d,
    iter: Box<dyn ChaosGameMoveIterator>,
}

impl ChaosGameAnimation {
    pub fn new(
        ctx: CanvasRenderingContext2d,
        chaos_game: Box<dyn ChaosGameMoveIterator>,
    ) -> ChaosGameAnimation {
        ChaosGameAnimation {
            ctx,
            iter: chaos_game,
        }
    }

    fn draw_point(&self, point: geometry::Point) {
        let canvas = self.ctx.canvas().unwrap();
        let pixel_pos = chaos_game_vat(&canvas).map_point_to_pixel(point);
        // log::debug(&format!("pixels: {}, {}", pixel_pos[0], pixel_pos[1]).into());
        self.ctx.set_fill_style(&"black".into());
        self.ctx.fill_rect(pixel_pos[0], pixel_pos[1], 1.0, 1.0);
        // self.ctx.stroke();
    }
}

impl FractalAnimation for ChaosGameAnimation {
    /// Draws one point of the chaos game animation.
    ///
    /// Should always return true, unless the underlying chaos game's iterator ends for some
    /// reason.
    fn draw_one_frame(&mut self) -> bool {
        if let Some(next_point) = self.iter.next() {
            // log::debug(&format!("{}", next_point).into());
            self.draw_point(next_point);
            true
        } else {
            log::debug!("No more points");
            false
        }
    }

    /// Translates a pixel-coordinate on the Canvas into the coordinate system used by a chaos
    /// game.
    ///
    /// See chaos_game_vat for more information on the coordinate system for chaos games.
    fn pixel_to_coordinate(&self, x: f64, y: f64) -> Array {
        let canvas = self.ctx.canvas().unwrap();
        let pos_point = chaos_game_vat(&canvas).map_pixel_to_point([x, y]);
        Array::of2(&pos_point.x.into(), &pos_point.y.into())
    }

    fn zoom(&mut self, _x1: f64, _y1: f64, _x2: f64, _y2: f64) -> bool {
        false
    }
}
