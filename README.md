# blo_cli

Eine einfache Rust-Bibliothek für das Styling von Terminal-Text mit ANSI-Escape-Codes.

## Features

- **Farben**: Unterstützt Vordergrundfarben (Red, Green, Blue, Yellow, Pink, Cyan, White, Black) und Hintergrundfarben.
- **Text-Styles**: Bold, Italic, Underline.
- **Borders**: Erstelle Rahmen um Text mit Angular- oder Rounded-Stil, optional mit Farben.

## Installation

Füge die Bibliothek zu deinem Projekt hinzu:

```bash
cargo add blo_cli
```

Oder klone das Repository und baue es lokal:

```bash
git clone https://github.com/Blonar257/blo_cli.git
cd blo_cli
cargo build
```

## Verwendung

### Beispiel

```rust
use blo_cli::colors::Color;
use blo_cli::colors::{fg_colorize, bg_colorize};
use blo_cli::styles::{TextStyle, style, BorderStyle, border};

fn main() {
    // Text in Blau farben
    println!("{}", fg_colorize("Hallo Welt!", Color::Blue));

    // Text fett machen
    println!("{}", style("Hallo Welt!", TextStyle::Bold));

    // Text unterstreichen
    println!("{}", style("Hallo Welt!", TextStyle::Underline));

    // Text mit Angular-Border
    println!("{}", border("Hallo Welt!", BorderStyle::Angular, None));

    // Text mit Rounded-Border in Rot
    println!("{}", border("Hallo Welt!", BorderStyle::Rounded, Some(Color::Red)));
}
```

### Border-Funktionen

Die `border()`-Funktion erstellt einen Rahmen um Text:

```rust
use blo_cli::styles::{BorderStyle, border};
use blo_cli::colors::Color;

// Einfacher Angular-Border (ohne Farbe)
println!("{}", border("Info", BorderStyle::Angular, None));

// Rounded-Border mit Rot
println!("{}", border("Warnung", BorderStyle::Rounded, Some(Color::Red)));

// Multi-line Text mit Border
println!("{}", border("Zeile 1\nZeile 2", BorderStyle::Angular, Some(Color::Green)));
```

**BorderStyle-Optionen:**
- `BorderStyle::Angular`: Eckige Rahmen (┌─┐│└┘)
- `BorderStyle::Rounded`: Gerundete Rahmen (╭─╮│╰╯)

**Farben:** Alle Farben (Red, Green, Blue, Yellow, Pink, Cyan, White, Black) werden unterstützt.

### Vollständiges Demo

Siehe `examples/demo.rs` für mehr Beispiele, einschließlich Hintergrundfarben und Borders.

## Tests

Führe die Tests aus:

```bash
cargo test
```

## Lizenz

Dieses Projekt ist unter der MIT-Lizenz lizenziert.
