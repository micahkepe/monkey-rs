# monkey-rs 🦀

[![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/micahkepe/monkey-rs/rust.yml?logo=github)](https://github.com/micahkepe/monkey-rs/actions)

<div align="center">
    <img src="./assets/img/monkey-rs.png" width="25%" alt="monkey-rs logo"/>
</div>

A Rust implementation of the [Monkey](https://monkeylang.org/) programming
language from Thorsten Ball's [Writing An Interpreter In Go](https://interpreterbook.com/).

> _"But why the name? Why is it called “Monkey”? Well, because monkeys are
> magnificent, elegant, fascinating and funny creatures. Exactly like our
> interpreter"_ — Thorsten Ball

## Quick Start

To run the Monkey REPL:

```bash
cargo run
```

To run tests:

```bash
cargo test
```

## Documentation

Comprehensive documentation is available as an mdbook. To build and view the documentation:

```bash
cd docs
mdbook serve
```

The documentation covers:

- Language syntax and features
- Data types and operations
- Built-in functions
- REPL usage
- Functional programming patterns

## TODOs

- [x] Lexer
- [x] Parser
- [x] Evaluation
- [x] Extending the Interpreter
- [x] Documentation with [`mdbook`](https://github.com/rust-lang/mdBook)
- [ ] Macro System ("The Lost Chapter")
- [ ] Support multi-line input in REPL with `rustyline`

## License

This repository is licensed under the MIT License. See [LICENSE](./LICENSE) for
more details.
