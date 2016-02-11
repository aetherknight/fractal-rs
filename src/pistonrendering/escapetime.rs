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
use super::super::geometry::{Point, ViewAreaTransformer};
use super::*;

const WHITE_U8: [u8; 4] = [255, 255, 255, 255];
const BLACK_U8: [u8; 4] = [0, 0, 0, 255];

/// Draws escape time fractals by testing the point that each pixel corresponds to on the complex
/// plane.
pub struct EscapeTimeWindowHandler<'a> {
    etsystem: &'a EscapeTime,
    screen_size: Vec2d,
    view_area: [Point; 2],
    vat: ViewAreaTransformer,
    state: WhichFrame,
    /// Must be a u8 to work with Texture::from_image?
    canvas: Box<im::ImageBuffer<im::Rgba<u8>, Vec<u8>>>,
    /// Main thread only
    texture: Option<Texture<gfx_device_gl::Resources>>,
}

impl<'a> EscapeTimeWindowHandler<'a> {
    pub fn new(etsystem: &'a EscapeTime) -> EscapeTimeWindowHandler {
        let canvas = Box::new(im::ImageBuffer::new(800, 600));
        let view_area_c = etsystem.default_view_area();
        let view_area = [Point::from(view_area_c[0]), Point::from(view_area_c[1])];

        EscapeTimeWindowHandler {
            etsystem: etsystem,
            screen_size: [800.0, 600.0],
            view_area: view_area,
            vat: ViewAreaTransformer::new([800.0, 600.0], view_area[0], view_area[1]),
            state: WhichFrame::FirstFrame,
            canvas: canvas,
            texture: None,
        }
    }

    /// Recomputes the fractal for the screen. This should usually be called after the
    /// screen/window is resized, or after a new area is selected for viewing.
    fn redraw(&mut self) {
        self.state = WhichFrame::FirstFrame;
        self.vat = ViewAreaTransformer::new(self.screen_size, self.view_area[0], self.view_area[1]);
        println!("view area: {:?}", self.view_area);
        println!("pixel 0,0 maps to {}",
                 self.vat.map_pixel_to_point([0.0, 0.0]));
        println!("pixel {},{} maps to {}",
                 self.screen_size[0] as u32,
                 self.screen_size[1] as u32,
                 self.vat.map_pixel_to_point(self.screen_size));
        self.canvas = Box::new(im::ImageBuffer::from_fn(self.screen_size[0] as u32,
                                                        self.screen_size[1] as u32,
                                                        |x, y| {
                                                            let c: Complex64 =
                                                                self.vat
                                                                    .map_pixel_to_point([x as f64,
                                                                                         y as f64])
                                                                    .into();
                                                            if self.etsystem
                                                                   .test_point(c) {
                                                                im::Rgba(BLACK_U8)
                                                            } else {
                                                                im::Rgba(WHITE_U8)
                                                            }
                                                        }));
        self.texture = None;
    }
}

impl<'a> WindowHandler for EscapeTimeWindowHandler<'a> {
    fn window_resized(&mut self, new_size: Vec2d) {
        self.screen_size = new_size;
        self.redraw();
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
