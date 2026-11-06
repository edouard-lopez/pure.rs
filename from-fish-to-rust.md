# From Fish to Rust

How to port a [pure-fish/pure] feature to pure.rs.
Read [How Pure works][how-it-works] first: it explains the Fish pipeline.
The [gap matrix](README.md#gap-matrix) lists what is left to port.
Rust concepts per feature live in [docs/learning/](docs/learning/).

## Mental model

| Fish Pure | pure.rs |
| :-------- | :------ |
| Fish calls `fish_prompt` before each prompt. | The shell runs the `pure` binary before each prompt. |
| A segment is a function `_pure_prompt_<name>`. | A segment is a module `src/<name>.rs` with a `render(...) -> String` function. |
| Segments read Fish variables (`$pipestatus`, `$CMD_DURATION`). | The binary cannot see them. The shell passes them as flags or env vars. |
| Settings are `pure_*` variables. | Settings are `PURE_*` env vars, read with `std::env::var`. |
| `set_color` prints colours. | `color::Color::paint` writes ANSI codes. |
| Fishtape tests per function. | `#[cfg(test)] mod tests;` per module, plus [agnostic-tests] for the binary. |

## Idioms

| Fish | Rust |
| :--- | :--- |
| `set --local name value` | `let name = value;` |
| `set --query var; and test "$var" = true` | `env::var("PURE_VAR").as_deref() == Ok("true")` |
| `test -n "$x"` | `!x.is_empty()` |
| `(command git status --porcelain)` | `Command::new("git").args(["status", "--porcelain"]).output()?` |
| `string replace --regex 'a' 'b' $s` | `s.replace("a", "b")` (plain) or the `regex` crate |
| `string split '/' $p \| tail -n 2` | `p.rsplit('/').take(2)` then reverse and `join("/")` |
| `string length` on visible text | strip ANSI, then `chars().count()` (or `unicode-width` for emoji) |
| `math "$ms / 1000"` | `Duration::from_millis(ms)` |
| `return 1` (failure) | `Option<String>`: `None` hides a segment; `Result` for real errors |

## Recipe for one segment (TDD)

1. **Pick** one row of the gap matrix, in phase order.
2. **Read** the Fish function and its test: `functions/_pure_prompt_<name>.fish`, `tests/_pure_prompt_<name>.test.fish`.
3. **Red**: copy each Fishtape case into `src/<name>/tests.rs`. Run `cargo test` and see it fail.
4. **Green**: write the smallest `render` that passes. Take inputs as arguments (`&str`, `i32`), not from `env`.
5. **Wire**: read env vars and flags in `main.rs`, then call `render`. Keep the Fish order.
6. **Spec**: add an `adapters/rust` to [agnostic-tests] (P0), then add or update the black-box spec.
7. **Refactor**, run `cargo fmt && cargo clippy --all-targets -- -D warnings`, then update the gap matrix row.
8. **Learn**: add a short `docs/learning/NN-<name>.md` note.

## Shell state contract

| Needed by | Fish source | Proposed input |
| :-------- | :---------- | :------------- |
| symbol, exit statuses | `$pipestatus` | `--last-command-status N` (exists), later `--pipestatus "0 1"` |
| command duration | `$CMD_DURATION` | `PURE_CMD_DURATION` (ms) |
| folder shortening | `$COLUMNS` | `COLUMNS` |
| jobs | `jobs -p` | `--jobs N` |
| vi mode | `$fish_bind_mode` | `--bind-mode insert` |
| transient prompt | `--final-rendering` | `--final-rendering` |

Agree on each name in [agnostic-tests] before you code it, so Python and Go use the same flag.

## Pitfalls

* **Dependencies**: prefer `std` first. Add a crate (for example `clap`, `unicode-width`) only when a test needs it.
* **Speed**: each `git` call costs time. Batch with `git status --porcelain=v2 --branch` when you add ahead/behind.
* **Width**: measure the visible width without ANSI codes, like `_pure_string_width`.
* **Colours**: Fish settings point to a base colour by name (`pure_color_git_branch=pure_color_mute`). Model base colours as an `enum` and resolve names once.
* **Never panic** in a segment. A panic breaks the user's shell prompt; return `None` instead.

[pure-fish/pure]: https://github.com/pure-fish/pure
[how-it-works]: https://pure-fish.github.io/pure/doc/#how-it-works
[agnostic-tests]: https://github.com/pure-fish/agnostic-tests
