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

//! Chaos games are a method of drawing fractals by randomly drawing a set of points. The
//! [Wikipedia page](https://en.wikipedia.org/wiki/Chaos_game) provides a detailed and technical
//! explanation for this process.

pub mod barnsleyfern;
pub mod sierpinski;

use std::sync::Arc;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use std::thread;

use super::geometry::Point;

/// A ChaosGame implementation with a method that will be run in a separate thread to create a
/// generator function that yields Points across the channel. The ChaosGame should track whatever
/// parameters it wants or needs to to customize itself, and `generate()` will likely use an RNG to
/// introduce the random element needed.
pub trait ChaosGame {
    /// Generator function that should send Points across a buffered channel, possibly forever. The
    /// function should only return if it is done or if the channel indicates that the remote side
    /// has closed/errored.
    fn generate(&self, channel: &mut SyncSender<Point>);
}

/// Iterator for ChaosGames that uses a thread and ChaosGame::generate() to yield Points.
/// Implemented as its own iterator and not just using the channel Receiver's iterator in order to
/// join the thread when the iterator goes out of scope.
pub struct ChaosGameMoveIterator {
    rx: Option<Receiver<Point>>,
    worker: Option<thread::JoinHandle<()>>,
}

impl ChaosGameMoveIterator {
    /// Construct a new ChaosGameMoveIterator using an instance of a ChaosGame. It will launch a
    /// thread that calls ChaosGame::generate() and sends the Points it generates to the iterator
    /// using a channel. The ChaosGame must be managed by an Arc in order to share the ChaosGame
    /// with the thread.
    pub fn new(game: Arc<ChaosGame + Send + Sync>) -> ChaosGameMoveIterator {
        let (mut tx, rx) = sync_channel::<Point>(10);
        let worker = thread::spawn(move || {
            game.generate(&mut tx);
            println!("Receiver exited");
        });

        ChaosGameMoveIterator {
            rx: Some(rx),
            worker: Some(worker),
        }
    }
}

impl Iterator for ChaosGameMoveIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        match self.rx.as_ref() {
            Some(ref rx) => {
                match rx.recv() {
                    Ok(result) => Some(result),
                    Err(e) => {
                        println!("Remote generator exited: {}", e.to_string());
                        None
                    }
                }
            }
            None => None,
        }
    }
}

impl Drop for ChaosGameMoveIterator {
    fn drop(&mut self) {
        println!("Waiting for worker to join");
        drop(self.rx.take()); // drop the receiver to convince the worker thread to exit
        match self.worker.take().unwrap().join() {
            Ok(_) => println!("Worker exited normally"),
            Err(e) => {
                println!("Worker exited abnormally.");
                panic!(e);
            }
        }
    }
}
