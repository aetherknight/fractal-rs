// Copyright (c) 2015-2016 William (B.J.) Snow Orvis
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

//! Lindenmayer systems are a methodology for representing various kinds of
//! iterated curves or other constructions. A given system has an alphabet of
//! symbols, an initial string, and one or more rules for transforming that
//! string into the next iteration/generation.
//!
//! The resulting string at a given iteration or generation can represent a
//! sequence of commands that some representation process, such as a turtle
//! drawing program, can then use to draw a curve/fractal/plant (which is what
//! this implementation provides).

use geometry::Point;
use std::cell::RefCell;
use std::marker::PhantomData;
use turtle::*;

/// Represents a particular Lindenmayer system. It requires an alphabet
/// (represented as an enum), an initial sequence ("string"), and one or more
/// rules that transform the sequence with each iteration/generation.
///
/// Under many circumstances, these methods will not actually use &self.
/// However, having a &self allows for more flexibility, such as creating
/// caching objects or creating L-systems that can be defined at runtime
/// instead of at compile time.
pub trait LindenmayerSystem<Alphabet: Clone> {
    /// Should return the initial Lindenmayer system string (iteration 0).
    fn initial(&self) -> Vec<Alphabet>;

    /// Apply Lindenmayer system rules to a given character.
    ///
    /// A common implementation approach would be to use `match` with the
    /// Alphabet. For example:
    ///
    /// ```
    /// use fractal::lindenmayer::LindenmayerSystem;
    ///
    /// /// Our Alphbet
    /// #[derive(Clone)]
    /// enum SomeAlphabet { A, B, C, Foo }
    ///
    /// struct SomeLSystem;
    ///
    /// impl LindenmayerSystem<SomeAlphabet> for SomeLSystem {
    ///      fn initial(&self) -> Vec<SomeAlphabet> {
    ///          vec![SomeAlphabet::A, SomeAlphabet::B, SomeAlphabet::C]
    ///      }
    ///
    ///     fn apply_rule(&self, l: SomeAlphabet) -> Vec<SomeAlphabet> {
    ///         match l {
    /// SomeAlphabet::A => vec![SomeAlphabet::A, SomeAlphabet::Foo,
    /// SomeAlphabet::B, SomeAlphabet::C],
    ///             SomeAlphabet::B => vec![SomeAlphabet::Foo],
    ///             SomeAlphabet::C => vec![SomeAlphabet::Foo, SomeAlphabet::B],
    ///             SomeAlphabet::Foo => vec![SomeAlphabet::Foo],
    ///         }
    ///     }
    /// }
    /// ```
    fn apply_rule(&self, curr_symbol: Alphabet) -> Vec<Alphabet>;

    fn generate_next_iteration(&self, last_iteration: &Vec<Alphabet>) -> Vec<Alphabet> {
        let mut newlstr: Vec<Alphabet> = vec![];

        for l in last_iteration.iter().cloned() {
            for other in self.apply_rule(l).iter().cloned() {
                newlstr.push(other);
            }
        }
        newlstr
    }

    /// Generates a Lindenmayer system string for `iteration`.
    ///
    /// This is done by starting with the initial sequence, and repeatedly
    /// applying the rules to the sequence `iteration` times. The result is a
    /// new vector that contains the sequence for the specified iteration.
    fn generate(&self, iteration: u64) -> Vec<Alphabet> {
        let mut last: Vec<Alphabet> = self.initial();
        let mut i = 0;
        while i < iteration {
            i = i + 1;
            last = self.generate_next_iteration(&last);
        }
        last
    }
}

/// In order to draw a fractal using a Lindenmayer System, we need to translate
/// the output from the L-System into turtle commands. To do this, we need to
/// initial the turtle, and we need a way to convert the L-System's symbols
/// into actions. This trait provides the methods needed to configure the
/// LindenmayerSystemTurtleProgram, which is the glue that issues arbitrary
/// Turtle commands.
pub trait LindenmayerSystemDrawingParameters<Alphabet> {
    /// Returns the iteration that should be drawn.
    fn iteration(&self) -> u64;

    /// Specifies the turtle's initial position. Defaults to (0.0, 0.0).
    fn initial_pos(&self) -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    /// Specifies the turtle's initial direction. Defaults to 0.0.
    fn initial_rad(&self) -> f64 {
        0.0
    }

    /// Convert symbol into a turtle command.
    ///
    /// Usually, when moving the turtle forwards, there is some formula that
    /// will ensure that the turtle always ends at a given point, such as at
    /// (1.0, 0.0).
    fn interpret_symbol(&self, symbol: Alphabet) -> TurtleStep;
}

/// In order to improve the performance of using a Lindenmayer System under
/// some circumstances, it is beneficial to cache each iteration. This allows
/// O(1) lookups for every iteration less than or equal to the largest
/// iteration already looked up at the cost of storing every iteration below
/// the largest iteration computed (the amount of memory used changes depending
/// on characteristics of the L-System, such as how rapidly the strings grow
/// between each iteration).
pub struct LindenmayerSystemCachingDecorator<L, A>
    where L: LindenmayerSystem<A>,
          A: Clone + 'static
{
    alphabet: PhantomData<A>,
    pub system: L,
    iteration_cache: RefCell<Vec<Vec<A>>>,
}

impl<'a, L, A> LindenmayerSystemCachingDecorator<L, A>
    where L: LindenmayerSystem<A>,
          A: Clone + 'static
{
    /// Return a new LindenmayerSystemCachingDecorator that wraps the input
    /// `system` and caches its iterations.
    pub fn new(system: L) -> LindenmayerSystemCachingDecorator<L, A> {
        LindenmayerSystemCachingDecorator {
            alphabet: PhantomData,
            system: system,
            iteration_cache: RefCell::new(vec![]),
        }
    }
}

impl<L, A> LindenmayerSystem<A> for LindenmayerSystemCachingDecorator<L, A>
    where L: LindenmayerSystem<A>,
          A: Clone + 'static
{
    /// Delegate to system
    fn initial(&self) -> Vec<A> {
        self.system.initial()
    }

    /// Delegate to system
    fn apply_rule(&self, curr_symbol: A) -> Vec<A> {
        self.system.apply_rule(curr_symbol)
    }

    /// Reimplement to to use caching. Currently using recursion.
    fn generate(&self, iteration: u64) -> Vec<A> {
        {
            let cache = self.iteration_cache.borrow_mut();
            let val = cache.get(iteration as usize);
            if val.is_some() {
                println!("found {}", iteration);
                return val.unwrap().clone();
            }
        } // end borrow of the cache
        {
            println!("generating {}", iteration);
            let curr = match iteration {
                0 => self.system.initial(), // terminating case
                _ => {
                    let last = self.generate(iteration - 1);
                    self.system.generate_next_iteration(&last)
                }
            };
            let mut cache = self.iteration_cache.borrow_mut();
            cache.push(curr);
        } // end borrow of the cache
        // Now that we have cached, try again (which should use the cache)
        self.generate(iteration)
    }
}

pub struct LindenmayerSystemTurtleProgram<L, A>
    where L: LindenmayerSystem<A> + LindenmayerSystemDrawingParameters<A>,
          A: Clone + 'static
{
    alphabet: PhantomData<A>,
    cacheable_system: LindenmayerSystemCachingDecorator<L, A>, // system: L,
}

impl<L, A> LindenmayerSystemTurtleProgram<L, A>
    where L: LindenmayerSystem<A> + LindenmayerSystemDrawingParameters<A>,
          A: Clone
{
    pub fn new(system: L) -> LindenmayerSystemTurtleProgram<L, A> {
        LindenmayerSystemTurtleProgram {
            alphabet: PhantomData,
            cacheable_system: LindenmayerSystemCachingDecorator::new(system), // system: system,
        }
    }
}

impl<L, A> TurtleProgram for LindenmayerSystemTurtleProgram<L, A>
    where L: LindenmayerSystem<A> + LindenmayerSystemDrawingParameters<A> + 'static,
          A: Clone + 'static
{
    fn init_turtle(&self) -> Vec<TurtleStep> {
        vec![
            TurtleStep::SetPos(self.cacheable_system.system.initial_pos()),
            TurtleStep::SetRad(self.cacheable_system.system.initial_rad()),
            TurtleStep::Down,
        ]
    }

    fn turtle_program_iter<'a>(&'a self) -> TurtleProgramIterator<'a> {
        let sequence = self.cacheable_system.generate(self.cacheable_system.system.iteration());
        println!("Done");

        TurtleProgramIterator::new(Box::new(LindenmayerSystemTurtleProgramIterator {
            alphabet: PhantomData,
            program: self,
            sequence: sequence,
            curr_step: 0,
        }))
    }
}

pub struct LindenmayerSystemTurtleProgramIterator<'a, L, A>
    where L: LindenmayerSystem<A> + LindenmayerSystemDrawingParameters<A> + 'static,
          A: Clone + 'static
{
    alphabet: PhantomData<A>,
    program: &'a LindenmayerSystemTurtleProgram<L, A>,
    sequence: Vec<A>,
    curr_step: usize,
}

impl<'a, L, A> Iterator for LindenmayerSystemTurtleProgramIterator<'a, L, A>
    where L: LindenmayerSystem<A> + LindenmayerSystemDrawingParameters<A> + 'static,
          A: Clone + 'static
{
    type Item = TurtleStep;

    fn next(&mut self) -> Option<TurtleStep> {
        if self.curr_step >= self.sequence.len() {
            return None;
        }

        let symbol = self.sequence[self.curr_step].clone();
        self.curr_step += 1;

        Some(self.program.cacheable_system.system.interpret_symbol(symbol))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Clone, PartialEq, Eq, Debug)]
    enum TestABC {
        A,
        B,
        C,
        Foo,
    }

    struct TestLS;

    impl LindenmayerSystem<TestABC> for TestLS {
        fn initial(&self) -> Vec<TestABC> {
            vec![TestABC::A, TestABC::B, TestABC::C]
        }

        fn apply_rule(&self, l: TestABC) -> Vec<TestABC> {
            match l {
                TestABC::A => vec![TestABC::A, TestABC::Foo, TestABC::B, TestABC::C],
                TestABC::B => vec![TestABC::Foo],
                TestABC::C => vec![TestABC::Foo, TestABC::B],
                TestABC::Foo => vec![TestABC::Foo],
            }
        }
    }

    /// Test a sample TestLS system
    #[test]
    fn test_lsystem() {
        assert_eq!(TestLS.generate(0), [TestABC::A, TestABC::B, TestABC::C]);
        assert_eq!(TestLS.generate(1),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B]);
        assert_eq!(TestLS.generate(2),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo]);
        assert_eq!(TestLS.generate(3),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo]);
    }

    /// Test the caching decorator
    #[test]
    fn test_caching() {
        // All this can really test is verify that multiple calls for the same
        // generation come out the same.
        let orig_system = TestLS;
        let system = LindenmayerSystemCachingDecorator::new(orig_system);
        // baseline
        assert_eq!(system.generate(0), [TestABC::A, TestABC::B, TestABC::C]);
        // again
        assert_eq!(system.generate(0), [TestABC::A, TestABC::B, TestABC::C]);
        // something that will require caching 1 and 2 and use 0's cache
        assert_eq!(system.generate(3),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo]);
        // use cache
        assert_eq!(system.generate(3),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::Foo]);
        // use cache
        assert_eq!(system.generate(1),
                   [TestABC::A,
                    TestABC::Foo,
                    TestABC::B,
                    TestABC::C,
                    TestABC::Foo,
                    TestABC::Foo,
                    TestABC::B]);
    }
}
