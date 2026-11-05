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
