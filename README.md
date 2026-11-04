# pure.rs

> A Rust port of [Pure](https://github.com/pure-fish/pure), the Fish prompt. Built one feature at a time to learn Rust.

## Usage

```sh
cargo build --release
./target/release/pure --last-command-status 0
```

Configuration uses `PURE_*` environment variables, like Fish's `pure_*` variables in upper case.

| Variable | Default | Effect |
| :------- | :------ | :----- |
| `PURE_SYMBOL_PROMPT` | `❯` | Symbol that ends the prompt |

## Features

| Feature | Status |
| :------ | :----- |
| `prompt_symbol` (+ colors, config) | ✅ |
| `new_line`, `current_folder` | ⏳ |
| `git_branch`, `git_dirty` | ⏳ |

Black-box specs live in [pure-fish/agnostic-tests](https://github.com/pure-fish/agnostic-tests).

## Learning notes

Each feature has a short note on the Rust concepts it uses: [docs/learning/](docs/learning/).

## Development

```sh
cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test
```
