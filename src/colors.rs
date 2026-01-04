/// Foreground colors for terminal text styling.
///
/// Provides 8 standard terminal colors that can be applied to text.
#[derive(Copy, Clone)]
pub enum Color {
    /// Red color
    Red,
    /// Green color
    Green,
    /// Blue color
    Blue,
    /// Yellow color
    Yellow,
    /// Magenta/Pink color
    Pink,
    /// Cyan color
    Cyan,
    /// White color
    White,
    /// Black color
    Black,
}

/// Background colors for terminal text styling.
///
/// Provides 8 standard terminal colors for text backgrounds.
#[derive(Copy, Clone)]
pub enum BackgroundColor {
    /// Red background
    Red,
    /// Green background
    Green,
    /// Blue background
    Blue,
    /// Yellow background
    Yellow,
    /// Magenta/Pink background
    Pink,
    /// Cyan background
    Cyan,
    /// White background
    White,
    /// Black background
    Black,
}

impl Color {
    /// Returns the ANSI escape code for this foreground color.
    pub fn code(&self) -> &str {
        match self {
            Color::Red => "\x1b[31m",
            Color::Green => "\x1b[32m",
            Color::Blue => "\x1b[34m",
            Color::Yellow => "\x1b[33m",
            Color::Pink => "\x1b[35m",
            Color::Cyan => "\x1b[36m",
            Color::White => "\x1b[37m",
            Color::Black => "\x1b[30m",
        }
    }
}

impl BackgroundColor {
    /// Returns the ANSI escape code for this background color.
    pub fn code(&self) -> &str {
        match self {
            BackgroundColor::Red => "\x1b[41m",
            BackgroundColor::Green => "\x1b[42m",
            BackgroundColor::Blue => "\x1b[44m",
            BackgroundColor::Yellow => "\x1b[43m",
            BackgroundColor::Pink => "\x1b[45m",
            BackgroundColor::Cyan => "\x1b[46m",
            BackgroundColor::White => "\x1b[47m",
            BackgroundColor::Black => "\x1b[40m",
        }
    }
}

/// Applies a foreground color to text using ANSI escape codes.
///
/// # Arguments
/// * `text` - The text to colorize
/// * `color` - The foreground color to apply
///
/// # Example
/// ```
/// use blo_cli::colors::{Color, fg_colorize};
/// let colored = fg_colorize("Hello", Color::Blue);
/// ```
pub fn fg_colorize(text: &str, color: Color) -> String {
    format!("{}{}{}", color.code(), text, "\x1b[0m")
}

/// Applies a background color to text using ANSI escape codes.
///
/// # Arguments
/// * `text` - The text to colorize
/// * `color` - The background color to apply
///
/// # Example
/// ```
/// use blo_cli::colors::{BackgroundColor, bg_colorize};
/// let colored = bg_colorize("Hello", BackgroundColor::Red);
/// ```
pub fn bg_colorize(text: &str, color: BackgroundColor) -> String {
    format!("{}{}{}", color.code(), text, "\x1b[0m")
}
