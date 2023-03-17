//! Predefined models for certain programming languages
//!
//! Currently only Brainfuck is supported

use std::cmp::Ordering;

use crate::Model;

/// This crate was originally created for Brainfuck.
///
/// This Brainfuck [`Model`](crate::Model) is included for backwards compatibility and demonstration purposes.
pub struct Brainfuck {
    program: String,
    ptr: isize,
}

impl Brainfuck {
    /// Creates a new model
    ///
    /// The `ptr` argument is the initial pointer position.
    pub fn new() -> Self {
        Self {
            program: String::new(),
            ptr: 0,
        }
    }

    fn shift_to(&mut self, cell: isize) {
        let diff = cell - self.ptr;
        match diff.cmp(&0) {
            Ordering::Less => self.program += &"<".repeat(diff.unsigned_abs()),
            Ordering::Equal => (),
            Ordering::Greater => self.program += &">".repeat(diff.unsigned_abs()),
        }
        self.ptr = cell;
    }
}

impl Default for Brainfuck {
    fn default() -> Self {
        Self::new()
    }
}

impl Model for Brainfuck {
    type Output = String;

    fn start(&mut self, cell: isize) {
        self.ptr = cell;
    }

    fn clear(&mut self, cell: isize) {
        self.shift_to(cell);
        self.program += "[-]";
    }

    fn mov(&mut self, cell: isize, to: Vec<isize>) {
        self.shift_to(cell);

        self.program += "[-";

        for position in to {
            self.shift_to(position);
            self.program += "+";
        }

        self.shift_to(cell);

        self.program += "]";
    }

    fn top(&mut self, cell: isize) {
        self.shift_to(cell);
    }

    fn finish(self) -> Self::Output {
        self.program
    }
}
