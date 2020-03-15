#!/bin/sh

# Script to manually test each kind of renderer

# Chaos Game
cargo run -- sierpinski --drawrate 10

# Turtle
cargo run -- terdragon 5 --drawrate 10

# Escape Time
cargo run -- mandelbrot 100 2
