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

//! Turtle program abstractions.

use crate::geometry::{deg2rad, Point};

/// A Turtle is an abstraction for drawing lines in a space. It has a position and it faces a
/// particular direction. A program usually tells a turtle to move forward based upon its facing,
/// to change direction, and to start or stop drawing.
///
/// The implementation must implement `set_rad()` and `turn_rad()` itself, while it gets
/// `set_deg()` and `turn_deg()` for free.
pub trait Turtle {
    /// How far the turtle should move forward.
    fn forward(&mut self, distance: f64);

    /// Move the turtle to the specified coordinates.
    fn set_pos(&mut self, new_pos: Point);

    /// Set the turtle's direction.
    fn set_deg(&mut self, new_deg: f64) {
        self.set_rad(deg2rad(new_deg));
    }

    fn set_rad(&mut self, new_rad: f64);

    /// Rotate the turtle, in degrees.
    ///
    /// Positive values turn the turtle "left" or counter-clockwise. Negative values turn the
    /// turtle "right" or clockwise.
    fn turn_deg(&mut self, degrees: f64) {
        self.turn_rad(deg2rad(degrees));
    }

    /// Convenience method for rotating the turtle in radians instead of degrees.
    ///
    /// 2*PI radians = 360 degrees.
    fn turn_rad(&mut self, radians: f64);

    /// Touch the turtle's pen to the drawing surface.
    fn down(&mut self);

    /// Lift the turtle's pen off of the drawing surface.
    fn up(&mut self);

    /// Perform the action represented by `step`.
    fn perform(&mut self, step: TurtleStep) {
        match step {
            TurtleStep::Forward(dist) => self.forward(dist),
            TurtleStep::SetPos(point) => self.set_pos(point),
            TurtleStep::SetRad(angle) => self.set_rad(angle),
            TurtleStep::TurnRad(angle) => self.turn_rad(angle),
            TurtleStep::Down => self.down(),
            TurtleStep::Up => self.up(),
        }
    }
}

/// Represents the possible actions that a `TurtleProgram` can perform.
#[derive(Clone, Debug, PartialEq)]
pub enum TurtleStep {
    /// Make the turtle move forward some distance in the coordinate system.
    Forward(f64),
    /// Move the turtle to the specified Point in the coordinate system.
    SetPos(Point),
    /// Set the turtle's angle. 0 and 2π are facing towards the positive X direction.
    SetRad(f64),
    /// Rotate the turtle the specified amount in radians.
    TurnRad(f64),
    /// Touch the turtle's pen to the drawing surface.
    Down,
    /// Lift the turtle's pen off of the drawing surface.
    Up,
}

/// Internal state of a turtle. Can be used by turtle implementations to store/pause their drawing.
#[derive(Clone, Debug)]
pub struct TurtleState {
    pub position: Point,
    pub angle: f64,
    pub down: bool,
}

impl TurtleState {
    /// Initializes a new TurtleState. A new turtle starts at (0,0) and faces towards the
    /// positive X axis.
    pub fn new() -> TurtleState {
        TurtleState {
            position: Point { x: 0.0, y: 0.0 },
            angle: 0.0,
            down: true,
        }
    }
}

impl Default for TurtleState {
    fn default() -> Self {
        Self::new()
    }
}

/// An object that knows how to draw something using a Turtle. Turtle programs are broken up into
/// two parts: an initializer method that should place the Turtle into its initial state, and a
/// method that returns a `TurtleProgramIterator` (which should wrap a Boxed internal iterator
/// implementation) that yields the sequence of steps for the main turtle program.
///
/// This approach adds some extra complexity and scaffolding by requiring an iterator (Rust doesn't
/// provide something equivalent to a generator function or coroutine yet), but it grants the
/// renderer renderer a huge amount of flexibility about how to draw/animate the turtle program.
pub trait TurtleProgram {
    /// Returns a sequence of steps that initialize the turtle's starting position and angle.
    fn init_turtle(&self) -> Vec<TurtleStep>;

    /// Should return an iterator object that yields TurtleSteps representing each command the
    /// turtle will take.
    fn turtle_program_iter(&self) -> TurtleProgramIterator;
}

pub struct NullTurtleProgramIterator;

impl Iterator for NullTurtleProgramIterator {
    type Item = TurtleStep;

    fn next(&mut self) -> Option<TurtleStep> {
        None
    }
}

/// The return type for `TurtleProgram::turtle_program_iter()`. Since Rust does not yet support
/// abstract return types (eg, a trait return type), the next best thing is a wrapper around a
/// boxed type.
///
/// The Iterator it is initialized with must not have a lifetime, so that the TurtleProgramIterator
/// can potentially outlive the lifetime the TurtleProgram it is created from. This means the
/// TurtleProgram probably needs to clone/copy itself and move that copy into the iterator.
pub struct TurtleProgramIterator {
    iter: Box<dyn Iterator<Item = TurtleStep>>,
}

impl<'a> TurtleProgramIterator {
    pub fn new(iter: Box<dyn Iterator<Item = TurtleStep>>) -> TurtleProgramIterator {
        TurtleProgramIterator { iter }
    }

    /// Turns the TurtleProgramIterator into an iterator that will return Vec<TurtleStep>s that
    /// each contain all steps up to the next TurtleStep::Forward. This allows a renderer to
    /// render a TurtleProgram in chunks that are broken up by moves that actually draw
    /// something.
    pub fn collect_to_next_forward(self) -> TurtleCollectToNextForwardIterator {
        TurtleCollectToNextForwardIterator { iter: self }
    }
}

impl<'a> Iterator for TurtleProgramIterator {
    type Item = TurtleStep;

    fn next(&mut self) -> Option<TurtleStep> {
        self.iter.next()
    }
}

/// Iterator that yields vectors of `TurtleSteps` until the next `TurtleStep::Forward` or until the
/// underlying iterator starts yielding None. This allows us to do perform a finite number of
/// drawing actions at a time.
pub struct TurtleCollectToNextForwardIterator {
    iter: TurtleProgramIterator,
}

impl<'a> TurtleCollectToNextForwardIterator {
    pub fn new_null_iter() -> TurtleCollectToNextForwardIterator {
        TurtleProgramIterator::new(Box::new(NullTurtleProgramIterator)).collect_to_next_forward()
    }
}

impl<'a> Iterator for TurtleCollectToNextForwardIterator {
    type Item = Vec<TurtleStep>;
    fn next(&mut self) -> Option<Vec<TurtleStep>> {
        let mut retval: Vec<TurtleStep> = Vec::new();
        loop {
            match self.iter.next() {
                None => {
                    if !retval.is_empty() {
                        return Some(retval);
                    } else {
                        return None;
                    }
                }
                Some(TurtleStep::Forward(f)) => {
                    // add the forward and return
                    retval.push(TurtleStep::Forward(f));
                    return Some(retval);
                }
                Some(step) => {
                    // add the step
                    retval.push(step);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::TurtleCollectToNextForwardIterator;
    use super::TurtleProgramIterator;
    use super::TurtleStep;

    #[test]
    fn test_collect_forward_iterator() {
        let base_iter = TurtleProgramIterator::new(Box::new(
            vec![
                TurtleStep::Forward(1.0),
                TurtleStep::TurnRad(9.0),
                TurtleStep::Forward(2.0),
            ]
            .into_iter(),
        ));
        let mut test_iter = TurtleCollectToNextForwardIterator { iter: base_iter };

        let first_vec = test_iter.next().expect("no first vector");
        assert_eq!(first_vec.len(), 1);
        assert_eq!(
            *first_vec.get(0).expect("no first_vec[0]"),
            TurtleStep::Forward(1.0)
        );

        let second_vec = test_iter.next().expect("no second vector");
        assert_eq!(second_vec.len(), 2);
        assert_eq!(
            *second_vec.get(0).expect("no second_vec[0]"),
            TurtleStep::TurnRad(9.0)
        );
        assert_eq!(
            *second_vec.get(1).expect("no second_vec[1]"),
            TurtleStep::Forward(2.0)
        );

        assert!(test_iter.next().is_none());
    }

    #[test]
    fn test_collect_forward_iterator_empty() {
        let base_iter = TurtleProgramIterator::new(Box::new(vec![].into_iter()));
        let mut test_iter = TurtleCollectToNextForwardIterator { iter: base_iter };

        assert!(test_iter.next().is_none());
    }

    #[test]
    fn test_collect_forward_iterator_actions_after_last_forward() {
        let base_iter = TurtleProgramIterator::new(Box::new(
            vec![
                TurtleStep::Forward(1.0),
                TurtleStep::TurnRad(9.0),
                TurtleStep::Forward(2.0),
                TurtleStep::TurnRad(-1.0),
            ]
            .into_iter(),
        ));
        let mut test_iter = TurtleCollectToNextForwardIterator { iter: base_iter };

        test_iter.next();
        test_iter.next();
        let lacks_forward = test_iter.next().expect("no third vec");
        assert_eq!(lacks_forward.len(), 1);
        assert_eq!(
            *lacks_forward.get(0).expect("no lacks_forward[0]"),
            TurtleStep::TurnRad(-1.0)
        );
        assert!(test_iter.next().is_none());
    }
}
