use blo_cli::colors::{BackgroundColor, Color, bg_colorize, fg_colorize};
use blo_cli::styles::{TextStyle, style};

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
}
