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

use super::super::escapetime::EscapeTime;
use super::*;

const WHITE_U8: [u8; 4] = [255, 255, 255, 255];
const BLACK_U8: [u8; 4] = [0, 0, 0, 255];

/// Draws escape time fractals by testing the point that each pixel corresponds to on the complex
/// plane.
pub struct EscapeTimeWindowHandler<'a> {
    etsystem: &'a EscapeTime,
    screen_size: Vec2d,
    state: WhichFrame,
    /// Must be a u8 to work with Texture::from_image?
    canvas: Box<im::ImageBuffer<im::Rgba<u8>, Vec<u8>>>,
    /// Main thread only
    texture: Option<Texture<gfx_device_gl::Resources>>,
}

impl<'a> EscapeTimeWindowHandler<'a> {
    pub fn new(etsystem: &'a EscapeTime) -> EscapeTimeWindowHandler {
        let canvas = Box::new(im::ImageBuffer::new(800, 600));

        EscapeTimeWindowHandler {
            etsystem: etsystem,
            screen_size: [800.0, 600.0],
            state: WhichFrame::FirstFrame,
            canvas: canvas,
            texture: None,
        }
    }

    /// Map a pixel coordinate to a complex coordinate.
    fn pixel_to_complex(&self, x: u32, y: u32) -> Complex64 {
        let max_x = self.screen_size[0];
        let max_y = self.screen_size[1];
        let min_dim = max_x.min(max_y);

        let scale = 3.0 / min_dim;
        let shift_x = 2.0 * max_x / min_dim;
        let shift_y = 1.5 * max_y / min_dim;

        let real = (x as f64) * scale - shift_x;
        let imag = -((y as f64) * scale - shift_y);

        Complex64::new(real, imag)
    }
}

impl<'a> WindowHandler for EscapeTimeWindowHandler<'a> {
    fn window_resized(&mut self, new_size: Vec2d) {
        self.state = WhichFrame::FirstFrame;
        self.screen_size = new_size;
        println!("pixel 0,0 maps to {}", self.pixel_to_complex(0, 0));
        println!("pixel {},{} maps to {}",
                 new_size[0] as u32,
                 new_size[1] as u32,
                 self.pixel_to_complex(new_size[0] as u32, new_size[1] as u32));
        self.canvas = Box::new(im::ImageBuffer::from_fn(new_size[0] as u32,
                                                        new_size[1] as u32,
                                                        |x, y| {
                                                            let c = self.pixel_to_complex(x, y);
                                                            if self.etsystem
                                                                   .test_point(c) {
                                                                im::Rgba(BLACK_U8)
                                                            } else {
                                                                im::Rgba(WHITE_U8)
                                                            }
                                                        }));
        self.texture = None;
    }

    fn render_frame(&mut self, render_context: &mut RenderContext, _: u32) {
        match self.state {
            WhichFrame::FirstFrame => {
                self.texture = Some(Texture::from_image(render_context.factory,
                                                        &self.canvas,
                                                        &TextureSettings::new())
                                        .unwrap());

                clear(WHITE, render_context.gfx);
                image(self.texture.as_ref().unwrap(),
                      render_context.context.transform,
                      render_context.gfx);

                self.state = WhichFrame::SecondFrame;
            }
            WhichFrame::SecondFrame => {
                clear(WHITE, render_context.gfx);
                image(self.texture.as_ref().unwrap(),
                      render_context.context.transform,
                      render_context.gfx);

                self.state = WhichFrame::AllOtherFrames;
            }
            WhichFrame::AllOtherFrames => {}
        }
    }
}
