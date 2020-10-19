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

//! Exports the various fractal curves so that they can be called and animated/rendered from
//! JavaScript in a browser.
//!
//! Each fractal type has its own factory function that returns an object that implements the
//! following interface/protocol (although traits don't map to JS):
//!
//! ```
//! trait FractalAnimation {
//!     fn draw_one_frame(&mut self) -> bool;
//!     fn pixel_to_coordinate(&self, x: f64, y: f64) -> Array;
//!     fn zoom(&mut self, x1, y1, x2, y2) -> bool;
//! }
//! ```

use console_error_panic_hook;
use console_log;
use js_sys::Array;
use log;
use std::str::FromStr;
use strum::IntoEnumIterator;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlInputElement};

mod chaosgame;
mod escapetime;
mod fractaldata;
mod turtle;

use fractaldata::{FractalConfig, SelectedFractal};

pub trait FractalAnimation {
    fn draw_one_frame(&mut self) -> bool;
    fn pixel_to_coordinate(&self, x: f64, y: f64) -> Array;
    fn zoom(&mut self, _x1: f64, _y1: f64, _x2: f64, _y2: f64) -> bool;
}

// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
// #![allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

/// Initializes the Model at app launch
fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        canvas: ElRef::default(),
        selected_fractal: SelectedFractal::BarnsleyFern,
        current_config: FractalConfig::NoConfig,
        current_animation: None,
        animation_ongoing: false,
    }
}

struct Model {
    /// The Canvas element that will be drawn. Stored here because a reference to it is needed
    /// during updates to kick off new animations.
    canvas: ElRef<HtmlCanvasElement>,
    /// The currently selected fractal. Used to show configuration about the fractal.
    selected_fractal: SelectedFractal,
    /// The configuration for the currently selected fractal. It needs to match the kind of fractl
    /// selected.
    current_config: FractalConfig,
    /// The currently ongoing fractal animation. This object does the work of rendering either each
    /// frame, or the whole fractal at once (if not drawn using an animation).
    current_animation: Option<Box<dyn FractalAnimation>>,
    /// Whether the animation is currently ongoing.
    ///
    /// TODO: can we use the None option for Current animation?
    animation_ongoing: bool,
}

enum Msg {
    /// Indiactes that we should render the next frame of the animation
    AnimationFrameRequested,
    /// Indicates that a configuration field changed. It specifies the configuration field name, as
    /// well as the new value (currently they can only be integers)
    ConfigChanged(String, u32),
    /// Indicates which fractal was selected for configuration and eventually running.
    FractalSelected(String),
    /// Whether to start the animation for the currently selected fractal and configuration.
    RunClicked,
}

/// Handles events that send `Msg`s, resulting in updates to the Model.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FractalSelected(selected) => {
            log::debug!("selected {}", selected);
            model.selected_fractal =
                SelectedFractal::from_str(selected.as_ref()).expect("unknown selected fractal");
            model.current_config = model.selected_fractal.default_config();
            log::debug!("config {:?}", model.current_config);
        }
        Msg::ConfigChanged(input, new_value) => {
            model.current_config.apply_change(input, new_value);
        }
        Msg::RunClicked => {
            log::debug!("clicked run");
            // Initialize a new animation
            let canvas = model.canvas.get().expect("get canvas element failed");
            model.current_animation = Some(
                model
                    .selected_fractal
                    .build_animation(&canvas, &model.current_config),
            );
            if !model.animation_ongoing {
                // There might already be an animation running. We don't want to double-up on the
                // number of AnimationFrameRequested messages if that's the case.
                orders.after_next_render(|_| Msg::AnimationFrameRequested);
                model.animation_ongoing = true;
            }
        }
        Msg::AnimationFrameRequested => {
            log::debug!("animation frame requested");
            match &mut model.current_animation {
                None => {}
                Some(animation) => {
                    // Animate until it says its done
                    if animation.draw_one_frame() {
                        orders.after_next_render(|_| Msg::AnimationFrameRequested);
                    } else {
                        model.animation_ongoing = false;
                    }
                }
            }
        }
    }
}

/// Renders/re-renders the HTML side of the UI
///
/// Note that updates to the canvas are handled by the currently ongoing animation.
fn view(model: &Model) -> Node<Msg> {
    log::debug!("view");
    div![
        div![
            style! {St::Float => "left"},
            canvas![
                el_ref(&model.canvas),
                attrs! { At::Id => "fractal-canvas", At::Width => "800", At::Height => "600"},
                style! {St::BackgroundColor => "white"}
            ],
            div![attrs! {At::Id => "coords"}, "Canvas coords:"],
            div![attrs! {At::Id => "fractal-coords"}, "Fractal coords:"],
        ],
        div![
            style! {St::Float => "left"},
            view_menu(),
            view_config(&model.current_config),
            button!["Run", ev(Ev::Click, |_| Msg::RunClicked)],
        ]
    ]
}

/// Renders/re-renders the menu for selecting which fractal animation to configure and run next.
fn view_menu() -> Node<Msg> {
    select![
        attrs! {At::Id => "fractal-type"},
        SelectedFractal::iter().map(|fractal| {
            option![
                attrs! {At::Value => <&'static str>::from(fractal)},
                fractal.name()
            ]
        }),
        input_ev(Ev::Input, Msg::FractalSelected),
        // TODO: while an animation is ongoing, the menu gets re-rendered while the dropdown is
        // open, making it unusable. Can we pause the animation, or should we disable the menu
        // until the animation itself is paused?

        // ev(Ev::Change, |event| log::debug!("change {:?}", event)),
        // ev(Ev::Focus, |event| log::debug!("focus {:?}", event)),
        // ev(Ev::Blur, |event| log::debug!("blur {:?}", event)),
    ]
}

/// Event callback that runs HTML form validations on the target form input element.
///
/// It returns a `Msg::ConfigChanged` if the modified field passes validations, or `None` if it
/// does not.
fn validate_input(event: web_sys::Event) -> Option<Msg> {
    let target = event
        .target()
        .unwrap()
        .dyn_into::<HtmlInputElement>()
        .unwrap();
    target.check_validity();
    if target.report_validity() {
        if let Ok(value) = target.value().parse::<u32>() {
            return Some(Msg::ConfigChanged(target.id(), value));
        }
    }
    None
}

/// Renders/re-renders the configuration for the specified FractalConfig.
///
/// It renders different fields based on the "kind" of fractal, and sets up HTML form validations
/// to ensure that the model is only updated if the configuration is valid.
fn view_config(config: &FractalConfig) -> Node<Msg> {
    div![
        attrs! {At::Id => "config"},
        match config {
            FractalConfig::NoConfig => div!["No configuration for this fractal"],
            FractalConfig::TurtleCurveConfig { iteration } => div![div![
                label![attrs! {At::For => "iteration"}, "Iterations"],
                input![
                    attrs! {
                        At::Id => "iteration",
                        At::Type => "number",
                        At::Required => "true",
                        At::Value => iteration,
                        At::Min => 0,
                    },
                    ev(Ev::Input, validate_input),
                ]
            ]],
            FractalConfig::EscapeTimeConfig {
                max_iterations,
                power,
            } => div![
                div![
                    label![attrs! {At::For => "max_iterations"}, "Max Iterations"],
                    input![
                        attrs! {
                            At::Id => "max_iterations",
                            At::Type => "number",
                            At::Required => "true",
                            At::Value => max_iterations,
                            At::Min => 1,
                        },
                        ev(Ev::Input, validate_input),
                    ],
                ],
                div![
                    label![attrs! {At::For => "power"}, "Power"],
                    input![
                        attrs! {
                            At::Id => "power",
                            At::Type => "number",
                            At::Required => "true",
                            At::Value => power,
                            At::Min => 1,
                        },
                        ev(Ev::Input, validate_input),
                    ],
                ],
            ],
        },
        format!("{:?}", config),
    ]
}

/// The start function for the WASM. It initializes seed with the app's init, update, and view.
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    log::debug!("Start App");
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
