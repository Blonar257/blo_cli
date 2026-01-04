# blo_cli

Eine einfache Rust-Bibliothek für das Styling von Terminal-Text mit ANSI-Escape-Codes.

## Features

- **Farben**: Unterstützt Vordergrundfarben (Red, Green, Blue, Yellow, Pink, Cyan, White, Black) und Hintergrundfarben.
- **Text-Styles**: Bold, Italic, Underline.

## Installation

Füge die Bibliothek zu deinem Projekt hinzu:

```bash
cargo add blo_cli
```

Oder klone das Repository und baue es lokal:

```bash
git clone https://github.com/yourusername/blo_cli.git
cd blo_cli
cargo build
```

## Verwendung

### Beispiel

```rust
use blo_cli::colors::{Color, fg_colorize};
use blo_cli::styles::{TextStyle, style};

fn main() {
    // Text in Blau farben
    println!("{}", fg_colorize("Hallo Welt!", Color::Blue));

    // Text fett machen
    println!("{}", style("Hallo Welt!", TextStyle::Bold));

    // Text unterstreichen
    println!("{}", style("Hallo Welt!", TextStyle::Underline));
}
```

### Vollständiges Demo

Siehe `examples/demo.rs` für mehr Beispiele, einschließlich Hintergrundfarben.

## Tests

Führe die Tests aus:

```bash
cargo test
```

## Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert.