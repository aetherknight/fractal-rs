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
use fractal_lib::FractalCategory;
use fractal_lib::SelectedFractal;
use wasm_bindgen::prelude::JsValue;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::FractalAnimation;
use super::{chaosgame, escapetime, turtle};

/// Extend SelectedFractal for fractal-wasm
pub trait SelectedFractalExt {
    fn default_config(self) -> FractalConfig;
    fn build_animation(
        self,
        canvas: &HtmlCanvasElement,
        config: &FractalConfig,
    ) -> Box<dyn FractalAnimation>;
}

impl SelectedFractalExt for SelectedFractal {
    /// Returns the initial/default configuration for the given fractal.
    fn default_config(self) -> FractalConfig {
        match self.category() {
            FractalCategory::ChaosGames => FractalConfig::NoConfig,
            FractalCategory::TurtleCurves => FractalConfig::TurtleCurveConfig { iteration: 1 },
            FractalCategory::EscapeTimeFractals => FractalConfig::EscapeTimeConfig {
                max_iterations: 100,
                power: 2,
            },
        }
    }

    fn build_animation(
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
pub enum FractalConfig {
    NoConfig,
    EscapeTimeConfig { max_iterations: u64, power: u64 },
    TurtleCurveConfig { iteration: u64 },
}

impl FractalConfig {
    pub fn apply_change(&mut self, field: String, new_value: u64) {
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

/// Macro that generates a function for constructing a TurtleAnimation for a particular kind of
/// turtle-based curve.
///
/// It takes a name identifier, a colon, and then expression that should evaluate to a
/// TurtleProgram. The expression may use `iteration` in order to configure the TurtleProgram.
///
/// For example:
/// ```rust,ignore
/// animated_turtle!(dragon: dragon::DragonFractal::new(iteration));
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
            pub fn [<animated_ $name>] (
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
        LindenmayerSystemTurtleProgram::new(cesaro::CesaroFractal::new(iteration))
    }
);
animated_turtle!(
    cesarotri: |iteration| {
        LindenmayerSystemTurtleProgram::new(cesarotri::CesaroTriFractal::new(iteration))
    }
);
animated_turtle!(
    dragon: |iteration| {
        dragon::DragonFractal::new(iteration)
    }
);
animated_turtle!(
    kochcurve: |iteration| {
        LindenmayerSystemTurtleProgram::new(kochcurve::KochCurve::new(iteration))
    }
);
animated_turtle!(
    levyccurve: |iteration| {
        LindenmayerSystemTurtleProgram::new(levyccurve::LevyCCurve::new(iteration))
    }
);
animated_turtle!(
    terdragon: |iteration| {
        LindenmayerSystemTurtleProgram::new(terdragon::TerdragonFractal::new(iteration))
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
            pub fn [<animated_ $name>] (
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
        BurningMandel::new(max_iterations, power)
    }
);
animated_escape_time!(
    burningship: |max_iterations, power| {
        BurningShip::new(max_iterations, power)
    }
);
animated_escape_time!(
    mandelbrot: |max_iterations, power| {
        Mandelbrot::new(max_iterations, power)
    }
);
animated_escape_time!(
    roadrunner: |max_iterations, power| {
        RoadRunner::new(max_iterations, power)
    }
);
