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

use fractal_lib::color;
use fractal_lib::escapetime::EscapeTime;
use fractal_lib::geometry;
use js_sys::Array;
use num::complex::Complex64;
use std::cmp;
use wasm_bindgen::prelude::*;
use wasm_bindgen::Clamped;
use web_sys::{console, CanvasRenderingContext2d, ImageData};

#[wasm_bindgen]
pub struct EscapeTimeAnimation {
    /// The rendering context.
    ctx: CanvasRenderingContext2d,

    /// Which EscapeTime system is being animated. Boxed to encapsulate/avoid generics.
    etsystem: Box<dyn EscapeTime>,

    /// The current part of the fractal we're viewing.
    view_area: [geometry::Point; 2],
}

impl EscapeTimeAnimation {
    pub fn new(
        ctx: CanvasRenderingContext2d,
        etsystem: Box<dyn EscapeTime>,
    ) -> EscapeTimeAnimation {
        let view_area_c = etsystem.default_view_area();
        let view_area = [
            geometry::Point::from(view_area_c[0]),
            geometry::Point::from(view_area_c[1]),
        ];
        EscapeTimeAnimation {
            ctx,
            etsystem,
            view_area,
        }
    }

    fn render(&self) {
        let screen_width = self.ctx.canvas().unwrap().width();
        let screen_height = self.ctx.canvas().unwrap().height();
        let vat = geometry::ViewAreaTransformer::new(
            [screen_width.into(), screen_height.into()],
            self.view_area[0],
            self.view_area[1],
        );
        console::log_1(&format!("View area: {:?}", self.view_area).into());
        console::log_1(&format!("pixel 0,0 maps to {}", vat.map_pixel_to_point([0.0, 0.0])).into());
        console::log_1(
            &format!(
                "pixel {},{} maps to {}",
                screen_width as u32,
                screen_height as u32,
                vat.map_pixel_to_point([screen_width.into(), screen_height.into()])
            )
            .into(),
        );

        console::log_1(&"build color range".into());
        let colors = color::color_range_linear(
            color::BLACK_U8,
            color::WHITE_U8,
            cmp::min(self.etsystem.max_iterations(), 50) as usize,
        );

        console::log_1(&"build image pixels".into());
        let mut image_pixels = (0..screen_height)
            .map(|y| {
                (0..screen_width)
                    .map(|x| {
                        let c: Complex64 =
                            vat.map_pixel_to_point([f64::from(x), f64::from(y)]).into();
                        let (attracted, time) = self.etsystem.test_point(c);
                        if attracted {
                            color::AEBLUE_U8.0.iter()
                        } else {
                            colors[cmp::min(time, 50 - 1) as usize].0.iter()
                        }
                    })
                    .flatten()
                    .collect::<Vec<&u8>>()
            })
            .flatten()
            .cloned()
            .collect::<Vec<u8>>();;

        // Construct a Clamped Uint8 Array
        console::log_1(&"build clamped image array".into());
        let clamped_image_array = Clamped(image_pixels.as_mut_slice());

        // Create an ImageData from the array
        console::log_1(&"Create Image Data".into());
        let image = ImageData::new_with_u8_clamped_array_and_sh(
            clamped_image_array,
            screen_width,
            screen_height,
        )
        .unwrap();

        console::log_1(&"Put Image Data".into());
        self.ctx.put_image_data(&image, 0.0, 0.0).unwrap();
    }
}

#[wasm_bindgen]
impl EscapeTimeAnimation {
    pub fn draw_one_frame(&mut self) -> bool {
        self.render();
        false
    }

    pub fn pixel_to_coordinate(&self, x: f64, y: f64) -> Array {
        let screen_width = self.ctx.canvas().unwrap().width();
        let screen_height = self.ctx.canvas().unwrap().height();

        let vat = geometry::ViewAreaTransformer::new(
            [screen_width.into(), screen_height.into()],
            self.view_area[0],
            self.view_area[1],
        );
        let pos_point = vat.map_pixel_to_point([x, y]);
        Array::of2(&pos_point.x.into(), &pos_point.y.into())
    }
}
