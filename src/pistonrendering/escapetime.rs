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

use gfx_device_gl;
use image as im;
use num::complex::Complex64;
use piston_window::*;
use std::cmp;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::*;
use std::thread;
use time;

use super::*;
use super::super::escapetime::EscapeTime;
use super::super::geometry::{Point, ViewAreaTransformer};

type ImageBuffer = im::ImageBuffer<im::Rgba<u8>, Vec<u8>>;

/// Holds information about a render and the threads working on it.
///
/// Once a RenderingJob is configured, it can be told to start working. It will spawn up to
/// thread_count threads, all of whom start working on a sharded subset of the problem, updating
/// the shared_canvas as they go.
///
/// Dropping the RenderingJob will signal all of the worker threads to abort their work, if they
/// have not finished yet, and the thread that owns the RenderingJob will block until all of the
/// workers join.
struct RenderingJob {
    /// How many worker threads we want
    pub thread_count: u32,
    thread_sync: Vec<Option<(Sender<()>, thread::JoinHandle<()>)>>,
}

impl RenderingJob {
    pub fn new(thread_count: u32) -> RenderingJob {
        RenderingJob {
            thread_count: thread_count,
            thread_sync: Vec::with_capacity(thread_count as usize),
        }
    }

    pub fn start_work(&mut self,
                      mycanvas: Arc<RwLock<ImageBuffer>>,
                      myvat: Arc<ViewAreaTransformer>,
                      myetsystem: Arc<EscapeTime + Send + Sync>,
                      mycolors: Arc<Vec<[u8; 4]>>,
                      tl: Vec2d,
                      br: Vec2d) {
        for i in 0..self.thread_count {
            let (tx, rx) = channel();

            let shared_canvas = mycanvas.clone();
            let vat = myvat.clone();
            let etsystem = myetsystem.clone();
            let colors = mycolors.clone();
            let shard_factor = self.thread_count;

            let res = thread::Builder::new().name(format!("worker thread.{}", i)).spawn(move || {
                // create some timers
                let start_time = time::now_utc();
                let mut fractaltime = time::Duration::nanoseconds(0);
                let mut canvastime = time::Duration::nanoseconds(0);

                // Each thread will process x values, sharded by the number of threads
                // no `step_by` in stable yet.
                let sequence = ((tl[0] as u32)..(br[0] as u32))
                                   .into_iter()
                                   .enumerate()
                                   .filter(|&(index, _)| (index as u32 + i) % shard_factor == 0)
                                   .map(|(_, val)| val);
                for x in sequence {
                    // Check to see if we should stop
                    if let Err(TryRecvError::Disconnected) = rx.try_recv() {
                        println!("worker thread.{}: Remote side disconnected", i);
                        break;
                    }
                    let y_colors = ((tl[1] as u32)..(br[1] as u32))
                                       .into_iter()
                                       .map(|y| {
                                           let c: Complex64 = vat.map_pixel_to_point([x as f64,
                                                                                      y as f64])
                                                                 .into();
                                           let (attracted, time) = etsystem.test_point(c);
                                           if attracted {
                                               im::Rgba(AEBLUE_U8)
                                           } else {
                                               im::Rgba(colors[cmp::min(time, 50 - 1) as usize])
                                           }
                                       })
                                       .collect::<Vec<im::Rgba<u8>>>();
                    // only lock the canvas while writing to it
                    {
                        // Write a column at a time to improve performance. Locking for every pixel
                        // actually winds up harming performance, but a column at a time seems to
                        // work much better. Haven't tested trying to do multiple columns at once
                        // yet.
                        let mut canvas = shared_canvas.write().unwrap();
                        for (y, color) in y_colors.into_iter().enumerate() {
                            canvas.put_pixel(x, y as u32, color);
                        }
                    }
                }
                let finish_time = time::now_utc();
                println!("Worker thread.{} finished in {}",
                         i,
                         finish_time - start_time);
            });
            if let Ok(handle) = res {
                self.thread_sync.push(Some((tx, handle)));
            } else {
                panic!("Failed to spawn thread {}", i);
            }
        }
    }

    // pub fn live_thread_count(&self) -> u32 {
    //     self.thread_sync
    //         .iter()
    //         .map(|maybe_x| {
    //             if let Some(tuple) = maybe_x.as_ref() {
    //                 if let Ok(_) = tuple.0.send(()) {
    //                     1
    //                 } else {
    //                     0
    //                 }
    //             } else {
    //                 0
    //             }
    //         })
    //         .fold(0, |acc, x| acc + x)
    // }

    pub fn stop(&mut self) {
        for thread_info in &mut self.thread_sync {
            if let Some((tx, handle)) = thread_info.take() {
                drop(tx);
                let thread_name = handle.thread().name().unwrap_or("UNKNOWN").to_string();
                match handle.join() {
                    Ok(_) => {
                        println!("Joined {}", thread_name);
                    }
                    Err(_) => {
                        println!("{} panicked while it ran", thread_name);
                    }
                }
            }
        }
    }
}

impl Drop for RenderingJob {
    fn drop(&mut self) {
        self.stop();
    }
}


/// Draws escape time fractals by testing the point that each pixel corresponds to on the complex
/// plane.
pub struct EscapeTimeWindowHandler {
    etsystem: Arc<EscapeTime + Send + Sync>,
    threadcount: u32,
    screen_size: Vec2d,
    view_area: [Point; 2],
    vat: Arc<ViewAreaTransformer>,
    state: WhichFrame,
    /// Must be a u8 to work with Texture::from_image?
    canvas: Arc<RwLock<ImageBuffer>>,
    threads: RenderingJob,
    /// Main thread only
    texture: Option<Texture<gfx_device_gl::Resources>>,
}

impl EscapeTimeWindowHandler {
    pub fn new(etsystem: Arc<EscapeTime + Send + Sync>,
               threadcount: u32)
               -> EscapeTimeWindowHandler {
        let canvas = Arc::new(RwLock::new(im::ImageBuffer::new(800, 600)));
        let view_area_c = etsystem.default_view_area();
        let view_area = [Point::from(view_area_c[0]), Point::from(view_area_c[1])];

        EscapeTimeWindowHandler {
            etsystem: etsystem,
            threadcount: threadcount,
            screen_size: [800.0, 600.0],
            view_area: view_area,
            vat: Arc::new(ViewAreaTransformer::new([800.0, 600.0], view_area[0], view_area[1])),
            state: WhichFrame::FirstFrame,
            canvas: canvas,
            threads: RenderingJob::new(threadcount),
            texture: None,
        }
    }

    /// Recomputes the fractal for the screen. This should usually be called after the
    /// screen/window is resized, or after a new area is selected for viewing.
    fn redraw(&mut self) {
        self.state = WhichFrame::FirstFrame;
        self.vat = Arc::new(ViewAreaTransformer::new(self.screen_size,
                                                     self.view_area[0],
                                                     self.view_area[1]));
        println!("view area: {:?}", self.view_area);
        println!("pixel 0,0 maps to {}",
                 self.vat.map_pixel_to_point([0.0, 0.0]));
        println!("pixel {},{} maps to {}",
                 self.screen_size[0] as u32,
                 self.screen_size[1] as u32,
                 self.vat.map_pixel_to_point(self.screen_size));
        let colors =
            Arc::new(color_range_linear(BLACK_U8,
                                        WHITE_U8,
                                        cmp::min(self.etsystem.max_iterations(), 50) as usize));

        self.canvas = Arc::new(RwLock::new(im::ImageBuffer::new(self.screen_size[0] as u32,
                                                                self.screen_size[1] as u32)));
        self.threads = RenderingJob::new(self.threadcount);
        self.threads.start_work(self.canvas.clone(),
                                self.vat.clone(),
                                self.etsystem.clone(),
                                colors,
                                [0.0, 0.0],
                                self.screen_size);
        self.texture = None;
    }
}

impl WindowHandler for EscapeTimeWindowHandler {
    fn window_resized(&mut self, new_size: Vec2d) {
        self.screen_size = new_size;
        self.redraw();
    }

    fn render_frame(&mut self, render_context: &mut RenderContext, _: u32) {
        match self.state {
            WhichFrame::FirstFrame => {
                {
                    let canvas = self.canvas.read().unwrap();
                    self.texture = Some(Texture::from_image(render_context.factory,
                                                            &*canvas,
                                                            &TextureSettings::new())
                                            .unwrap());
                }

                clear(WHITE_F32, render_context.gfx);
                image(self.texture.as_ref().unwrap(),
                      render_context.context.transform,
                      render_context.gfx);

                self.state = WhichFrame::AllOtherFrames;
            }
            WhichFrame::SecondFrame => {}
            WhichFrame::AllOtherFrames => {
                let mut texture = self.texture.as_mut().unwrap();
                {
                    let canvas = self.canvas.read().unwrap();
                    if let Err(e) = texture.update(render_context.factory, &*canvas) {
                        println!("texture update error: {:?}", e);
                    }
                }

                clear(WHITE_F32, render_context.gfx);
                image(texture,
                      render_context.context.transform,
                      render_context.gfx);

                self.state = WhichFrame::AllOtherFrames;
            }
        }
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
