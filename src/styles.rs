pub enum TextStyle {
    Bold,
    Italic,
    Underline,
}

impl TextStyle {
    fn code(&self) -> &str {
        match self {
            TextStyle::Bold => "\x1b[1m",
            TextStyle::Italic => "\x1b[3m",
            TextStyle::Underline => "\x1b[4m",
        }
    }
}

pub fn style(text: &str, style: TextStyle) -> String {
    format!("{}{}{}", style.code(), text, "\x1b[0m")
}
