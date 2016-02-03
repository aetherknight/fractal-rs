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

//! Implementation of the Barnsley Fern.

use std::sync::mpsc::SyncSender;
use rand;
use rand::distributions::{IndependentSample, WeightedChoice, Weighted};

use super::ChaosGame;
use super::super::geometry::*;

#[derive(Clone)]
pub struct BarnsleyFern;

impl BarnsleyFern {
    pub fn new() -> BarnsleyFern {
        BarnsleyFern
    }

    /// 1%
    fn f1(n: Point) -> Point {
        Point {
            x: 0.0,
            y: 0.16 * n.y,
        }
    }
    /// 85%
    fn f2(n: Point) -> Point {
        Point {
            x: 0.85 * n.x + 0.04 * n.y,
            y: -0.04 * n.x + 0.85 * n.y + 1.6,
        }
    }
    /// 7%
    fn f3(n: Point) -> Point {
        Point {
            x: 0.2 * n.x + -0.26 * n.y,
            y: 0.23 * n.x + 0.22 * n.y + 1.6,
        }
    }
    /// 7%
    fn f4(n: Point) -> Point {
        Point {
            x: -0.15 * n.x + 0.28 * n.y,
            y: 0.26 * n.x + 0.24 * n.y + 0.44,
        }
    }

    // fn pick_and_apply_function(n: Point) -> Point {
    // }
    fn pick_function() -> Box<Fn(Point) -> Point> {
        // TODO: macro the weighted_indices to avoid doing it at runtime.
        let weights = [1, 85, 7, 7];
        let mut weighted_indices: Vec<Weighted<usize>> = (0..4)
                                                             .into_iter()
                                                             .map(|i| {
                                                                 Weighted {
                                                                     weight: weights[i],
                                                                     item: i as usize,
                                                                 }
                                                             })
                                                             .collect();
        let chooser = rand::distributions::WeightedChoice::new(&mut weighted_indices);
        let mut rng = rand::thread_rng();
        match chooser.ind_sample(&mut rng) {
            0 => Box::new(BarnsleyFern::f1),
            1 => Box::new(BarnsleyFern::f2),
            2 => Box::new(BarnsleyFern::f3),
            3 => Box::new(BarnsleyFern::f4),
            _ => panic!("not reachable?"),
        }
    }

    fn apply_function(n: Point) -> Point {
        Self::pick_function()(n)
    }
}

impl ChaosGame for BarnsleyFern {
    fn generate(&self, channel: &mut SyncSender<Point>) {
        let mut curr_point = Point { x: 0.0, y: 0.0 };
        while let Ok(_) = channel.send(Point {
            x: curr_point.x / 10.0,
            y: curr_point.y / 10.0,
        }) {
            curr_point = Self::apply_function(curr_point);
        }
    }
}
