# fractal-rs

A Rust library and program for exploring a variety of fractals:

At the moment, it can only display [Dragon
curves](https://en.wikipedia.org/wiki/Dragon_curve), and it seems to run into
performance issues with larger iterations (I am still learning piston and its
GL bindings, although a different backend may be in order). However, it is
still is very early development.

In the future, I hope to add support for:

* More/arbitrary curves constructed using the [Lindenmeyer
  system](https://en.wikipedia.org/wiki/L-system)
* [Iterated Function
  Systems](https://en.wikipedia.org/wiki/Iterated_function_system)
* Information about what is being viewed
* Greater interactivity, maybe a UI for choosing and configuring which fractal
  to display
* Color
* Ability to export images


## Usage

Fetch the git repository, and then use cargo to build and run it:

```sh
git clone https://github.com/aetherknight/fractal-rs.git
cd fractal-rs
cargo build
cargo run ${DRAGON_ITERATION}
```

Where `${DRAGON_ITERATION}` is an integer 0 or greater. Note that iterations
above 15 may start taking much longer to generate on modern computers.


## Contributing

Contributions should work on the current stable release of Rust.


## License

Copyright (c) 2015 William (B.J.) Snow Orvis. Licensed under the Apache
License, Version 2.0. See [LICENSE](LICENSE) for details.
