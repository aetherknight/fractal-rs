// Copyright (c) 2016 William (B.J.) Snow Orvis
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

//! Implementation of a 2-D Sierpinski triangle as a `ChaosGame`.

use super::super::geometry::*;
use super::{ChaosGameMoveIterator, ChaosGameThreadedGenerator};
use rand;
use rand::distributions::{Distribution, Uniform};
use std::sync::mpsc::SyncSender;

#[derive(Clone)]
pub struct SierpinskiChaosGame {
    vertices: [Point; 3],
    curr_point: Point,
}

impl SierpinskiChaosGame {
    pub fn new() -> SierpinskiChaosGame {
        let mut rng = rand::thread_rng();

        // Generate the outer triangle points, and start at the center point.
        let vertices = Self::gen_vertices();
        let center_point = Self::center_point(&vertices);
        let mut game = SierpinskiChaosGame {
            vertices, curr_point: center_point,
        };

        // Pick the first vertex to jump halfway towards
        let point_range = Uniform::from(0..3);
        let target = point_range.sample(&mut rng);
        let target_point = game.vertices[target];

        // first move towards that vertex
        game.curr_point = Point {
            x: (game.curr_point.x + target_point.x) / 2.0,
            y: (game.curr_point.y + target_point.y) / 2.0,
        };
        game
    }

    fn gen_vertices() -> [Point; 3] {
        let mut rng = rand::thread_rng();
        let space_range = Uniform::from(-1.0f64..1.0f64);

        [
            Point {
                x: space_range.sample(&mut rng),
                y: space_range.sample(&mut rng),
            },
            Point {
                x: space_range.sample(&mut rng),
                y: space_range.sample(&mut rng),
            },
            Point {
                x: space_range.sample(&mut rng),
                y: space_range.sample(&mut rng),
            },
        ]
    }

    fn center_point(vertices: &[Point]) -> Point {
        let sum_point = vertices
            .iter()
            .fold(Point { x: 0.0, y: 0.0 }, |acc, &p| Point {
                x: acc.x + p.x,
                y: acc.y + p.y,
            });

        Point {
            x: sum_point.x / (vertices.len() as f64),
            y: sum_point.y / (vertices.len() as f64),
        }
    }
}

impl ChaosGameThreadedGenerator for SierpinskiChaosGame {
    fn generate(&self, channel: &mut SyncSender<Point>) {
        let mut rng = rand::thread_rng();
        let point_range = Uniform::from(0..3);
        let mut curr_point = self.curr_point;

        // Send the move, repeat ad naseum
        while let Ok(_) = channel.send(curr_point) {
            let target = point_range.sample(&mut rng);
            let target_point = self.vertices[target];
            curr_point = Point {
                x: (self.curr_point.x + target_point.x) / 2.0,
                y: (self.curr_point.y + target_point.y) / 2.0,
            };
        }
    }
}

impl Iterator for SierpinskiChaosGame {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let mut rng = rand::thread_rng();
        let point_range = Uniform::from(0..3);
        let target = point_range.sample(&mut rng);
        let target_point = self.vertices[target];
        self.curr_point = Point {
            x: (self.curr_point.x + target_point.x) / 2.0,
            y: (self.curr_point.y + target_point.y) / 2.0,
        };
        Some(self.curr_point)
    }
}

impl ChaosGameMoveIterator for SierpinskiChaosGame {
    fn reset_game(&mut self) {
        self.vertices = Self::gen_vertices();
        self.curr_point = Self::center_point(&self.vertices);
    }
}
