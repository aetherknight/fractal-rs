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
use fractal_lib::chaosgame::barnsleyfern;
use fractal_lib::chaosgame::sierpinski;
use fractal_lib::curves::cesaro;
use fractal_lib::curves::cesarotri;
use fractal_lib::curves::dragon;
use fractal_lib::curves::kochcurve;
use fractal_lib::curves::levyccurve;
use fractal_lib::curves::terdragon;
use fractal_lib::escapetime::burningship::{BurningMandel, BurningShip, RoadRunner};
use fractal_lib::escapetime::mandelbrot::Mandelbrot;
use fractal_lib::lindenmayer::LindenmayerSystemTurtleProgram;
use js_sys::Array;
use log;
use paste;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString, IntoStaticStr};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

mod chaosgame;
mod escapetime;
mod turtle;

// #[wasm_bindgen(start)]
// pub fn start() {
//     console_error_panic_hook::set_once();
//     console_log::init_with_level(log::Level::Debug).unwrap();
// }
//

pub trait FractalAnimation {
    fn draw_one_frame(&mut self) -> bool;
    fn pixel_to_coordinate(&self, x: f64, y: f64) -> Array;
    fn zoom(&mut self, _x1: f64, _y1: f64, _x2: f64, _y2: f64) -> bool;
}

/// Macro that generates a function for constructing a TurtleAnimation for a particular kind of
/// turtle-based curve.
///
/// It takes a name identifier, a colon, and then expression that should evaluate to a
/// TurtleProgram. The expression may use `iteration` in order to configure the TurtleProgram.
///
/// For example:
/// ```rust,ignore
/// animated_turtle!(dragon: dragon::DragonFractal::new(u64::from(iteration)));
/// ```
///
/// Will create a function with signature:
///
/// ```rust, ignore
/// pub fn animated_dragon(canvas: &HtmlCanvaselement, config: &FractalConfig) -> turtle::TurtleAnimation;
/// ```
///
/// It will blank out the screen, start the TurtleAnimation, and then return it. The caller may
/// then call `draw_one_frame` on future frames/ticks to update/animate the canvas.
macro_rules! animated_turtle {
    ($name:ident: $expr:expr) => {
        // Paste is needed to concatenate render_ and the name of the fractal. Rust's own macros
        // don't provide a good way to do this.
        paste::item! {
            /// Blanks the canvas and constructs a `TurtleAnimation` that represents the given
            /// curve, and then returns the `TurtleAnimation`, allowing the call to render
            /// additional frames.
            ///
            /// The iteration specifies which iteration of the TurtleProgram it will draw.
            fn [<animated_ $name>] (
                canvas: &HtmlCanvasElement,
                config: &FractalConfig,
            ) -> turtle::TurtleAnimation {
                match config {
                    FractalConfig::TurtleCurveConfig{iteration: iteration} => {
                        log::debug!("Starting animation {}", stringify!($name));
                        let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                            .dyn_into::<CanvasRenderingContext2d>()
                            .unwrap();
                        ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                        let program = $expr;
                        turtle::TurtleAnimation::new(ctx, &(program(*iteration)))
                    },
                    _ => { panic!("{} needs a TurtleCurveConfig", stringify!($name)) },
                }
            }
        }
    };
}

animated_turtle!(
    cesaro: |iteration| {
        LindenmayerSystemTurtleProgram::new(cesaro::CesaroFractal::new(u64::from(iteration)))
    }
);
animated_turtle!(
    cesarotri: |iteration| {
        LindenmayerSystemTurtleProgram::new(cesarotri::CesaroTriFractal::new(u64::from(iteration)))
    }
);
animated_turtle!(
    dragon: |iteration| {
        dragon::DragonFractal::new(u64::from(iteration))
    }
);
animated_turtle!(
    kochcurve: |iteration| {
        LindenmayerSystemTurtleProgram::new(kochcurve::KochCurve::new(u64::from(iteration)))
    }
);
animated_turtle!(
    levyccurve: |iteration| {
        LindenmayerSystemTurtleProgram::new(levyccurve::LevyCCurve::new(u64::from(iteration)))
    }
);
animated_turtle!(
    terdragon: |iteration| {
        LindenmayerSystemTurtleProgram::new(terdragon::TerdragonFractal::new(u64::from(iteration)))
    }
);

/// Macro that generates a function for constructing (and starting) a ChaosGameAnimation for a
/// particular kind of ChaosGame.
///
/// It takes a name identifier, a colon, and then expression that should evaluate to a
/// a ChaosGame.
///
/// For example:
/// ```rust,ignore
/// animated_chaos_game!(sierpinski: sierpinski::SierpinskiChaosGame::new());
/// ```
///
/// Will create a function with signature:
///
/// ```rust,ignore
/// #[wasm_bindgen]
/// pub fn animated_sierpinski(canvas: &HtmlCanvaselement) -> ChaosGameAnimation;
/// ```
///
/// It will blank out the screen, start the ChaosGameAnimation, and then return it. The caller may
/// then call `draw_one_frame` on future frames/ticks to update/animate the canvas.
macro_rules! animated_chaos_game {
    ($name:ident: $expr:expr) => {
        // Paste is needed to concatenate render_ and the name of the fractal. Rust's own macros
        // don't provide a good way to do this.
        paste::item! {
            fn [<animated_ $name>] (
                canvas: &HtmlCanvasElement
            ) -> chaosgame::ChaosGameAnimation {
                log::debug!("Starting animation {}", stringify!($name));
                let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                chaosgame::ChaosGameAnimation::new(ctx, Box::new($expr))
            }
        }
    };
}

animated_chaos_game!(
    barnsleyfern:
        barnsleyfern::BarnsleyFern::new(
            &barnsleyfern::REFERENCE_TRANSFORMS,
            &barnsleyfern::REFERENCE_WEIGHTS,
        )
);

animated_chaos_game!(sierpinski: sierpinski::SierpinskiChaosGame::new());

macro_rules! animated_escape_time {
    ($name:ident: $expr:expr) => {
        // Paste is needed to concatenate render_ and the name of the fractal. Rust's own macros
        // don't provide a good way to do this.
        paste::item! {
            fn [<animated_ $name>] (
                canvas: &HtmlCanvasElement,
                config: &FractalConfig,
            ) -> escapetime::EscapeTimeAnimation {
                match config {
                    FractalConfig::EscapeTimeConfig{max_iterations: max_iterations, power: power} => {
                        log::debug!("Starting animation {}", stringify!($name));
                        let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                            .dyn_into::<CanvasRenderingContext2d>()
                            .unwrap();

                        ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                        let fractal = $expr;
                        escapetime::EscapeTimeAnimation::new(ctx, Box::new(fractal(*max_iterations, *power)))
                    },
                    _ => { panic!("{} needs a EscapeTimeconfig", stringify!($name)) },
                }
            }
        }
    };
}

animated_escape_time!(
    burningmandel: |max_iterations, power| {
        BurningMandel::new(u64::from(max_iterations), u64::from(power))
    }
);
animated_escape_time!(
    burningship: |max_iterations, power| {
        BurningShip::new(u64::from(max_iterations), u64::from(power))
    }
);
animated_escape_time!(
    mandelbrot: |max_iterations, power| {
        Mandelbrot::new(u64::from(max_iterations), u64::from(power))
    }
);
animated_escape_time!(
    roadrunner: |max_iterations, power| {
        RoadRunner::new(u64::from(max_iterations), u64::from(power))
    }
);

#[derive(Copy, Clone)]
enum FractalCategory {
    ChaosGames,
    EscapeTimeFractals,
    TurtleCurves,
}

// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
// #![allow(clippy::wildcard_imports)]
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

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

// ------ ------
//     Model
// ------ ------

struct Model {
    canvas: ElRef<HtmlCanvasElement>,
    selected_fractal: SelectedFractal,
    current_config: FractalConfig,
    current_animation: Option<Box<dyn FractalAnimation>>,
    animation_ongoing: bool,
}

/// All of the support fractals, with associated data for using them.
///
/// You can list all of them by using a derived iterator:
///
/// ```rust
/// use strum::IntoEnumIterator;
///
/// SelectedFractal::iter()
/// ```
///
/// You can parse a string token into one of these enums using something like:
///
/// ```rust
/// use std::str::FromStr;
///
/// SelectedFractal::from_str("Dragon").unwrap()
/// ```
///
/// You can generate a static str representation using:
///
/// ```rust.ignore
/// <&'static str>::from(SelectedFractal::Dragon)
/// ```
#[derive(Copy, Clone, EnumString, EnumIter, IntoStaticStr)]
enum SelectedFractal {
    BarnsleyFern,
    BurningMandel,
    BurningShip,
    Cesaro,
    CesaroTri,
    Dragon,
    KochCurve,
    LevyCCurve,
    Mandelbrot,
    RoadRunner,
    Sierpinski,
    TerDragon,
}

impl SelectedFractal {
    pub fn name(self) -> &'static str {
        match self {
            SelectedFractal::BarnsleyFern => "Barnsley Fern",
            SelectedFractal::BurningMandel => "Burning Mandel",
            SelectedFractal::BurningShip => "Burning Ship",
            SelectedFractal::Cesaro => "Cesàro",
            SelectedFractal::CesaroTri => "Cesàro Triangle",
            SelectedFractal::Dragon => "Dragon",
            SelectedFractal::KochCurve => "Koch Curve",
            SelectedFractal::LevyCCurve => "Lévy C Curve",
            SelectedFractal::Mandelbrot => "Mandelbrot",
            SelectedFractal::RoadRunner => "Roadrunner",
            SelectedFractal::Sierpinski => "Sierpiński Triangle",
            SelectedFractal::TerDragon => "Terdragon",
        }
    }

    pub fn category(self) -> FractalCategory {
        match self {
            SelectedFractal::BarnsleyFern => FractalCategory::ChaosGames,
            SelectedFractal::BurningMandel => FractalCategory::EscapeTimeFractals,
            SelectedFractal::BurningShip => FractalCategory::EscapeTimeFractals,
            SelectedFractal::Cesaro => FractalCategory::TurtleCurves,
            SelectedFractal::CesaroTri => FractalCategory::TurtleCurves,
            SelectedFractal::Dragon => FractalCategory::TurtleCurves,
            SelectedFractal::KochCurve => FractalCategory::TurtleCurves,
            SelectedFractal::LevyCCurve => FractalCategory::TurtleCurves,
            SelectedFractal::Mandelbrot => FractalCategory::EscapeTimeFractals,
            SelectedFractal::RoadRunner => FractalCategory::EscapeTimeFractals,
            SelectedFractal::Sierpinski => FractalCategory::ChaosGames,
            SelectedFractal::TerDragon => FractalCategory::TurtleCurves,
        }
    }

    /// Returns the initial/default configuration for the given fractal.
    pub fn default_config(self) -> FractalConfig {
        match self.category() {
            FractalCategory::ChaosGames => FractalConfig::NoConfig,
            FractalCategory::TurtleCurves => FractalConfig::TurtleCurveConfig { iteration: 1 },
            FractalCategory::EscapeTimeFractals => FractalConfig::EscapeTimeConfig {
                max_iterations: 100,
                power: 2,
            },
        }
    }

    pub fn build_animation(
        self,
        canvas: &HtmlCanvasElement,
        config: &FractalConfig,
    ) -> Box<dyn FractalAnimation> {
        match self {
            SelectedFractal::BarnsleyFern => Box::new(animated_barnsleyfern(canvas)),
            SelectedFractal::BurningMandel => Box::new(animated_burningmandel(canvas, config)),
            SelectedFractal::BurningShip => Box::new(animated_burningship(canvas, config)),
            SelectedFractal::Cesaro => Box::new(animated_cesaro(canvas, config)),
            SelectedFractal::CesaroTri => Box::new(animated_cesarotri(canvas, config)),
            SelectedFractal::Dragon => Box::new(animated_dragon(canvas, config)),
            SelectedFractal::KochCurve => Box::new(animated_kochcurve(canvas, config)),
            SelectedFractal::LevyCCurve => Box::new(animated_levyccurve(canvas, config)),
            SelectedFractal::Mandelbrot => Box::new(animated_mandelbrot(canvas, config)),
            SelectedFractal::RoadRunner => Box::new(animated_roadrunner(canvas, config)),
            SelectedFractal::Sierpinski => Box::new(animated_sierpinski(canvas)),
            SelectedFractal::TerDragon => Box::new(animated_terdragon(canvas, config)),
        }
    }
}

#[derive(Debug)]
enum FractalConfig {
    NoConfig,
    EscapeTimeConfig { max_iterations: u32, power: u32 },
    TurtleCurveConfig { iteration: u32 },
}

impl FractalConfig {
    pub fn apply_change(&mut self, field: String, new_value: u32) {
        log::debug!("apply_change {:?}", self);
        match self {
            FractalConfig::NoConfig => panic!("{:?} does not have a {}", self, field),
            FractalConfig::EscapeTimeConfig {
                ref mut max_iterations,
                ref mut power,
            } => {
                match field.as_str() {
                    "max_iterations" => *max_iterations = new_value,
                    "power" => *power = new_value,
                    _ => panic!("{:?} does not have a {}", self, field),
                };
                log::debug!("{:?}", self);
            }
            FractalConfig::TurtleCurveConfig { ref mut iteration } => match field.as_str() {
                "iteration" => *iteration = new_value,
                _ => panic!("{:?} does not have a {}", self, field),
            },
        }
    }
}

// ------ ------
//    Update
// ------ ------

// `Msg` describes the different events you can modify state with.
enum Msg {
    FractalSelected(String),
    RunClicked,
    AnimationFrameRequested,
    ConfigChanged(String, u32),
}

// `update` describes how to handle each `Msg`.
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

// ------ ------
//     View
// ------ ------

// (Remove the line below once your `Model` become more complex.)
#[allow(clippy::trivially_copy_pass_by_ref)]
// `view` describes what to display.
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
        // ev(Ev::Change, |event| log::debug!("change {:?}", event)),
        // ev(Ev::Focus, |event| log::debug!("focus {:?}", event)),
        // ev(Ev::Blur, |event| log::debug!("blur {:?}", event)),
    ]
}

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

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).unwrap();
    log::debug!("Start App");
    // Mount the `app` to the element with the `id` "app".
    App::start("app", init, update, view);
}
