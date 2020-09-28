# fractal-rs

A Rust application (and library) for exploring a variety of fractals and for me
to learn more about Rust and its ecosystem.

It now contains a shared library that contains general purpose code and
definitions for various kinds of fractals (`fractal-lib`), a piston-based
renderer that can be run from the command line (`fractal-postion`), and a
WASM-based target for rendering fractals in web browsers (`fractal-wasm`).


## Subprojects

### `fractal-lib`

The shared library contains definitions for all of the supported fractals, plus
some core code/interfaces for implementing turtle programs (the frontends then
implement turtles that can run these programs), and [Lindenmeyer
systems](https://en.wikipedia.org/wiki/L-system). It also contains modules to
support colors and geometry used by some of the fractals.

* Curves supported:
    * [Cesàro square fractal (torn fractal)](http://mathworld.wolfram.com/CesaroFractal.html)
    * Cesàro triangle fractal (with angles calculated to prevent overlapping
      line segments)
    * [Dragon curve](https://en.wikipedia.org/wiki/Dragon_curve)
    * [Koch snowflake](https://en.wikipedia.org/wiki/Koch_snowflake)
    * [Lévy C curve](https://en.wikipedia.org/wiki/L%C3%A9vy_C_curve)
    * Terdragon fractal
* Chaos game images supported:
    * [Barnsley fern](https://en.wikipedia.org/wiki/Barnsley_fern)
    * [Sierpiński triangle](https://en.wikipedia.org/wiki/Sierpi%C5%84ski_triangle)
* Escape time fractals, with support for shading/color, and zooming in and out.
  Supported families of escape time fractals include:
    * [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) with
      generalized support for some
      [multibrot sets](https://en.wikipedia.org/wiki/Multibrot_set)
    * [Burning ship fractal](https://en.wikipedia.org/wiki/Burning_Ship_fractal)
      with generalized support for some [related power
      sets](https://theory.org/fracdyn/burningship/symmetry.html)

### `fractal-wasm`

TODO: link to a hosted page for `fractal-wasm`.


### `fractal-piston`

A minimalistic piston-based UI that can be run from the command line.

#### Usage

Fetch the git repository, and then use cargo to build it:

```sh
git clone https://github.com/aetherknight/fractal-rs.git
cd fractal-rs
cargo build
```
To run the application, you can use `cargo run` or call the binary directly
(Eg, if you use `cargo install` to install the program). For command usage
information, try:

```sh
cargo run -- help
```

For example, to open a window and draw iteration 4 of the Cesàro square fractal:

```sh
cargo run -- cesaro 4
```

You can exit by closing the window or pressing the `esc` key.

To draw the animation of a curve faster, you can use the `--drawrate` option.
You must specify the number of line segments (or points) that should be drawn
per frame as well, such as `1` for one line per frame (usually the defualt).
The following will animate iteration 11 of the dragon fractal at a rate of 10
line segments per frame:

```sh
cargo run -- dragon 11 --drawrate 10
```

Note that for most fractals the iteration number results in an exponential
increase in computation, so if you want to explore a higher
iteration/generation of a curve, you may want to start with a low iteration
number and increment your way up.

#### Exploring Fractals

The fractal program includes the following subcommands:

| Subcommand | Description |
| ---------- | ----------- |
| `barnsleyfern [--drawrate MPF]` | Draws the Barnsley Fern fractal using a chaos game with affine transforms. |
| `burningmandel MAX_IT POWER` | Draws a variation of the burning ship fractal |
| `burningship MAX_IT POWER` | Draws the burning ship fractal |
| `cesaro [--drawrate MPF] ITER` | Draws a square Cesàro fractal |
| `cestarotri [--drawrate MPF] ITER` | Draws a triangle Cesàro fractal |
| `dragon [--drawrate MPF] ITER` | Draws a dragon curve fractal |
| `kochcurve [--drawrate MPF] ITER` | Draws a Koch snowflake curve |
| `levyccurve [--drawrate MPF] ITER` | Draws a Lévy C Curve |
| `mandelbrot MAX_IT POWER` | Draws the mandelbrot fractal |
| `roadrunner MAX_IT POWER` | Draws a variation of the burning ship fractal |
| `sierpinski [--drawrate MPF]` | Draws a Sierpiński triangle using a chaos game and 3 randomly chosen points on the screen |
| `terdragon [--drawrate MPF] ITER` | Draws a terdragon curve |

Where the arguments have the following meaning:

| Argument | Description |
| -------- | ----------- |
| `ITER` | The iteration of the curve to draw |
| `MPF` | The number of lines or points to draw per frame [default: 1] |
| `MAX_IT` | The maximum number of iterations of the escape time function before deciding the fracal has escaped |
| `POWER` | The exponent used in the escape time function (positive integer) |

The chaos game and turtle-drawn curves are not particularly interactive. If you
resize the screen, they will redraw themselves (the Sierpiński triangle will
pick 3 new random points as vertices for the triangle).

The escape-time fractals (`burningmandel`, `burningship`, `mandelbrot`, and
`roadrunner`) support a greater degree of interactivity:

* You can select an area of the fractal to zoom in on using a cursor/mouse
* Resizing the window will keep the current view instead of resetting to the
  initial zoom/view
* backspace (delete) will reset the view area back to the initial/default view
  of the fractal
* Arrow keys can be used to move the view area around


## Future ideas

Some future ideas (in no particular order):

* Option to automatically profile and adjust how much can be animated per-frame
  based on system performance for curves and chaos games.
    * The native/piston based rendered automatically scales up the number of
      threads it uses for the scape time fractal renders, but the curves and
      chaos games require manually choosing how lines or points to draw per
      frame.
* Display information about the current fractal.
* Greater interactivity, maybe a UI for choosing and configuring which fractal
  to display, or arrow keys to increment/decrement the iteration number.
* Customizable color for some curves
* Ability to export images or animations
* Dynamically specify more parameters through configuration instead of
  compiling them in, or support some sort of configuration format for
  specifying parameters.
* Other kinds of fractals like Julia/Fatou sets, etc.
* Explore using generators for turtle programs once generators are stable in
  Rust to simplify the keeping of turtle state.
* Explore using threads+channels for turtle programs, allowing for
  coroutine-like behavior and avoid having to box+wrap an underlying iterator.
  However, this would likely require different implementations for native
  (piston) implementations vs web/WASM.
* `fractal-wasm`: Offload the rendering of escape-time fractals from the main
  thread, and/or look into parallelizing its rendering. This may become more
  feasible once OffscreenCanvas is more widely supported by browsers (it has
  Chrome support and experimental Firefox support at the time of this writing).
  (WebGL might be another option, but would be more an exercise in WebGL than
  an exercise in WASM).


## Contributing

* If you have a feature request and would like to discuss it, please open a
  ticket.
* If you would like to implement a feature, feel free to submit a pull request
  or open a ticket to discuss followed by a pull request.
* If I receive many contributions, I will shift the project's copyright
  structure to reference a contributers file.

A few rules about contributed code:

* Contributions should work on the current stable release of Rust. If you want
  to use a nightly Rust feature, we should discuss the approach (eg, can it be
  made optional, does it provide a huge performance improvement, etc.).
* Use [rustfmt](https://github.com/rust-lang-nursery/rustfmt) to format your
  code before committing.
* Keep all `use` statements together and let `rustfmt` keep them sorted.
* Take care of compiler and clippy lints before merging.
* Write tests where it makes sense to do so (ie, test behaviors and
  functionality that could change as a side-effect of some other change, and
  test highly numeric code), but do not fret about it.


## License

Copyright (c) 2015-2019 William (B.J.) Snow Orvis. Licensed under the Apache
License, Version 2.0. See [LICENSE](LICENSE) for details.
