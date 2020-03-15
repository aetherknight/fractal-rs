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

use super::super::work_multiplexer::{
    ThreadedWorkMultiplexerBuilder, ThreadedWorkMultiplexerHandles,
};
use super::{RenderContext, WindowHandler};
use ::image::{ImageBuffer, Rgba};
use fractal_lib::color;
use fractal_lib::escapetime::EscapeTime;
use fractal_lib::geometry::{Point, ViewAreaTransformer};
use graphics::math::Vec2d;
use log;
use num::complex::Complex64;
use piston_window;
use std::cmp;
use std::sync::{Arc, RwLock};

type FractalImageBuffer = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Draws escape time fractals by testing the point that each pixel corresponds to on the complex
/// plane.
pub struct EscapeTimeWindowHandler {
    etsystem: Arc<dyn EscapeTime + Send + Sync>,
    screen_size: Vec2d,
    view_area: [Point; 2],
    vat: Arc<ViewAreaTransformer>,
    /// Must be a u8 to work with Texture::from_image?
    canvas: Arc<RwLock<FractalImageBuffer>>,
    threads: Option<ThreadedWorkMultiplexerHandles>,
    /// Main thread only
    texture_context: Option<piston_window::G2dTextureContext>,
}

impl EscapeTimeWindowHandler {
    pub fn new(etsystem: Arc<dyn EscapeTime + Send + Sync>) -> EscapeTimeWindowHandler {
        let canvas = Arc::new(RwLock::new(FractalImageBuffer::new(800, 600)));
        let view_area_c = etsystem.default_view_area();
        let view_area = [Point::from(view_area_c[0]), Point::from(view_area_c[1])];

        EscapeTimeWindowHandler {
            etsystem,
            screen_size: [800.0, 600.0],
            view_area,
            vat: Arc::new(ViewAreaTransformer::new(
                [800.0, 600.0],
                view_area[0],
                view_area[1],
            )),
            canvas,
            threads: None,
            texture_context: None,
        }
    }

    /// Recomputes the fractal for the screen. This should usually be called after the
    /// screen/window is resized, or after a new area is selected for viewing.
    fn redraw(&mut self) {
        self.vat = Arc::new(ViewAreaTransformer::new(
            self.screen_size,
            self.view_area[0],
            self.view_area[1],
        ));
        log::debug!("view area: {:?}", self.view_area);
        log::debug!(
            "pixel 0,0 maps to {}",
            self.vat.map_pixel_to_point([0.0, 0.0])
        );
        log::debug!(
            "pixel {},{} maps to {}",
            self.screen_size[0] as u32,
            self.screen_size[1] as u32,
            self.vat.map_pixel_to_point(self.screen_size)
        );
        let colors = Arc::new(color::color_range_linear(
            color::BLACK_U8,
            color::WHITE_U8,
            cmp::min(self.etsystem.max_iterations(), 50) as usize,
        ));

        self.canvas = Arc::new(RwLock::new(FractalImageBuffer::new(
            self.screen_size[0] as u32,
            self.screen_size[1] as u32,
        )));

        {
            let shared_canvas = (&self.canvas).clone();
            let vat = Arc::clone(&self.vat);
            let etsystem = Arc::clone(&self.etsystem);
            let colors = Arc::clone(&colors);
            let tl = [0.0, 0.0];
            let br = self.screen_size;

            let work_muxer = ThreadedWorkMultiplexerBuilder::new()
                .base_name("escapetime_render")
                .split_work(move |thread_id, total_threads, notifier, name| {
                    // Each thread will process x values, sharded by the number of
                    // threads.
                    //
                    // no `step_by` in stable yet.
                    let sequence = ((tl[0] as u32)..(br[0] as u32))
                        .enumerate()
                        .filter(|&(index, _)| (index + thread_id) % total_threads == 0)
                        .map(|(_, val)| val);
                    for x in sequence {
                        if notifier.should_i_stop() {
                            log::debug!("{}: Remote side disconnected", name);
                            break;
                        }
                        let y_colors = ((tl[1] as u32)..(br[1] as u32))
                            .map(|y| {
                                let c: Complex64 =
                                    vat.map_pixel_to_point([f64::from(x), f64::from(y)]).into();
                                let (attracted, time) = etsystem.test_point(c);
                                if attracted {
                                    Rgba(color::AEBLUE_U8.0)
                                } else {
                                    Rgba(colors[cmp::min(time, 50 - 1) as usize].0)
                                }
                            })
                            .collect::<Vec<Rgba<u8>>>();
                        // only lock the canvas while writing to it
                        {
                            // Write a column at a time to improve performance. Locking for every
                            // pixel actually winds up harming performance, but a column at a time
                            // seems to work much better. Haven't tested trying to do multiple
                            // columns at once yet.
                            let mut canvas = shared_canvas.write().unwrap();
                            for (y, color) in y_colors.into_iter().enumerate() {
                                canvas.put_pixel(x, y as u32, color);
                            }
                        }
                    }
                });
            self.threads = Some(work_muxer);
        }
    }
}

impl WindowHandler for EscapeTimeWindowHandler {
    fn window_resized(&mut self, new_size: Vec2d, window: &mut piston_window::PistonWindow) {
        // Set the new size
        self.screen_size = new_size;
        // Create a new canvas and start rendering it
        self.redraw();
        // Recreate the texture context (may not be necessary)
        self.texture_context = Some(window.create_texture_context());
    }

    fn render_frame(&mut self, render_context: &mut RenderContext, _: u32) {
        // With piston_window 0.85.0, I was able to create a texture, store it on the
        // WindowHandler, and then use texture.update(...) when rendering each frame in order to
        // update the texture object with the current canvas. However, with piston_window 0.107.0,
        // I seem unable to actually update the texture. The update does not fail, but the image in
        // the texture itself does not seem to change.
        //
        // Get a read-lock on the canvas, and create a texture from it.
        let texture = {
            let canvas = self.canvas.read().unwrap();
            piston_window::Texture::from_image(
                self.texture_context.as_mut().unwrap(),
                &*canvas,
                &piston_window::TextureSettings::new(),
            )
            .unwrap()
        };

        piston_window::clear(color::WHITE_F32.0, render_context.gfx);
        piston_window::image(
            &texture,
            render_context.context.transform,
            render_context.gfx,
        );
    }

    /// Change the view area to the newly selected area, and then redraw.
    fn zoom(&mut self, rect: [Vec2d; 2]) {
        let tlp = self.vat.map_pixel_to_point(rect[0]);
        let brp = self.vat.map_pixel_to_point(rect[1]);

        self.view_area = [tlp, brp];
        self.redraw();
    }

    fn reset_view(&mut self) {
        let view_area_c = self.etsystem.default_view_area();
        self.view_area = [Point::from(view_area_c[0]), Point::from(view_area_c[1])];
        self.redraw();
    }
}
