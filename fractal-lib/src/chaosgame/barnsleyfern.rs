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

//! Implementation of the Barnsley Fern.

use super::super::geometry::*;
use super::{ChaosGameMoveIterator, ChaosGameThreadedGenerator};
use rand;
use rand::distributions::{Distribution, WeightedIndex};
use std::sync::mpsc::SyncSender;

/// The reference affine transforms for the Barnsley Fern.
pub const REFERENCE_TRANSFORMS: [CartesianAffineTransform; 4] = [
    [[0.0, 0.0, 0.0], [0.0, 0.16, 0.0]],
    [[0.85, 0.04, 0.0], [-0.04, 0.85, 1.6]],
    [[0.2, -0.26, 0.0], [0.23, 0.22, 1.6]],
    [[-0.15, 0.28, 0.0], [0.26, 0.24, 0.44]],
];
/// The reference affine transform weights for the Barnsley Fern.
pub const REFERENCE_WEIGHTS: [u32; 4] = [1, 85, 7, 7];

/// [Barnsley Fern](https://en.wikipedia.org/wiki/Barnsley_fern) fractal, generated using an IFS
/// and a chaos game. A fern is constructed by starting at (0,0), randomly picking one of 4 affine
/// transformations (each has a separate weight), and applying the chosen affine transformation
/// function to the point to get the next point. The process is then applied again to the new
/// point, indefinitely.
#[derive(Clone)]
pub struct BarnsleyFern {
    /// Defined by 4 affine transforms
    transforms: [CartesianAffineTransform; 4],
    /// And their probabilistic weights
    weights: [u32; 4],
    last_point: Point,
}

impl BarnsleyFern {
    pub fn new(transforms: &[CartesianAffineTransform; 4], weights: &[u32; 4]) -> BarnsleyFern {
        BarnsleyFern {
            transforms: *transforms,
            weights: *weights,
            last_point: Point { x: 0.0, y: 0.0 },
        }
    }

    /// Internal helper for ChaosGameThreadedGenerator.
    ///
    /// The lifetime is needed here to satisfy the compiler's use of the box elsewhere.
    fn pick_transform<'a>(&'a self) -> Box<Fn(Point) -> Point + 'a> {
        let dist = WeightedIndex::new(&self.weights).unwrap();
        let mut rng = rand::thread_rng();
        let chosen_index = dist.sample(&mut rng);
        // box up and return a closure to do the call
        Box::new(move |p| self.transforms[chosen_index].transform(p))
    }
}

impl ChaosGameThreadedGenerator for BarnsleyFern {
    fn generate(&self, channel: &mut SyncSender<Point>) {
        let mut curr_point = Point { x: 0.0, y: 0.0 };
        while let Ok(_) = channel.send(Point {
            x: curr_point.x / 10.0,
            y: curr_point.y / 10.0,
        }) {
            curr_point = self.pick_transform()(curr_point);
        }
    }
}

impl Iterator for BarnsleyFern {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        let dist = WeightedIndex::new(&self.weights).unwrap();
        let mut rng = rand::thread_rng();
        let chosen_index = dist.sample(&mut rng);

        self.last_point = self.transforms[chosen_index].transform(self.last_point);
        Some(Point {
            x: self.last_point.x / 10.0,
            y: self.last_point.y / 10.0,
        })
    }
}

impl ChaosGameMoveIterator for BarnsleyFern {
    fn reset_game(&mut self) {
        self.last_point = Point { x: 0.0, y: 0.0 };
    }
}
