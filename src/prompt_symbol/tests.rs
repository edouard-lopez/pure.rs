use super::*;

#[test]
fn magenta_after_success() {
    assert_eq!(render(0, "❯"), "\x1b[35m❯\x1b[0m");
}

#[test]
fn red_after_failure() {
    assert_eq!(render(1, "❯"), "\x1b[31m❯\x1b[0m");
}

#[test]
fn custom_symbol() {
    assert_eq!(render(0, ">"), "\x1b[35m>\x1b[0m");
}
