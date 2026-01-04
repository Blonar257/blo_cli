use blo_cli::colors::{BackgroundColor, Color, bg_colorize, fg_colorize};
use blo_cli::styles::{BorderStyle, TextStyle, border, style};

fn main() {
    println!("{}", fg_colorize("Hallo Leute!", Color::Blue));
    println!("---");
    println!("{}", bg_colorize("Hallo Leute!", BackgroundColor::Red));
    println!("---");
    println!("{}", bg_colorize("Hallo Leute!", BackgroundColor::Green));
    println!("---");
    println!("{}", style("Hallo Leute!", TextStyle::Bold));
    println!("---");
    println!("{}", style("Hallo Leute!", TextStyle::Underline));
    println!("---");
    println!("{}", style("Hallo Leute!", TextStyle::Italic));
    println!("---");
    println!("{}", border("Hallo Leute!", BorderStyle::Angular, None));
    println!("---");
    println!(
        "{}",
        border("Hallo Leute!", BorderStyle::Rounded, Some(Color::Green))
    );
    println!("---");
    println!(
        "{}",
        border("Mehrzeiliger\nText", BorderStyle::Angular, Some(Color::Red))
    );
    println!("---");
    println!(
        "{}",
        border(
            "Mehrzeiliger\nText",
            BorderStyle::Rounded,
            Some(Color::Blue)
        )
    );
    println!("---");
    println!(
        "{}",
        border(
            "Dies ist ein weiterer\nmehrzeiliger Text.\nHoffe es klappt!",
            BorderStyle::Rounded,
            None,
        )
    );
    println!("---");
    let test = format!("{}", fg_colorize("Hallo Leute!", Color::Blue));
    println!(
        "{}",
        border(test.as_str(), BorderStyle::Rounded, Some(Color::Black))
    );
}
