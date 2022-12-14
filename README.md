# autoperm

autoperm is a tool for generating brainfuck programs that apply [stack effect diagrams](https://en.wikipedia.org/wiki/Stack-oriented_programming#Stack_effect_diagrams). The produced result has the fewest number of _loops_ and it's foundation is [Tarjan's Strongly Connected Components Algorithm](https://en.wikipedia.org/wiki/Tarjan%27s_strongly_connected_components_algorithm). 

Eventually I will create a proper write up but for now here is my quick [explanation](./explanation.md).

## Install

```test
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

There are also opportunities to use the project as a library. This is currently **unstable** and is a work in progress.

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

A walk through for (a b -- a b a b)

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

