# fractal-rs

A Rust application (and library) for exploring a variety of fractals and to
learn more about Rust.

Features include:

* Web/WASM-based target for rendering fractals on the web
* Piston-based renderer for rendering fractals on the desktop
* Ability to animate the drawing of the curves
* Library support for computing turtle directions using the [Lindenmeyer
  system](https://en.wikipedia.org/wiki/L-system)
* Curves supported:
    * [Cesàro square fractal (torn
      fractal)](http://mathworld.wolfram.com/CesaroFractal.html)
    * Cesàro triangle fractal (with angles calculated to prevent overlapping
      line segments)
    * [Dragon curve](https://en.wikipedia.org/wiki/Dragon_curve)
    * [Koch snowflake](https://en.wikipedia.org/wiki/Koch_snowflake)
    * [Lévy C curve](https://en.wikipedia.org/wiki/L%C3%A9vy_C_curve)
    * Terdragon fractal
* Chaos game images supported:
    * [Barnsley fern](https://en.wikipedia.org/wiki/Barnsley_fern)
    * [Sierpinski triangle](https://en.wikipedia.org/wiki/Sierpinski_triangle)
* Escape time fractals, with support for shading/color, and zooming in and out.
  Supported families of escape time fractals include:
    * [Mandelbrot set](https://en.wikipedia.org/wiki/Mandelbrot_set) with
      generalized support for some
      [multibrot sets](https://en.wikipedia.org/wiki/Multibrot_set)
    * [Burning ship fractal](https://en.wikipedia.org/wiki/Burning_Ship_fractal)
      with generalized support for some [related power
      sets](https://theory.org/fracdyn/burningship/symmetry.html)

Some future ideas (in no particular order):

* Option to automatically profile and adjust how much can be animated per-frame
  based on system performance for curves and chaos games.
* Display information about the current fractal.
* Greater interactivity, maybe a UI for choosing and configuring which fractal
  to display, or arrow keys to increment/decrement the iteration number.
* Customizable color for some curves
* Ability to export images or animations
* Dynamically specify more parameters through configuration instead of
  compiling them in, or support some sort of configuration format for
  specifying parameters.
* Other kinds of fractals like Julia/Fatou sets, etc.
* Explore using threading and channels to construct a generic iterator of turtle
  program steps (simulating coroutines). This might allow for more programming
  styles within a TurtleProgram instead of having to create custom iterators
  for each TurtleProgram implementation that have to track state/program
  counter for the program. It would also be a great exercise in multi-threading.
* Explore using a multi-threaded approach to render escape time fractals, which
  are highly parallelizable, and could also show the results in real time.


## Usage

Fetch the git repository, and then use cargo to build it:

```sh
git clone https://github.com/aetherknight/fractal-rs.git
cd fractal-rs
cargo build
```

To run the application, you can use `cargo run` or call the binary directly
(Eg, if you use `cargo install` to install the program). Resizing the window
will dynamically resize the curve as well. You can exit by pressing the `esc`
key.

For command usage information, try:
```sh
cargo run -- help
```

To open a window and draw iteration 4 of the Cesàro square fractal:

```sh
cargo run -- cesaro 4
```

To animate the drawing of a curve, you can use the `--drawrate` option. You must
specify the number of line segments that should be drawn per frame as well,
such as `1` for one line per frame. The following will animate iteration 11 of
the dragon fractal at a rate of 10 line segments per frame:

```sh
cargo run -- dragon 11 --drawrate 10 
```


### Exploring Fractals
The fractal program has a the following subcommands:

| Subcommand | Description |
| ---------- | ----------- |
| barnsleyfern&nbsp;[--drawrate&nbsp;&lt;MPF&gt;] | Draws the Barnsley Fern fractal using a chaos game with affine transforms. |
| burningmandel&nbsp;&lt;MAX_IT&gt;&nbsp;&lt;POWER&gt; | Draws a variation of the burning ship fractal
| burningship&nbsp;&lt;MAX_IT&gt;&nbsp;&lt;POWER&gt; | Draws the burning ship fractal
| cesaro&nbsp;[--drawrate&nbsp;&lt;MPF&gt;] | Draws a square Césaro fractal
| cestarotri&nbsp;[--drawrate&nbsp;&lt;MPF&gt;] | Draws a triangle Césaro fractal
| dragon&nbsp;[--drawrate&nbsp;&lt;MPF&gt;] | Draws a dragon curve fractal
| kochcurve&nbsp;[--drawrate&nbsp;&lt;MPF&gt;] | Draws a Koch snowflake curve
| levyccurve&nbsp;[--drawrate&nbsp;&lt;MPF&gt;] | Draws a Levy C Curve
| mandelbrot&nbsp;&lt;MAX_IT&gt;&nbsp;&lt;POWER&gt; | Draws the mandelbrot fractal
| roadrunner&nbsp;&lt;MAX_IT&gt;&nbsp;&lt;POWER&gt; | Draws a variation of the burning ship fractal
| sierpinski&nbsp;[--drawrate&nbsp;&lt;MPF&gt;] | Draws a Sierpinski triangle using a chaos game and 3 randomly chosen points on the screen
| terdragon&nbsp;[--drawrate&nbsp;&lt;MPF&gt;] | Draws a terdragon curve

The subcommands arguments have the following explanations

| Argument | Description |
| -------- | ----------- |
| MPF | The number of points to draw per frame [default: 1]
| MAX_IT | The maximum number of iterations of the escape time function before deciding the fracal has escaped
| POWER | The exponent used in the escape time function (positive integer)


### Notes
Note that for most fractals the iteration number results in an exponential
increase in computation, so if you want to explore a higher
iteration/generation of a curve, you may want to start with a low iteration
number and increment your way up. (At the moment, the dragon fractal tends to
take more than a few seconds to draw iterations above 15 my laptop when it
draws the entire curve in a single frame).

At present, the other parameters that make up one of the curves --- such as the
coordinate space, the L-system used to determine the drawing steps, or the
angles chosen --- require changing the source code.

## Contributing

* If you have a feature request and would like to discuss it, please open a
  ticket.
* If you would like to implement a feature, feel free to submit a pull request
  or open a ticket to discuss followed by a pull request.
* If I receive many contributions, I will shift the project's copyright
  structure to reference a contributers file.

A few rules about contributed code:

* In general, contributions should work on the current stable release of Rust.
  If you want to use a nightly Rust feature, we should discuss the approach
  (eg, can it be made optional, does it provide a huge performance improvement,
  etc.).
* Use [rustfmt](https://github.com/rust-lang-nursery/rustfmt) to format your
  code before committing.
* Write tests where it makes sense to do so (ie, test behaviors and
  functionality that could change as a side-effect of some other change), but
  do not fret about it.
* Try to keep the `use` statements lexicographically sorted, with std and crate
  modules grouped together, and local modules grouped together after them.


## License

Copyright (c) 2015-2019 William (B.J.) Snow Orvis. Licensed under the Apache
License, Version 2.0. See [LICENSE](LICENSE) for details.
