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

## Gap matrix

Parity with the reference [pure-fish/pure](https://github.com/pure-fish/pure) implementation.
See [How Pure works](https://pure-fish.github.io/pure/doc/#how-it-works) for the Fish internals, and the [From Fish to Rust](https://github.com/edouard-lopez/pure.rs/wiki/From-Fish-to-Rust) wiki page for the porting guide.

✅ done · 🟡 partial · ❌ missing. Phases match the repository milestones.

| Phase | Feature | Fish setting | Status |
| :---- | :------ | :----------- | :----- |
| P0 Foundation | CLI contract `--last-command-status N` | — | ✅ |
| P0 Foundation | Settings from `PURE_*` environment variables | `pure_*` | 🟡 1 setting |
| P0 Foundation | Nine base colours, configurable | `pure_color_*` | 🟡 magenta and red only |
| P0 Foundation | Agnostic tests in CI | — | ❌ |
| P1 Core prompt | Prompt symbol `❯` | `pure_symbol_prompt` | ✅ |
| P1 Core prompt | Symbol red after a failure, magenta after a success | `pure_color_prompt_on_*` | ✅ |
| P1 Core prompt | Separate error symbol | `pure_separate_prompt_on_error` | ❌ |
| P1 Core prompt | Exit statuses list, signal names | `pure_show_exit_status` | ❌ |
| P1 Core prompt | Current folder with `~` | `pure_color_current_directory` | ❌ |
| P1 Core prompt | Shorten and truncate folder | `pure_shorten_*`, `pure_truncate_*` | ❌ |
| P1 Core prompt | Blank line before the prompt | — | ❌ |
| P1 Core prompt | Single-line (compact) prompt | `pure_enable_single_line_prompt` | ❌ |
| P1 Core prompt | Folder first or last | `pure_begin_prompt_with_current_directory` | ❌ |
| P2 Git | Git branch (and detached HEAD) | `pure_color_git_branch` | ❌ |
| P2 Git | Git dirty `*` | `pure_symbol_git_dirty` | ❌ |
| P2 Git | Git stash `≡` | `pure_symbol_git_stash` | ❌ |
| P2 Git | Git ahead `⇡` / behind `⇣` | `pure_symbol_git_unpushed_commits` | ❌ |
| P2 Git | Numbered git indicators | `pure_show_numbered_git_indicator` | ❌ |
| P2 Git | Disable git | `pure_enable_git` | ❌ |
| P3 Context segments | Command duration | `pure_threshold_command_duration` | ❌ |
| P3 Context segments | `user@host` over SSH | `pure_symbol_ssh_prefix` | ❌ |
| P3 Context segments | Container detection | `pure_enable_container_detection` | ❌ |
| P3 Context segments | Python virtualenv / conda | `pure_enable_virtualenv` | ❌ |
| P3 Context segments | AWS profile | `pure_enable_aws_profile` | ❌ |
| P3 Context segments | Kubernetes context/namespace | `pure_enable_k8s` | ❌ |
| P3 Context segments | Nix dev shell | `pure_enable_nixdevshell` | ❌ |
| P3 Context segments | Background jobs | `pure_show_jobs` | ❌ |
| P3 Context segments | System time | `pure_show_system_time` | ❌ |
| P3 Context segments | Root prefix `#` | `pure_show_prefix_root_prompt` | ❌ |
| P4 Shell integration | Vi mode `❮` / `[N]` | `pure_reverse_prompt_symbol_in_vimode` | ❌ |
| P4 Shell integration | Private mode prefix `!` | `pure_show_prefix_private_prompt` | ❌ |
| P4 Shell integration | Transient prompt | `fish_transient_prompt` | ❌ |
| P4 Shell integration | Window title | `pure_symbol_title_bar_separator` | ❌ |
| P4 Shell integration | iTerm2 prompt mark | — | ❌ |
| P5 Release | Check for new release | `pure_check_for_new_release` | ❌ |

Black-box specs live in [pure-fish/agnostic-tests](https://github.com/pure-fish/agnostic-tests).

## Learning notes

Each feature has a short note on the Rust concepts it uses: [docs/learning/](docs/learning/).

## Development

```sh
cargo fmt && cargo clippy --all-targets -- -D warnings && cargo test
```
