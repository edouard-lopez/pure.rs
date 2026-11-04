//! Terminal colours as ANSI escape codes.

/// The colours Pure uses. Each variant maps to one ANSI code.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Red,
    Magenta,
}

impl Color {
    /// ANSI foreground code, e.g. `35` for magenta.
    fn code(self) -> u8 {
        match self {
            Color::Red => 31,
            Color::Magenta => 35,
        }
    }

    /// Wrap `text` in this colour, then reset to normal.
    pub fn paint(self, text: &str) -> String {
        format!("\x1b[{}m{}\x1b[0m", self.code(), text)
    }
}
