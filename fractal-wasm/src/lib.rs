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

//! A Seed application that runs and renders various fractal curves.

use fractal_lib::SelectedFractal;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, HtmlElement, HtmlInputElement, PointerEvent};

mod chaosgame;
mod escapetime;
mod fractaldata;
mod turtle;

use fractaldata::{FractalConfig, SelectedFractalExt};

pub trait FractalAnimation {
    fn draw_one_frame(&mut self) -> bool;
    fn pixel_to_coordinate(&self, x: f64, y: f64) -> [f64; 2];
    fn zoom(&mut self, _x1: f64, _y1: f64, _x2: f64, _y2: f64) -> bool;
}

#[derive(Debug, PartialEq)]
enum FractalAnimationStatus {
    NotStarted,
    Animating,
    Paused,
    Done,
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
        current_animation_status: FractalAnimationStatus::NotStarted,
        cursor_coords: [0, 0],
        fractal_coords: None,
        zoom_start_coords: None,
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
    /// Whether the current animation is animating, paused, or done.
    current_animation_status: FractalAnimationStatus,
    cursor_coords: [i32; 2],
    fractal_coords: Option<[f64; 2]>,
    zoom_start_coords: Option<[i32; 2]>,
}

enum Msg {
    /// Indiactes that we should render the next frame of the animation
    AnimationFrameRequested,
    /// Indicates that a configuration field changed. It specifies the configuration field name, as
    /// well as the new value (currently they can only be integers)
    ConfigChanged(String, u64),
    /// Indicates which fractal was selected for configuration and eventually running.
    FractalSelected(String),
    /// Whether to start the animation for the currently selected fractal and configuration.
    RunClicked,
    /// Whether to pause the currently running animation.
    PauseClicked,
    /// Whether to resume the currentl running animation.
    ResumeClicked,
    CursorCoordsCanged(i32, i32),
    CursorDown,
    CursorUp,
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
            model.current_animation_status = FractalAnimationStatus::NotStarted;
        }
        Msg::ConfigChanged(input, new_value) => {
            model.current_config.apply_change(input, new_value);
            model.current_animation_status = FractalAnimationStatus::NotStarted;
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
            // If it's not currently animating, start the animation
            match model.current_animation_status {
                FractalAnimationStatus::Animating => {}
                _ => {
                    // There might already be an animation running. We don't want to double-up on
                    // the number of AnimationFrameRequested messages if that's the case.
                    model.current_animation_status = FractalAnimationStatus::Animating;
                    orders.after_next_render(|_| Msg::AnimationFrameRequested);
                }
            }
        }
        Msg::PauseClicked => {
            log::debug!("clicked pause");
            model.current_animation_status = FractalAnimationStatus::Paused;
        }
        Msg::ResumeClicked => {
            log::debug!("clicked pause");
            model.current_animation_status = FractalAnimationStatus::Animating;
            orders.after_next_render(|_| Msg::AnimationFrameRequested);
        }
        Msg::AnimationFrameRequested => {
            log::debug!("animation frame requested");
            match &mut model.current_animation {
                None => {}
                Some(animation) => {
                    // Animate until it says its done
                    if animation.draw_one_frame() {
                        // Request another frame only if the animation is still animating.
                        match model.current_animation_status {
                            FractalAnimationStatus::Animating => {
                                orders.after_next_render(|_| Msg::AnimationFrameRequested);
                            }
                            _ => {}
                        }
                    } else {
                        model.current_animation_status = FractalAnimationStatus::Done
                    }
                }
            }
        }
        Msg::CursorCoordsCanged(x, y) => {
            model.cursor_coords = [x, y];
            if let Some(animation) = &model.current_animation {
                let fractal_coords = animation.pixel_to_coordinate(x.into(), y.into());
                model.fractal_coords = Some(fractal_coords);
            }
        }
        Msg::CursorDown => {
            // Track where the cursor started being down, to begin selecting an area to zoom into.
            model.zoom_start_coords = Some(model.cursor_coords);
            log::debug!("Mouse down: {:?}", model.zoom_start_coords);
        }
        Msg::CursorUp => match model.zoom_start_coords {
            None => {}
            Some(zoom_start_coords) => {
                // If the cursor has been held down and was just raised up, compute the zoom and
                // resume the animation, if zoom is supported.
                let zoom_end_coords = model.cursor_coords;
                log::debug!("Mouse up: {:?}", zoom_end_coords);
                match &mut model.current_animation {
                    None => {}
                    Some(animation) => {
                        if animation.zoom(
                            f64::from(zoom_start_coords[0]),
                            f64::from(zoom_start_coords[1]),
                            f64::from(zoom_end_coords[0]),
                            f64::from(zoom_end_coords[1]),
                        ) {
                            // After `zoom()` is called, try to render the next frame. `zoom()`
                            // should prepare the animation.
                            orders.after_next_render(|_| Msg::AnimationFrameRequested);
                            model.current_animation_status = FractalAnimationStatus::Animating;
                        }
                    }
                }
            }
        },
    }
}

/// Renders/re-renders the HTML side of the UI
///
/// Note that updates to the canvas are handled by the currently ongoing animation.
fn view(model: &Model) -> Node<Msg> {
    log::debug!("view");
    div![
        div![
            canvas![
                el_ref(&model.canvas),
                attrs! { At::Id => "fractal-canvas", At::Width => "800", At::Height => "600"},
                style! {St::BackgroundColor => "white"},
                //  update the pointer location
                ev(Ev::PointerMove, |event| {
                    let pointer_event = event.dyn_into::<PointerEvent>().unwrap();
                    let target = pointer_event
                        .target()
                        .unwrap()
                        .dyn_into::<HtmlElement>()
                        .unwrap();

                    Msg::CursorCoordsCanged(
                        pointer_event.client_x() - target.offset_left(),
                        pointer_event.client_y() - target.offset_top(),
                    )
                }),
                ev(Ev::PointerDown, |_| Msg::CursorDown),
                ev(Ev::PointerUp, |_| Msg::CursorUp),
            ],
            div![
                attrs! {At::Id => "status"},
                format!("Status: {:?}", model.current_animation_status)
            ],
            div![
                attrs! {At::Id => "coords"},
                format!("Canvas coords: {:?}", model.cursor_coords)
            ],
            div![
                attrs! {At::Id => "fractal-coords"},
                IF!(not(model.fractal_coords.is_none()) => format!(
                    "Fractal coords: X: {}, Y: {}",
                    model.fractal_coords.unwrap()[0], model.fractal_coords.unwrap()[1]
                )),
                IF!(model.fractal_coords.is_none() => "Fractal coords: No fractal being rendered")
            ],
        ],
        div![
            view_menu(model.current_animation_status == FractalAnimationStatus::Animating),
            view_config(&model.current_config),
            IF!(
                model.current_animation_status == FractalAnimationStatus::Animating => button![
                "Pause", ev(Ev::Click, |_| Msg::PauseClicked)]
            ),
            IF!(
                model.current_animation_status == FractalAnimationStatus::Paused => button![
                "Resume", ev(Ev::Click, |_| Msg::ResumeClicked)]
            ),
            IF!(
                model.current_animation_status != FractalAnimationStatus::Animating => div![
                    button!["Start a new fractal", ev(Ev::Click, |_| Msg::RunClicked)]
                ]
            ),
        ]
    ]
}

/// Renders/re-renders the menu for selecting which fractal animation to configure and run next.
///
/// The menu can be disabled -- while an animation is running, it causes seed to re-compute the
/// view (the animation uses the seed event loop to schedule the next animation frame after the
/// next render), which causes the dropdown menu beecome unusable.
fn view_menu(disable: bool) -> Node<Msg> {
    log::debug!("menu disabled: {}", disable);
    select![
        attrs! { At::Id => "fractal-type" },
        // the presence of `disabled`, not its value, determines if an element is disabled.
        IF!(disable => attrs!{ At::Disabled => disable }),
        SelectedFractal::by_category()
            .iter()
            .map(|(category, fractals)| {
                optgroup![
                    attrs! {At::Label => category.display_name()},
                    fractals.iter().map(|fractal| {
                        option![
                            attrs! {At::Value => <&'static str>::from(fractal)},
                            fractal.name()
                        ]
                    })
                ]
            }),
        input_ev(Ev::Input, Msg::FractalSelected),
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
        if let Ok(value) = target.value().parse::<u64>() {
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
            FractalConfig::TurtleCurveConfig { iteration } => div![
                div![
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
                ],
                p![
                    "Draws the fractal using a turtle animation."
                ],
            ],
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
                p![
                    "Renders the escape time fractal using the provided parameters. After the fractal renders, you can use a pointer to select an area to zoom in on."
                ],
            ],
        },
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
