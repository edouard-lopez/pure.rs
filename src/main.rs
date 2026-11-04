//! pure.rs: a Rust port of the Pure prompt for Fish.
//!
//! Contract (see pure-fish/agnostic-tests):
//!   pure --last-command-status N
//! Configuration comes from `PURE_*` environment variables.

mod color;
mod prompt_symbol;

use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let last_command_status = match parse_last_command_status(env::args().skip(1)) {
        Ok(status) => status,
        Err(message) => {
            eprintln!("pure: {message}");
            return ExitCode::from(2);
        }
    };

    let symbol = env::var("PURE_SYMBOL_PROMPT")
        .unwrap_or_else(|_| prompt_symbol::DEFAULT_SYMBOL.to_string());

    print!("{} ", prompt_symbol::render(last_command_status, &symbol));
    ExitCode::SUCCESS
}

/// Read `--last-command-status N` from the arguments. Defaults to 0.
fn parse_last_command_status(mut args: impl Iterator<Item = String>) -> Result<i32, String> {
    let mut status = 0;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--last-command-status" => {
                let value = args.next().ok_or("--last-command-status needs a value")?;
                status = value
                    .parse()
                    .map_err(|_| format!("invalid status '{value}'"))?;
            }
            other => return Err(format!("unknown argument '{other}'")),
        }
    }
    Ok(status)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(list: &[&str]) -> impl Iterator<Item = String> {
        list.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .into_iter()
    }

    #[test]
    fn status_defaults_to_zero() {
        assert_eq!(parse_last_command_status(args(&[])), Ok(0));
    }

    #[test]
    fn status_is_read() {
        assert_eq!(
            parse_last_command_status(args(&["--last-command-status", "1"])),
            Ok(1)
        );
    }

    #[test]
    fn missing_value_is_an_error() {
        assert!(parse_last_command_status(args(&["--last-command-status"])).is_err());
    }

    #[test]
    fn unknown_argument_is_an_error() {
        assert!(parse_last_command_status(args(&["--nope"])).is_err());
    }
}
