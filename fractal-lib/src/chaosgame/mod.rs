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

//! Chaos games are a method of drawing fractals by randomly drawing a set of points. The
//! [Wikipedia page](https://en.wikipedia.org/wiki/Chaos_game) provides a detailed and technical
//! explanation for this process.

pub mod barnsleyfern;
pub mod sierpinski;

use super::geometry::Point;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::sync::Arc;
use std::thread;

/// A chaos game iterator needs to be an iterator, but it also needs to be able to reset the game
/// back to an initial state (Eg, when the screen is resized).
pub trait ChaosGameMoveIterator: Iterator<Item = Point> {
    fn reset_game(&mut self);
}

/// A chaos game specification that uses threads, and can be used with a
/// `ChaosGameMoveThreadedIterator` to yield `Point`s from a thread, across a channel.
///
/// A `ChaosGameThreadedGenerator` implmentation should track whatever parameters it wants or needs
/// to to customize itself, and `generate()` will likely use an RNG to introduce the random element
/// needed.
pub trait ChaosGameThreadedGenerator {
    /// Generator function that should send Points across a buffered channel, possibly forever.
    /// The function should only return if it is done or if the channel indicates that the
    /// remote side has closed/errored.
    fn generate(&self, channel: &mut SyncSender<Point>);
}

/// Iterator for `ChaosGame`s that uses a thread and `ChaosGame::generate()` to yield Points.
/// Implemented as its own iterator and not just using the channel Receiver's iterator in order to
/// join the thread when the iterator goes out of scope.
pub struct ChaosGameMoveThreadedIterator {
    rx: Option<Receiver<Point>>,
    worker: Option<thread::JoinHandle<()>>,
}

impl ChaosGameMoveThreadedIterator {
    /// Construct a new ChaosGameMoveThreadedIterator using an instance of a ChaosGame. It will
    /// launch a thread that calls ChaosGame::generate() and sends the Points it generates to the
    /// iterator using a channel. The ChaosGame must be managed by an Arc in order to share the
    /// ChaosGame with the thread.
    pub fn new(
        game: Arc<dyn ChaosGameThreadedGenerator + Send + Sync>,
    ) -> ChaosGameMoveThreadedIterator {
        let (mut tx, rx) = sync_channel::<Point>(10);
        let worker = thread::spawn(move || {
            game.generate(&mut tx);
            log::debug!("Receiver exited");
        });

        ChaosGameMoveThreadedIterator {
            rx: Some(rx),
            worker: Some(worker),
        }
    }
}

impl Iterator for ChaosGameMoveThreadedIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        match self.rx {
            Some(ref rx) => match rx.recv() {
                Ok(result) => Some(result),
                Err(e) => {
                    log::debug!("Remote generator exited: {}", e.to_string());
                    None
                }
            },
            None => None,
        }
    }
}

impl Drop for ChaosGameMoveThreadedIterator {
    fn drop(&mut self) {
        log::debug!("Waiting for worker to join");
        drop(self.rx.take()); // drop the receiver to convince the worker thread to exit
        match self.worker.take().unwrap().join() {
            Ok(_) => log::debug!("Worker exited normally"),
            Err(e) => {
                log::debug!("Worker exited abnormally.");
                std::panic::panic_any(e);
            }
        }
    }
}
