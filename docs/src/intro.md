# Introduction

`monkey-rs` is a Rust implementation of the [Monkey](https://monkeylang.org/)
programming language from Thorsten Ball's
[Writing An Interpreter In Go](https://interpreterbook.com/).

> _"But why the name? Why is it called "Monkey"? Well, because monkeys are
> magnificent, elegant, fascinating and funny creatures. Exactly like our
> interpreter"_ — Thorsten Ball

## What is Monkey?

Monkey is a programming language designed to teach the fundamentals of interpreter construction. It features:

- **C-like syntax** that's familiar and easy to read
- **Variable bindings** with `let` statements
- **Integers and booleans** as basic data types
- **Arithmetic expressions** with standard operators
- **Built-in functions** for common operations
- **First-class and higher-order functions** supporting functional programming
- **Closures** that capture their environment
- **String data type** with built-in functions
- **Array data type** with built-in functions
- **Hash data type** for key-value storage

## Getting Started

To start using monkey-rs, you can run the interactive REPL:

```bash
cargo run
```

This will launch the Monkey REPL where you can experiment with the language interactively.

## Example Program

Here's a simple example of Monkey code:

```monkey
let fibonacci = fn(x) {
  if (x == 0) {
    0
  } else {
    if (x == 1) {
      1
    } else {
      fibonacci(x - 1) + fibonacci(x - 2);
    }
  }
};

fibonacci(10);
```

This documentation will guide you through all the features and capabilities of the Monkey programming language as implemented in Rust.
