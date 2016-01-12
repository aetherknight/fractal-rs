# fractal-rs

A Rust application (and library) for exploring a variety of fractals and to learn more about rust.

Features include:

* Piston-based turtle graphics engine for line-drawn curves (using OpenGL
  bindings)
* Support for computing turtle directions using the [Lindenmeyer
  system](https://en.wikipedia.org/wiki/L-system)
* Fractals supported:
    * [Cesàro square fractal (torn
      fractal)](http://mathworld.wolfram.com/CesaroFractal.html)
    * Cesàro triangle fractal
    * [Dragon curve](https://en.wikipedia.org/wiki/Dragon_curve)
    * [Koch snowflake](https://en.wikipedia.org/wiki/Koch_snowflake)
    * [Lévy C curve](https://en.wikipedia.org/wiki/L%C3%A9vy_C_curve)
    * Terdragon fractal

Some future ideas (in no particular order):

* Improve performance in various ways. The current implementation is single
  threaded, naively uses the graphics engine, etc.
* Animating Turtle programs while drawing instead of (re)drawing the entire
  program before displaying it (this may also help improve responsiveness for
  larger fractals that take a long time to draw).
* Display information about the current fractal.
* Greater interactivity, maybe a UI for choosing and configuring which fractal
  to display, or arrow keys to increment/decrement the iteration number.
* Color for various curves
* Ability to export images
* More [Iterated Function
  Systems](https://en.wikipedia.org/wiki/Iterated_function_system), such as
  those drawn using the chaos game, or that can be draw with shapes
* Mandelbrot sets, Julia sets, Burning ship fractal, etc.


## Usage

Fetch the git repository, and then use cargo to build and run it:

```sh
git clone https://github.com/aetherknight/fractal-rs.git
cd fractal-rs
cargo build
cargo run dragon ${ITERATION}
```

Where `${ITERATION}` is a non-negative integer (0 or greater). This will
eventually bring up a window that displays the fractal. You can exit by
pressing your `esc` key.


### Exploring Fractals

TODO: Usage for all supported fractals

Note that for most fractals the iteration number results in an exponential
increase in computation, so if you want to explore a higher
iteration/generation of a curve, you may want to start with a low iteration
number and increment your way up. (At the moment, the dragon fractal tends to
take more than a few seconds to draw iterations above 15 on modern computers).

At present, if you want to play with other parameters --- such as the
coordinate space where a fractal is drawn, the L-system used to determine what
steps to take, or the angles chosen --- then you will need to update the source
code and use cargo to build and run the program.

## Contributing

* If you have a feature request and would like to discuss it, please open a
  ticket.
* If you would like to implement a feature, feel free to submit a pull request
  or open a ticket to discuss followed by a pull request.

A few rules about contributed code:

* In general, contributions should work on the current stable release of Rust.
  If you want to use a nightly Rust feature, we should discuss the approach
  (eg, can it be made optional, does it provide a huge performance improvement,
  etc.).
* Use [rustfmt](https://github.com/rust-lang-nursery/rustfmt) to format your
  code before committing.
* Write tests where it makes sense to do so (ie, test behaviors and
  functionality that could change as a side-effect of some other change), but
  do not fret about it. It can be difficult to write effective, non-brittle
  tests for graphically oriented programs.


## License

Copyright (c) 2015-2016 William (B.J.) Snow Orvis. Licensed under the Apache
License, Version 2.0. See [LICENSE](LICENSE) for details.
