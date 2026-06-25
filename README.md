# C Compiler in Rust

A C compiler written in Rust, following the ["Write a Compiler"](https://norasandler.com/2017/11/29/Write-a-Compiler.html) tutorial series by Nora Sandler. It compiles a subset of C into x86 assembly, built incrementally — one language feature at a time.

## How it works

The compiler is structured as three sequential stages:

```
C source → [Lexer] → Tokens → [Parser] → AST → [Code Gen] → x86 Assembly
```

1. **Lexer** — scans the source file and produces a flat list of tokens (`int`, `return`, identifiers, integer literals, punctuation).
2. **Parser** — consumes the token stream and builds an Abstract Syntax Tree via recursive descent.
3. **Code Generation** — walks the AST and emits x86 assembly instructions.

## Building

Requires the [Rust toolchain](https://rustup.rs/).

```bash
cargo build
```

## Roadmap

Following the tutorial series, the planned feature milestones are:

- [x] Lexer
- [ ] Parser & AST
- [ ] Code generation (return integer)
- [ ] Unary operators (`-`, `~`, `!`)
- [ ] Binary operators and arithmetic
- [ ] Conditionals (`if`/`else`, ternary)
- [ ] Local variables
- [ ] Function calls

## Reference

- [Write a Compiler — Nora Sandler](https://norasandler.com/2017/11/29/Write-a-Compiler.html)
