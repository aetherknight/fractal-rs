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


use super::ChaosGame;
use super::super::geometry::*;
use rand;
use rand::distributions::{IndependentSample, Range};
use std::sync::mpsc::SyncSender;

#[derive(Clone, Default)]
pub struct SierpinskiChaosGame;

impl SierpinskiChaosGame {
    pub fn new() -> SierpinskiChaosGame {
        SierpinskiChaosGame
    }
}

impl ChaosGame for SierpinskiChaosGame {
    fn generate(&self, channel: &mut SyncSender<Point>) {
        let mut rng = rand::thread_rng();
        let space_range = Range::new(-1.0f64, 1.0f64);

        // Generate the outer triangle points.
        let vertices = [Point {
                            x: space_range.ind_sample(&mut rng),
                            y: space_range.ind_sample(&mut rng),
                        },
                        Point {
                            x: space_range.ind_sample(&mut rng),
                            y: space_range.ind_sample(&mut rng),
                        },
                        Point {
                            x: space_range.ind_sample(&mut rng),
                            y: space_range.ind_sample(&mut rng),
                        }];

        let point_range = Range::new(0, 3);

        // Construct the center point.
        let sum_point = vertices.iter().fold(Point { x: 0.0, y: 0.0 }, |acc, &p| {
            Point {
                x: acc.x + p.x,
                y: acc.y + p.y,
            }
        });
        let curr_point = Point {
            x: sum_point.x / (vertices.len() as f64),
            y: sum_point.y / (vertices.len() as f64),
        };

        // pick the first vertex to jump halfway towards
        let mut target = point_range.ind_sample(&mut rng);
        let mut target_point = vertices[target];

        // first move towards that vertex
        let mut curr_point = Point {
            x: (curr_point.x + target_point.x) / 2.0,
            y: (curr_point.y + target_point.y) / 2.0,
        };

        // Send the move, repeat ad naseum
        while let Ok(_) = channel.send(curr_point) {
            target = point_range.ind_sample(&mut rng);
            target_point = vertices[target];
            curr_point = Point {
                x: (curr_point.x + target_point.x) / 2.0,
                y: (curr_point.y + target_point.y) / 2.0,
            };
        }
    }
}
