# 01 · Prompt symbol

Goal: print `❯` in magenta after success and in red after failure.

## Cargo

- **Cargo** is Rust's build tool and package manager. `cargo init` creates `Cargo.toml` and `src/main.rs`.
- **Crate** means a Rust package. This crate is a binary named `pure`.
- **Edition** (`2024`) picks the language rule set. Editions stay compatible with each other.
- `cargo test` runs every `#[test]` function. `cargo clippy` is the linter. `cargo fmt` formats.

## Modules

- `mod color;` in `main.rs` loads `src/color.rs` as the module `color`.
- Items are private by default. `pub` makes them visible to other modules.
- `use crate::color::Color;` imports a name. `crate` means the root of this crate.

## `String` vs `&str`

- `String` owns its text on the heap. You can grow it.
- `&str` borrows text that someone else owns. It is read-only.
- `render(…, symbol: &str)` borrows the symbol. `main` keeps ownership and passes `&symbol`.
- Rule of thumb: take `&str` as input, return `String` as output.

## `enum` and `match`

- `enum Color { Red, Magenta }` lists every possible colour.
- `match self { … }` must cover every variant. The compiler rejects a missing case.
- `impl Color { … }` attaches methods. `self` is the value the method runs on.
- `#[derive(Clone, Copy, …)]` asks the compiler to write common traits for you. `Copy` lets you pass a `Color` without moving it.

## `if` is an expression

`let color = if status == 0 { Magenta } else { Red };` assigns the value of the branch.
A block without a trailing `;` returns its last expression. That is why `render` has no `return`.

## Errors with `Result` and `?`

- `Result<T, E>` is either `Ok(value)` or `Err(error)`. Rust has no exceptions.
- `?` returns the error early if there is one, otherwise unwraps the value.
- `Option<T>` is either `Some(value)` or `None`. `ok_or(…)` turns `None` into an `Err`.
- `env::var(…)` returns a `Result`. `unwrap_or_else(…)` supplies a default.

## Iterators

- `env::args()` yields the program arguments as an iterator. `.skip(1)` drops the program name.
- `impl Iterator<Item = String>` means "any iterator of strings". Tests pass a fake list this way.
- `while let Some(arg) = args.next()` loops until the iterator returns `None`.

## Tests

- `#[cfg(test)] mod tests;` compiles only under `cargo test`. It has no body, so Rust loads the file for you.
- For `src/prompt_symbol.rs`, the file is `src/prompt_symbol/tests.rs`. For `src/main.rs`, it is `src/tests.rs`.
- `use super::*;` imports everything from the parent module, private items included.
- Unit tests in a child module can reach private items. Files in a top-level `tests/` folder are *integration tests* and see only `pub` items.

## Try it

```sh
cargo run -- --last-command-status 1
PURE_SYMBOL_PROMPT='>' cargo run
```

Exercise: support `PURE_SEPARATE_PROMPT_ON_ERROR=true`, as in `_pure_prompt_symbol.fish`.
