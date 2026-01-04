pub mod colors;
pub mod styles;

#[cfg(test)]
mod tests {
    use crate::colors::fg_colorize;
    use crate::styles::style;

    use super::*;

    #[test]
    fn color_blue_works() {
        let text = fg_colorize("Hallo", colors::Color::Blue);
        assert_eq!(text, "\x1b[34mHallo\x1b[0m".to_string());
    }

    #[test]
    fn color_pink_works() {
        let text = fg_colorize("Hallo", colors::Color::Pink);
        assert_eq!(text, "\x1b[35mHallo\x1b[0m".to_string());
    }

    #[test]
    fn color_cyan_works() {
        let text = fg_colorize("Hallo", colors::Color::Cyan);
        assert_eq!(text, "\x1b[36mHallo\x1b[0m".to_string());
    }

    #[test]
    fn color_white_works() {
        let text = fg_colorize("Hallo", colors::Color::White);
        assert_eq!(text, "\x1b[37mHallo\x1b[0m".to_string());
    }

    #[test]
    fn color_black_works() {
        let text = fg_colorize("Hallo", colors::Color::Black);
        assert_eq!(text, "\x1b[30mHallo\x1b[0m".to_string());
    }

    #[test]
    fn style_bold_works() {
        let text = style("Hallo", styles::TextStyle::Bold);
        assert_eq!(text, "\x1b[1mHallo\x1b[0m".to_string());
    }

    #[test]
    fn style_underline_works() {
        let text = style("Hallo", styles::TextStyle::Underline);
        assert_eq!(text, "\x1b[4mHallo\x1b[0m".to_string());
    }

    #[test]
    fn style_italic_works() {
        let text = style("Hallo", styles::TextStyle::Italic);
        assert_eq!(text, "\x1b[3mHallo\x1b[0m".to_string());
    }
}
