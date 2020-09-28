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
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

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
/// #[wasm_bindgen]
/// pub fn animated_dragon(canvas: &HtmlCanvaselement, iteration: u32) -> turtle::TurtleAnimation;
/// ```
///
/// It will blank out the screen, start the TurtleAnimation, and then return it. The caller may
/// then call `draw_one_frame` on future frames/ticks to update/animate the canvas.
///
/// Note: `iteration` is a u32 in the function signature (and not a u64) because as of 2019/06/08,
/// wasm-bindgen uses BigUint64Array to help pass around 64-bit unsigned integers, but
/// BigUint64Array has not yet been standardized/ Firefox 67 does not yet support BigUint64Array or
/// bigints.
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
            #[wasm_bindgen]
            pub fn [<animated_ $name>] (
                canvas: &HtmlCanvasElement,
                iteration: u32
            ) -> turtle::TurtleAnimation {
                log::debug!("Starting animation {}", stringify!($name));
                let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                let program = $expr;
                ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                turtle::TurtleAnimation::new(ctx, &program)
            }
        }
    };
}

animated_turtle!(
    cesaro: LindenmayerSystemTurtleProgram::new(cesaro::CesaroFractal::new(u64::from(iteration)))
);
animated_turtle!(
    cesarotri:
        LindenmayerSystemTurtleProgram::new(cesarotri::CesaroTriFractal::new(u64::from(iteration)))
);
animated_turtle!(dragon: dragon::DragonFractal::new(u64::from(iteration)));
animated_turtle!(
    kochcurve: LindenmayerSystemTurtleProgram::new(kochcurve::KochCurve::new(u64::from(iteration)))
);
animated_turtle!(
    levyccurve:
        LindenmayerSystemTurtleProgram::new(levyccurve::LevyCCurve::new(u64::from(iteration)))
);
animated_turtle!(
    terdragon:
        LindenmayerSystemTurtleProgram::new(terdragon::TerdragonFractal::new(u64::from(iteration)))
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
            #[wasm_bindgen]
            pub fn [<animated_ $name>] (
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
            #[wasm_bindgen]
            pub fn [<animated_ $name>] (
                canvas: &HtmlCanvasElement, max_iterations: u32, power: u32
            ) -> escapetime::EscapeTimeAnimation {
                log::debug!("Starting animation {}", stringify!($name));
                let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();

                ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

                escapetime::EscapeTimeAnimation::new(ctx, Box::new($expr))
            }
        }
    };
}

animated_escape_time!(
    burningmandel: BurningMandel::new(u64::from(max_iterations), u64::from(power))
);
animated_escape_time!(burningship: BurningShip::new(u64::from(max_iterations), u64::from(power)));
animated_escape_time!(mandelbrot: Mandelbrot::new(u64::from(max_iterations), u64::from(power)));
animated_escape_time!(roadrunner: RoadRunner::new(u64::from(max_iterations), u64::from(power)));

#[derive(Copy, Clone)]
enum FractalCategory {
    ChaosGames,
    EscapeTimeFractals,
    LindenmayerCurves,
    OtherCurves,
}

// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
// #![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.
fn init(_url: Url, _orders: &mut impl Orders<Msg>) -> Model {
    Model {
        canvas: ElRef::default(),
        selected_fractal: SelectedFractal::BarnsleyFern,
        current_animation: None,
        animation_ongoing: false,
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    selected_fractal: SelectedFractal,
    canvas: ElRef<HtmlCanvasElement>,
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
            SelectedFractal::Cesaro => FractalCategory::LindenmayerCurves,
            SelectedFractal::CesaroTri => FractalCategory::LindenmayerCurves,
            SelectedFractal::Dragon => FractalCategory::OtherCurves,
            SelectedFractal::KochCurve => FractalCategory::LindenmayerCurves,
            SelectedFractal::LevyCCurve => FractalCategory::LindenmayerCurves,
            SelectedFractal::Mandelbrot => FractalCategory::EscapeTimeFractals,
            SelectedFractal::RoadRunner => FractalCategory::EscapeTimeFractals,
            SelectedFractal::Sierpinski => FractalCategory::ChaosGames,
            SelectedFractal::TerDragon => FractalCategory::LindenmayerCurves,
        }
    }

    pub fn build_animation(self, canvas: &HtmlCanvasElement) -> Box<dyn FractalAnimation> {
        match self {
            SelectedFractal::BarnsleyFern => Box::new(animated_barnsleyfern(canvas)),
            SelectedFractal::BurningMandel => Box::new(animated_burningmandel(canvas, 100, 2)),
            SelectedFractal::BurningShip => Box::new(animated_burningship(canvas, 100, 2)),
            SelectedFractal::Cesaro => Box::new(animated_cesaro(canvas, 5)),
            SelectedFractal::CesaroTri => Box::new(animated_cesarotri(canvas, 5)),
            SelectedFractal::Dragon => Box::new(animated_dragon(canvas, 5)),
            SelectedFractal::KochCurve => Box::new(animated_kochcurve(canvas, 5)),
            SelectedFractal::LevyCCurve => Box::new(animated_levyccurve(canvas, 5)),
            SelectedFractal::Mandelbrot => Box::new(animated_mandelbrot(canvas, 100, 2)),
            SelectedFractal::RoadRunner => Box::new(animated_roadrunner(canvas, 100, 2)),
            SelectedFractal::Sierpinski => Box::new(animated_sierpinski(canvas)),
            SelectedFractal::TerDragon => Box::new(animated_terdragon(canvas, 5)),
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
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FractalSelected(selected) => {
            log::debug!("selected {}", selected);
            model.selected_fractal =
                SelectedFractal::from_str(selected.as_ref()).expect("unknown selected fractal");
        }
        Msg::RunClicked => {
            log::debug!("clicked run");
            // Initialize a new animation
            let canvas = model.canvas.get().expect("get canvas element failed");
            model.current_animation = Some(model.selected_fractal.build_animation(&canvas));
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
            div![attrs! {At::Id => "configs"}],
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
        ev(Ev::Change, |event| log::debug!("change {:?}", event)),
        ev(Ev::Focus, |event| log::debug!("focus {:?}", event)),
        ev(Ev::Blur, |event| log::debug!("blur {:?}", event)),
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
