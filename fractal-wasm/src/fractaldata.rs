use fractal_lib::chaosgame::barnsleyfern;
use fractal_lib::chaosgame::sierpinski;
use fractal_lib::chaosgame::ChaosGameMoveIterator;
use fractal_lib::curves::cesaro;
use fractal_lib::curves::cesarotri;
use fractal_lib::curves::dragon;
use fractal_lib::curves::kochcurve;
use fractal_lib::curves::levyccurve;
use fractal_lib::curves::terdragon;
use fractal_lib::escapetime::burningship::{BurningMandel, BurningShip, RoadRunner};
use fractal_lib::escapetime::mandelbrot::Mandelbrot;
use fractal_lib::escapetime::EscapeTime;
use fractal_lib::lindenmayer::LindenmayerSystemTurtleProgram;
use fractal_lib::turtle::TurtleProgram;
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
            SelectedFractal::BarnsleyFern => Box::new(animated_chaos_game(
                canvas,
                &|| {
                    barnsleyfern::BarnsleyFern::new(
                        &barnsleyfern::REFERENCE_TRANSFORMS,
                        &barnsleyfern::REFERENCE_WEIGHTS,
                    )
                },
                self.name(),
            )),
            SelectedFractal::BurningMandel => Box::new(animated_escape_time(
                canvas,
                config,
                &BurningMandel::new,
                self.name(),
            )),
            SelectedFractal::BurningShip => Box::new(animated_escape_time(
                canvas,
                config,
                &BurningShip::new,
                self.name(),
            )),
            SelectedFractal::Cesaro => Box::new(animated_turtle(
                canvas,
                config,
                &LindenmayerSystemTurtleProgram::build(cesaro::CesaroFractal::new),
                self.name(),
            )),
            SelectedFractal::CesaroTri => Box::new(animated_turtle(
                canvas,
                config,
                &LindenmayerSystemTurtleProgram::build(cesarotri::CesaroTriFractal::new),
                self.name(),
            )),
            SelectedFractal::Dragon => Box::new(animated_turtle(
                canvas,
                config,
                &dragon::DragonFractal::new,
                self.name(),
            )),
            SelectedFractal::KochCurve => Box::new(animated_turtle(
                canvas,
                config,
                &LindenmayerSystemTurtleProgram::build(kochcurve::KochCurve::new),
                self.name(),
            )),
            SelectedFractal::LevyCCurve => Box::new(animated_turtle(
                canvas,
                config,
                &LindenmayerSystemTurtleProgram::build(levyccurve::LevyCCurve::new),
                self.name(),
            )),
            SelectedFractal::Mandelbrot => Box::new(animated_escape_time(
                canvas,
                config,
                &Mandelbrot::new,
                self.name(),
            )),
            SelectedFractal::RoadRunner => Box::new(animated_escape_time(
                canvas,
                config,
                &RoadRunner::new,
                self.name(),
            )),
            SelectedFractal::Sierpinski => Box::new(animated_chaos_game(
                canvas,
                &sierpinski::SierpinskiChaosGame::new,
                self.name(),
            )),
            SelectedFractal::TerDragon => Box::new(animated_turtle(
                canvas,
                config,
                &LindenmayerSystemTurtleProgram::build(terdragon::TerdragonFractal::new),
                self.name(),
            )),
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

fn animated_turtle<E, F>(
    canvas: &HtmlCanvasElement,
    config: &FractalConfig,
    ctor: &F,
    name: &'static str,
) -> turtle::TurtleAnimation
where
    E: TurtleProgram + 'static,
    F: Fn(u64) -> E,
{
    match config {
        FractalConfig::TurtleCurveConfig { iteration } => {
            log::debug!("Starting animation {}", name);
            let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
            ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

            turtle::TurtleAnimation::new(ctx, &(ctor(*iteration)))
        }
        _ => panic!("{} needs a TurtleCurveConfig", stringify!($name)),
    }
}

fn animated_chaos_game<E, F>(
    canvas: &HtmlCanvasElement,
    ctor: &F,
    name: &'static str,
) -> chaosgame::ChaosGameAnimation
where
    E: ChaosGameMoveIterator + 'static,
    F: Fn() -> E,
{
    log::debug!("Starting animation {}", name);
    let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

    chaosgame::ChaosGameAnimation::new(ctx, Box::new(ctor()))
}

fn animated_escape_time<E, F>(
    canvas: &HtmlCanvasElement,
    config: &FractalConfig,
    ctor: &F,
    name: &'static str,
) -> escapetime::EscapeTimeAnimation
where
    E: EscapeTime + 'static,
    F: Fn(u64, u64) -> E,
{
    match config {
        FractalConfig::EscapeTimeConfig {
            max_iterations,
            power,
        } => {
            log::debug!("Starting animation {}", name);
            let ctx = JsValue::from(canvas.get_context("2d").unwrap().unwrap())
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            ctx.clear_rect(0.0, 0.0, canvas.width().into(), canvas.height().into());

            escapetime::EscapeTimeAnimation::new(ctx, Box::new(ctor(*max_iterations, *power)))
        }
        _ => panic!("{} needs a EscapeTimeconfig", stringify!($name)),
    }
}
