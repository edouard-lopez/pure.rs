//! Prompt symbol segment: the `❯` that ends the prompt.
//! Port of pure-fish `functions/_pure_prompt_symbol.fish`.

use crate::color::Color;

/// Default symbol, like Fish's `pure_symbol_prompt`.
pub const DEFAULT_SYMBOL: &str = "❯";

/// Return the coloured prompt symbol.
///
/// The symbol is magenta after a success and red after a failure.
pub fn render(last_command_status: i32, symbol: &str) -> String {
    let color = if last_command_status == 0 {
        Color::Magenta
    } else {
        Color::Red
    };
    color.paint(symbol)
}

#[cfg(test)]
mod tests;
