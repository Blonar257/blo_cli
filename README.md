# blo_cli

A simple Rust library for styling terminal text with ANSI escape codes.

## Features

- **Colors**: Supports foreground colors (Red, Green, Blue, Yellow, Pink, Cyan, White, Black) and background colors.
- **Text Styles**: Bold, Italic, Underline.
- **Borders**: Create borders around text with Angular or Rounded style, optionally with colors.

## Installation

Add the library to your project:

```bash
cargo add blo_cli
```

Or clone the repository and build it locally:

```bash
git clone https://github.com/Blonar257/blo_cli.git
cd blo_cli
cargo build
```

## Usage

### Example

```rust
use blo_cli::colors::Color;
use blo_cli::colors::{fg_colorize, bg_colorize};
use blo_cli::styles::{TextStyle, style, BorderStyle, border};

fn main() {
    // Color text in blue
    println!("{}", fg_colorize("Hello World!", Color::Blue));

    // Make text bold
    println!("{}", style("Hello World!", TextStyle::Bold));

    // Underline text
    println!("{}", style("Hello World!", TextStyle::Underline));

    // Text with angular border
    println!("{}", border("Hello World!", BorderStyle::Angular, None));

    // Text with rounded border in red
    println!("{}", border("Hello World!", BorderStyle::Rounded, Some(Color::Red)));
}
```

### Border Functions

The `border()` function creates a border around text:

```rust
use blo_cli::styles::{BorderStyle, border};
use blo_cli::colors::Color;

// Simple angular border (without color)
println!("{}", border("Info", BorderStyle::Angular, None));

// Rounded border in red
println!("{}", border("Warning", BorderStyle::Rounded, Some(Color::Red)));

// Multi-line text with border
println!("{}", border("Line 1\nLine 2", BorderStyle::Angular, Some(Color::Green)));
```

**BorderStyle Options:**
- `BorderStyle::Angular`: Angular corners (┌─┐│└┘)
- `BorderStyle::Rounded`: Rounded corners (╭─╮│╰╯)

**Colors:** All colors (Red, Green, Blue, Yellow, Pink, Cyan, White, Black) are supported.

### Complete Demo

See `examples/demo.rs` for more examples, including background colors and borders.

## Tests

Run the tests:

```bash
cargo test
```

## License

This project is licensed under the MIT License.