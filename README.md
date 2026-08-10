# systemstd

This is an implementation using the standard library of `SystemIO` from the
[`systemio`](https://crates.io/crates/systemio) crate.

## Usage

This crate exposes the Zero-Sized Type `System` that just refers to the internals of the implementation.

```rust
use systemstd::System;

System::print("hello!");
```

For more information about usage and features, see [`systemio`](https://crates.io/crates/systemio).

## When to use it

Use this type when you want to have a `SystemIO` environment in a normal Rust program.

> [!WARNING] Libraries should **NOT** implement direct calls to a certain `SystemIO` implementations like `System`,
but allow the caller binary to provide one or return `Result<T, Issue>`.