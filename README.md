# autoperm

autoperm is a tool for generating brainfuck programs that apply [stack effect diagrams](https://en.wikipedia.org/wiki/Stack-oriented_programming#Stack_effect_diagrams). 

## Examples

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

## Constraints

The program assumes the memory pointers is pointing at the top of the stack. Any new cells should start empty and there must be 1 free cell at the top of the stack for temporary storage.

For example walking through (a b -- a b a b)
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
