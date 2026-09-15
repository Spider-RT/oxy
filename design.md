# Goals of Oxy
- Simple and close to the hardware (like C)
- Modern features such as:
    - proper structs, enums, tagged union
    - file/module management and importing
    - macros
    - compiler hints inlined in code
    - [Parametric Polymorphism](https://en.wikipedia.org/wiki/Parametric_polymorphism#Predicativity,_impredicativity,_and_higher-rank_polymorphism)
- an extendable, modular build system aimed at cross-compilation
- easy compatibility with C
- programmer has full control over software e.g.:
    - manual memory management
    - no runtime by default (has to specified in build) *maybe change this later unsure rigth now*

---
# Inspirations
Simplicity and Control: ==C==
Modern Features: ==Rust==
Build System: ==Zig==

---
# Philosophy
The Language is built around ==Definitions== and ==Expressions==
## Definitions
Code that describes structure, but not computation.
Examples:
- Function declaration
- Datastructures
- Constants
- Compiler flags
- Macros
- Variables
- Branching (If-Else, match, etc.)

## Expressions
Code that describse ==computation==, but not ==structure==
Examples:
- Function or Macro Call
- Math

---
# Syntax
## Definitions
Definitions are always described in this format:
```
<label> :: <def_type> <{} or ;>
main :: func() {}
Foo :: struct {}
PI :: const u8 3.14;
```
---
### Branching and Variable declaration
```
main :: func() {
    // i16 by default
    let x = 5;
    if x == 5 {
       ... 
    } else if x < 5 {
        ...
    } else {
        ...
    }

    switch x {
        5 :: {
            ...
        },

        6 :: { ... },
    }
}
```
---
### Datastructures
```
// Structs
Foo :: struct {
    x: u16,
    y: u16,
    z: u16,
}

// Enums (Results in tagged Union)
NumResult :: enum {
    Ok(u16),
    Err(NumErr),
}

Pet :: enum {
    Dog,
    Cat,
    Fish,
}
```
### Generics
```
Vec :: struct<T> {
    items: &[T];
    len: usize;
    cap: usize;

    // Generic function that accepts the structs single generic type T
    append :: func(&self, item: T) { ... }

    impl Iterator {
        type Item = T;

        next :: func(&self) &Self::Item { ... }
    }
}

Iterator :: trait {
    type Item;

    next :: func(&self) &Self::Item { ... }
}
```
---
### Module management
Filetree:
main.oxy
foo.oxy

```File: main.oxy
use foo::bar

main :: func() {
    bar()
}
```

```File: foo.oxy
bar :: func() { ... }
```
### Compiler Hints
Compiler hints can tell the compiler to process objects in certain ways or direct them to
a certain component in the compile-chain
```
// routes the path function through 'custom-component'
// when 'custom-component' runs is determined by the component itself
@custom-component
path :: func() { ... }
```
