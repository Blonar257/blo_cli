pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Pink,
    Cyan,
    White,
    Black,
}

pub enum BackgroundColor {
    Red,
    Green,
    Blue,
    Yellow,
    Pink,
    Cyan,
    White,
    Black,
}

impl Color {
    fn code(&self) -> &str {
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
    fn code(&self) -> &str {
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

pub fn fg_colorize(text: &str, color: Color) -> String {
    format!("{}{}{}", color.code(), text, "\x1b[0m")
}

pub fn bg_colorize(text: &str, color: BackgroundColor) -> String {
    format!("{}{}{}", color.code(), text, "\x1b[0m")
}
