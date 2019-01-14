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

use std::cmp;
use std::sync::{Arc, RwLock};

use ::image::{ImageBuffer, Rgba};
use gfx_device_gl;
use gfx_device_gl::Factory;
use num::complex::Complex64;
use piston_window::*;

use super::super::escapetime::EscapeTime;
use super::super::geometry::{Point, ViewAreaTransformer};
use super::super::work_multiplexer::*;
use super::*;

type FractalImageBuffer = ImageBuffer<Rgba<u8>, Vec<u8>>;

/// Draws escape time fractals by testing the point that each pixel corresponds to on the complex
/// plane.
pub struct EscapeTimeWindowHandler {
    etsystem: Arc<EscapeTime + Send + Sync>,
    screen_size: Vec2d,
    view_area: [Point; 2],
    vat: Arc<ViewAreaTransformer>,
    /// Must be a u8 to work with Texture::from_image?
    canvas: Arc<RwLock<FractalImageBuffer>>,
    threads: Option<ThreadedWorkMultiplexerHandles>,
    /// Main thread only
    texture: Option<Texture<gfx_device_gl::Resources>>,
}

impl EscapeTimeWindowHandler {
    pub fn new(etsystem: Arc<EscapeTime + Send + Sync>) -> EscapeTimeWindowHandler {
        let canvas = Arc::new(RwLock::new(FractalImageBuffer::new(800, 600)));
        let view_area_c = etsystem.default_view_area();
        let view_area = [Point::from(view_area_c[0]), Point::from(view_area_c[1])];

        EscapeTimeWindowHandler {
            etsystem: etsystem,
            screen_size: [800.0, 600.0],
            view_area: view_area,
            vat: Arc::new(ViewAreaTransformer::new(
                [800.0, 600.0],
                view_area[0],
                view_area[1],
            )),
            canvas: canvas,
            threads: None,
            texture: None,
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
        println!("view area: {:?}", self.view_area);
        println!(
            "pixel 0,0 maps to {}",
            self.vat.map_pixel_to_point([0.0, 0.0])
        );
        println!(
            "pixel {},{} maps to {}",
            self.screen_size[0] as u32,
            self.screen_size[1] as u32,
            self.vat.map_pixel_to_point(self.screen_size)
        );
        let colors = Arc::new(color_range_linear(
            BLACK_U8,
            WHITE_U8,
            cmp::min(self.etsystem.max_iterations(), 50) as usize,
        ));

        self.canvas = Arc::new(RwLock::new(FractalImageBuffer::new(
            self.screen_size[0] as u32,
            self.screen_size[1] as u32,
        )));

        {
            let shared_canvas = Arc::clone(&self.canvas);
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
                        .into_iter()
                        .enumerate()
                        .filter(|&(index, _)| (index + thread_id) % total_threads == 0)
                        .map(|(_, val)| val);
                    for x in sequence {
                        if notifier.should_i_stop() {
                            println!("{}: Remote side disconnected", name);
                            break;
                        }
                        let y_colors = ((tl[1] as u32)..(br[1] as u32))
                            .into_iter()
                            .map(|y| {
                                let c: Complex64 =
                                    vat.map_pixel_to_point([f64::from(x), f64::from(y)]).into();
                                let (attracted, time) = etsystem.test_point(c);
                                if attracted {
                                    Rgba(AEBLUE_U8.0)
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
    fn initialize_with_window(&mut self, window: &mut PistonWindow) {
        let canvas = self.canvas.read().unwrap();
        self.texture = Some(
            Texture::from_image(&mut window.factory, &*canvas, &TextureSettings::new()).unwrap(),
        );
    }

    fn window_resized(&mut self, new_size: Vec2d, factory: &mut Factory) {
        // Set the new size
        self.screen_size = new_size;
        // Create a new canvas and start rendering it
        self.redraw();
        // Recreate the Texture for rendering (it will also be updated with the canvas on each
        // tick)
        {
            let canvas = self.canvas.read().unwrap();
            self.texture =
                Some(Texture::from_image(factory, &*canvas, &TextureSettings::new()).unwrap());
        }
    }

    fn render_frame(&mut self, render_context: &mut RenderContext, _: u32) {
        // Convert the Option<Texture> into a mutable reference to the Texture
        let texture = self.texture.as_mut().unwrap();
        {
            let canvas = self.canvas.read().unwrap();
            if let Err(e) = texture.update(&mut render_context.gfx.encoder, &*canvas) {
                println!("texture update error: {:?}", e);
            }
        }

        clear(WHITE_F32.0, render_context.gfx);
        image(
            texture,
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
