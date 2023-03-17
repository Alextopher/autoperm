//! Autoperm is a tool for generating programs to apply stack effect diagrams.
//!
//! It is backend agnostic and could be used to generate programs for any language as long
//! as the language implements the [`Model`](model::Model) trait.
//!
//! A [brainfuck](https://en.wikipedia.org/wiki/Brainfuck) backend is provided and accessible
//! with [`autoperm_bf`](crate::autoperm_bf`).
//!
//! ## Binary
//!
//! Installing the crate as a binary gives access to the `autoperm` command which uses this brainfuck backend as REPL.
//!
//! ```test
//! cargo install autoperm
//! ```
//!
//! **Usage:**
//!
//! ```bf
//! $ autoperm a b -- b a
//! [->+<]<[->+<]>>[-<<+>>]<
//!
//! $ autoperm
//! a b c -- c a b
//! [->+<]<[->+<]<[->+<]>>>[-<<<+>>>]<
//!
//! a -- a a a a
//! [->>>>+<<<<]>>>>[-<+<+<+<+>>>>]<
//!
//! a b c d -- d c a b
//! [->+<]<<[->>+<<]>[-<+>]<<[->>+<<]>>>>[-<<<<+>>>>]<
//!
//! a b c -- c
//! <<[-]>[-]>[-<<+>>]<<
//!
//! a b c d e f -- c d d f e e b
//! <<<<<[-]>[->>>>>+<<<<<]>[-<<+>>]>[-<+<+>>]>>[-<<+>>]<[->>>+<<<]>>>[-<<+<+>>>]<
//!
//! ```
//!
//! The program assumes the memory pointer starts by pointing at the top of the stack.
//! Any "new" cells (cells that are not defined in the input) should start empty.
//! There must also be 1 free cell at the top of the stack for temporary storage.
//!
//! For example:
//! ```bf
//! (a b c -- c)
//! start must be:
//!   a  b *c  0 // a and b are cleared
//! <<[-]>[-]>[-<<+>>]<<
//! end:
//!  *c  0  0  0
//!
//! (a -- a a a a)
//! start must be:
//!   a  0  0  0  0 // note: no 0s are initialized before usage
//! [->>>>+<<<<]>>>>[-<+<+<+<+>>>>]<
//! end:
//!   a  a  a *a  0
//! ```
//!
//! A walk through for (a b -- a b a b)
//!
//! ```bf
//! a b -- a b a b
//! <[->>>>+<<<<]>>>>[-<<+<<+>>>>]<<<[->>>+<<<]>>>[-<+<<+>>>]<
//!
//! # the tape
//!  0 *1  2  3  T
//!  a  b  0  0  0
//!
//! <[->>>>+<<<<]      0 → {T}
//! *0  1  2  3  T
//!  0  b  0  0  a
//!
//! >>>>[-<<+<<+>>>>]  T → {2 0}
//!  0  1  2  3 *T
//!  a  b  a  0  0
//!
//! <<<[->>>+<<<]      1 → {T}
//!  0 *1  2  3  T
//!  a  0  a  0  b
//!
//! >>>[-<+<<+>>>]     T → {1 3}
//!  0  1  2  3 *T
//!  a  b  a  b  0
//!
//! <
//!  0  1  2 *3  T
//!  a  b  a  b  0
//! ```
#![warn(missing_docs)]

mod model;
mod parse;
mod solve;
use models::Brainfuck;

pub mod models;
pub use model::Model;
pub use parse::{parse, ParseError, StackEffectDiagram};
pub use solve::{solve, Instruction};

/// Generate a brainfuck program that applies a given [`StackEffectDiagram`](crate::StackEffectDiagram)
///
/// # Examples
///
/// ```
/// use autoperm::autoperm_bf;
///
/// let program = autoperm_bf("a b -- b a");
///
/// assert_eq!(program, Ok("[->+<]<[->+<]>>[-<<+>>]<".to_string()));
/// ```
pub fn autoperm_bf(stack_effect: &str) -> Result<String, ParseError> {
    autoperm(stack_effect, Brainfuck::new())
}

/// Generate a program to apply a given [`StackEffectDiagram`](crate::StackEffectDiagram).
///
/// This function is backend agnostic and can be used to generate programs for any language.
///
/// See: [`Model`](crate::Model).
///
/// # Examples
///
/// ```
/// use autoperm::autoperm;
/// use autoperm::models::Brainfuck;
///
/// let model = Brainfuck::new();
/// let program = autoperm("a b -- b a", model);
///
/// assert_eq!(program, Ok("[->+<]<[->+<]>>[-<<+>>]<".to_string()));
/// ```
pub fn autoperm<M>(stack_effect: &str, model: M) -> Result<M::Output, ParseError>
where
    M: Model,
{
    let diagram = parse(stack_effect)?;

    let instructions = solve(&diagram);

    // For each instruction send it to the backend to generate the program
    Ok(generate(instructions, model))
}

/// Generate a program from a list of [`Instruction`](crate::Instruction)s using a given [`Model`](crate::Model).
pub fn generate<M>(instructions: Vec<Instruction>, mut model: M) -> M::Output
where
    M: Model,
{
    instructions
        .into_iter()
        .for_each(|instruction| match instruction {
            Instruction::Clear { cell } => model.clear(cell),
            Instruction::Mov { cell, to } => model.mov(cell, to),
            Instruction::Start { cell } => model.start(cell),
            Instruction::Top { cell } => model.top(cell),
        });

    model.finish()
}

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[cfg(test)]
mod test;
