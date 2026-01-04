use blo_cli::colors::{BackgroundColor, Color, bg_colorize, fg_colorize};
use blo_cli::styles::{TextStyle, style};

fn main() {
    println!("{}", fg_colorize("Hallo Leute!", Color::Blue));
    println!("{}", bg_colorize("Hallo Leute!", BackgroundColor::Red));
    println!("{}", bg_colorize("Hallo Leute!", BackgroundColor::Green));

    println!("{}", style("Hallo Leute!", TextStyle::Bold));
    println!("{}", style("Hallo Leute!", TextStyle::Underline));
    println!("{}", style("Hallo Leute!", TextStyle::Italic));
}
