[![Crates.io](https://img.shields.io/crates/v/autoperm.svg)](https://crates.io/crates/autoperm)
[![Documentation](https://docs.rs/autoperm/badge.svg)](https://docs.rs/autoperm/)
[![Dependency status](https://deps.rs/repo/github/Alextopher/autoperm/status.svg)](https://deps.rs/repo/github/Alextopher/autoperm)

# autoperm

autoperm is a tool for generating programs that apply [stack effect diagrams](https://en.wikipedia.org/wiki/Stack-oriented_programming#Stack_effect_diagrams). The produced result has the fewest number of `MOV`\* instructions. The algorithm has its foundations in [Tarjan's Strongly Connected Components](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm).

The library is backend agnostic. A [brainfuck](https://en.wikipedia.org/wiki/Brainfuck) backend is provided. Installing the crate as a binary gives access to the `autoperm` command which uses this brainfuck backend.

\* A `MOV` Clears the data at a particular memory address and writes it to a list of other cells

Checkout the [explanation](./explanation.md)!

## Install

```shell
cargo install autoperm
```

## Usage

```bf
$ autoperm a b -- b a
[->+<]<[->+<]>>[-<<+>>]<

$ autoperm
a b c -- c a b
[->+<]<[->+<]<[->+<]>>>[-<<<+>>>]<

a -- a a a a
[->>>>+<<<<]>>>>[-<+<+<+<+>>>>]<

a b c d -- d c a b
[->+<]<<[->>+<<]>[-<+>]<<[->>+<<]>>>>[-<<<<+>>>>]<

a b c -- c
<<[-]>[-]>[-<<+>>]<<

a b c d e f -- c d d f e e b
<<<<<[-]>[->>>>>+<<<<<]>[-<<+>>]>[-<+<+>>]>>[-<<+>>]<[->>>+<<<]>>>[-<<+<+>>>]<

```

The program assumes the memory pointer is pointing at the top of the stack. Any new cells should start empty and there must be 1 free cell at the top of the stack for temporary storage.

For example:

```bf
(a b c -- c)
start must be:
  a  b *c  0 // a and b are cleared
<<[-]>[-]>[-<<+>>]<<
end:
 *c  0  0  0

(a -- a a a a)
start must be:
  a  0  0  0  0 // note: no 0s are initialized before usage
[->>>>+<<<<]>>>>[-<+<+<+<+>>>>]<
end:
  a  a  a *a  0
```

A walk through of (a b -- a b a b)

```bf
a b -- a b a b
<[->>>>+<<<<]>>>>[-<<+<<+>>>>]<<<[->>>+<<<]>>>[-<+<<+>>>]<

# the tape
 0 *1  2  3  T 
 a  b  0  0  0

<[->>>>+<<<<]      0 → {T}
*0  1  2  3  T 
 0  b  0  0  a

>>>>[-<<+<<+>>>>]  T → {2 0}
 0  1  2  3 *T 
 a  b  a  0  0

<<<[->>>+<<<]      1 → {T}
 0 *1  2  3  T 
 a  0  a  0  b

>>>[-<+<<+>>>]     T → {1 3}
 0  1  2  3 *T 
 a  b  a  b  0

< 
 0  1  2 *3  T 
 a  b  a  b  0
```
