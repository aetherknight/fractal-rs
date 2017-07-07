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

//! A relatively uncomplicated multiplexer abstraction that allows you to run a parallelizable
//! problem across multiple threads, and signal them to stop if they need to halt early. Eg:
//!
//! ```
//! use fractal::work_multiplexer::*;
//!
//! let handles = ThreadedWorkMultiplexerBuilder::new(8)
//!     .base_name("hello worlder")
//!     .split_work(|thread_id, total_threads, notifier, name| {
//!         // break up a larger problem into smaller ones by
//!         // sharding the original problem space
//!         let sharded = (0..100)
//!             .into_iter()
//!             .enumerate()
//!             .filter(|&(index, _)| {
//!                 (index as u32 + thread_id) %
//!                 total_threads == 0
//!             })
//!             .map(|(_, val)| val);
//!         for i in sharded {
//!             if notifier.should_i_stop() {
//!                 break;
//!             }
//!             println!("{}: do some work for index {}", name, i);
//!         }
//!     });
//! handles.wait();
//! println!("Done!")
//! ```

use std::sync::Arc;
use std::sync::mpsc::*;
use std::thread;

use num_cpus;
use time;

/// Measures how long a block takes to complete, and returns that time.
fn measure_time<T, F>(block: F) -> (time::Duration, T)
where
    F: Fn() -> T,
{
    let start_time = time::now_utc();
    let res = block();
    let finish_time = time::now_utc();

    (finish_time - start_time, res)
}

/// Object that can be used by a thread to determine if it should stop processing early. the
/// `ThreadedWorkMultiplexerHandles` object that is associated with the thread can signal the
/// worker threads to stop, but the worker threads have to check for themselves.
pub struct ThreadNotifier {
    receiver: Receiver<()>,
}

impl ThreadNotifier {
    pub fn new(receiver: Receiver<()>) -> ThreadNotifier {
        ThreadNotifier { receiver: receiver }
    }

    /// If true, then the thread should break out of its processing loop.
    pub fn should_i_stop(&self) -> bool {
        Err(TryRecvError::Disconnected) == self.receiver.try_recv()
    }
}

/// Builds up the configuration for a set of worker threads.
pub struct ThreadedWorkMultiplexerBuilder {
    pub thread_count: usize,
    thread_base_name: String,
}

impl ThreadedWorkMultiplexerBuilder {
    pub fn new() -> ThreadedWorkMultiplexerBuilder {
        ThreadedWorkMultiplexerBuilder {
            thread_count: num_cpus::get(),
            thread_base_name: "worker thread".to_string(),
        }
    }

    pub fn base_name(mut self, name: &str) -> ThreadedWorkMultiplexerBuilder {
        self.thread_base_name = name.to_string();
        self
    }

    pub fn split_work<F>(self, job: F) -> ThreadedWorkMultiplexerHandles
    where
        F: Fn(usize, usize, &ThreadNotifier, &str) + Send + Sync + 'static,
    {
        let mut thread_sync = Vec::with_capacity(self.thread_count as usize);
        // ARC the closure out here, so it is moved just once
        let arc_code = Arc::new(job);
        for i in 0..self.thread_count {
            let (tx, rx) = channel();
            let name = format!("{}.{}", self.thread_base_name, i);

            let total_threads = self.thread_count;
            let notifier = ThreadNotifier::new(rx);
            let thread_name = name.clone();
            let thread_code = arc_code.clone();

            let res = thread::Builder::new().name(name).spawn(move || {
                let (time_delta, _) = measure_time(|| {
                    thread_code(i, total_threads, &notifier, thread_name.as_ref());
                });
                println!("{} finished in {}", thread_name, time_delta);
            });
            if let Ok(handle) = res {
                thread_sync.push(Some((tx, handle)));
            } else {
                panic!("Failed to spawn thread {}", i);
            }
        }
        ThreadedWorkMultiplexerHandles { thread_sync: thread_sync }
    }
}

/// Tracks the running threads and allows the owner to control those threads.
///
/// If this object is dropped or goes out of scope, then it will try to stop the worker threads ---
/// this is desired behavior if the handles are replaced by new worker threads. In order to wait
/// for them to finish first, use `ThreadedWorkMultiplexerHandles::wait()`.
pub struct ThreadedWorkMultiplexerHandles {
    thread_sync: Vec<Option<(Sender<()>, thread::JoinHandle<()>)>>,
}

impl ThreadedWorkMultiplexerHandles {
    /// Blocks until all of the threads finish.
    pub fn wait(mut self) {
        for thread_info in &mut self.thread_sync {
            if let Some((_, handle)) = thread_info.take() {
                let thread_name = handle.thread().name().unwrap_or("UNKNOWN").to_string();
                match handle.join() {
                    Ok(_) => {
                        println!("Joined {}", thread_name);
                    }
                    Err(_) => {
                        println!("{} panicked while it ran", thread_name);
                    }
                }
            }
        }
    }

    /// Signals each thread to stop, then blocks until they have stopped.
    ///
    /// Threads have to check to see if they have been signaled using their notifier.
    pub fn stop(&mut self) {
        for thread_info in &mut self.thread_sync {
            if let Some((tx, handle)) = thread_info.take() {
                drop(tx);
                let thread_name = handle.thread().name().unwrap_or("UNKNOWN").to_string();
                match handle.join() {
                    Ok(_) => {
                        println!("Joined {}", thread_name);
                    }
                    Err(_) => {
                        println!("{} panicked while it ran", thread_name);
                    }
                }
            }
        }
    }

    // pub fn live_thread_count(&self) -> u32 {
    //     self.thread_sync
    //         .iter()
    //         .map(|maybe_x| {
    //             if let Some(tuple) = maybe_x.as_ref() {
    //                 if let Ok(_) = tuple.0.send(()) {
    //                     1
    //                 } else {
    //                     0
    //                 }
    //             } else {
    //                 0
    //             }
    //         })
    //         .fold(0, |acc, x| acc + x)
    // }
}

impl Drop for ThreadedWorkMultiplexerHandles {
    fn drop(&mut self) {
        self.stop();
    }
}
