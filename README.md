# Compiler in Rust

[![Build Icon]][Build Status]&emsp;[![License Icon]][LICENSE]

[Build Icon]: https://img.shields.io/github/actions/workflow/status/1git2clone/compiler-in-rust/rust.yml?branch=main
[Build Status]: https://github.com/1git2clone/leetcode-trees-rs/actions?query=branch%3Amaster
[License Icon]: https://img.shields.io/badge/license-MIT-blue.svg
[LICENSE]: LICENSE

## Description

By some this could be could be considered an interpreter, in fact, you can use
it as one if you want to.

Currently supports the following features:

- [x] Addition
- [x] Subtraction
- [x] Multiplication
- [x] Division
- [x] Parentheses (_`(!) Partially`_)

It **doesn't** support:

- [ ] Nested parentheses
- [ ] Named variables
- [ ] Data types (different from `i32`'s)

### The bottom line

An over-glorified calculator.

## Getting started

You can add it as a git dependency in your `Cargo.toml` file. This project
isn't really meant to be a crate though, so expect breaking changes.

```toml
# /Cargo.toml
[dependencies]
compiler = { git = "https://github.com/1Git2Clone/compiler-in-rust/" }
```

```rs
// /src/main.rs
use compiler::prelude::*;

fn main() -> Result<(), Error> {
    println!("{}", compile(&parse(&tokenize("(3 + 2) * 6")?)?.unwrap())); // 30
}
```

> [!NOTE]
> This "compiler" is very limited, first of all it doesn't handle nested
> parentheses, nor named variables, nor any values other than `i32`.
