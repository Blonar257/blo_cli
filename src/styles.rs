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

pub enum BorderStyle {
    Angular,
    Rounded,
}

pub fn style(text: &str, style: TextStyle) -> String {
    format!("{}{}{}", style.code(), text, "\x1b[0m")
}

pub fn border(
    text: &str,
    style: BorderStyle,
    border_color: Option<crate::colors::Color>,
) -> String {
    if text.is_empty() {
        return String::new();
    }

    let lines: Vec<&str> = text.lines().collect();
    let max_width = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);

    if max_width == 0 {
        return String::new();
    }

    let (top_left, top_right, bottom_left, bottom_right, side, horizontal) = match style {
        BorderStyle::Angular => ("┌", "┐", "└", "┘", "│", "─"),
        BorderStyle::Rounded => ("╭", "╮", "╰", "╯", "│", "─"),
    };

    let color_start = match border_color {
        Some(crate::colors::Color::Red) => "\x1b[31m",
        Some(crate::colors::Color::Green) => "\x1b[32m",
        Some(crate::colors::Color::Blue) => "\x1b[34m",
        Some(crate::colors::Color::Yellow) => "\x1b[33m",
        Some(crate::colors::Color::Pink) => "\x1b[35m",
        Some(crate::colors::Color::Cyan) => "\x1b[36m",
        Some(crate::colors::Color::White) => "\x1b[37m",
        Some(crate::colors::Color::Black) => "\x1b[30m",
        None => "",
    };
    let color_end = if border_color.is_some() {
        "\x1b[0m"
    } else {
        ""
    };

    let top = format!(
        "{}{}{}{}{}",
        color_start,
        top_left,
        horizontal.repeat(max_width),
        top_right,
        color_end
    );
    let bottom = format!(
        "{}{}{}{}{}",
        color_start,
        bottom_left,
        horizontal.repeat(max_width),
        bottom_right,
        color_end
    );

    let middle: Vec<String> = lines
        .iter()
        .map(|line| {
            let padded_line = format!("{:width$}", line, width = max_width);
            let left = format!("{}{}{}", color_start, side, color_end);
            let right = format!("{}{}{}", color_start, side, color_end);
            left + &padded_line + &right
        })
        .collect();

    let mut result = vec![top];
    result.extend(middle);
    result.push(bottom);
    result.join("\n")
}
