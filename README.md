![Logo](https://raw.githubusercontent.com/azur1s/vari/master/assets/vari_logo.png)
# Vari
[![crates.io](https://img.shields.io/crates/v/vari.svg)](https://crates.io/crates/vari)
[![crates.io](https://docs.rs/chumsky/badge.svg)](https://docs.rs/vari/)
[![crates.io](https://img.shields.io/crates/dr/vari)](https://crates.io/crates/vari)
[![License](https://img.shields.io/crates/l/vari.svg)](https://github.com/azur1s/vari#license)

Vari (Väri) is a Rust library for formatting strings with colors and cosmetic stuff to the terminal. Like [Rich](https://github.com/Textualize/rich) library for Python.

## Installing
```toml
[dependencies]
vari = "0.1.5"
```

## Features

### Color Anchor

Color anchor are a token that is inside a string with "[\$...]" format (eg. "[\$red]", "[bg\$yellow]", "[\$bold]")

```rust
// [$/] is shorthand for [$reset]
let message = vari::format("[$blue]Hello, [$green]World![$/]");
println!("{}", message);

// Custom RGB!
println!("{}", vari::format("[$[114, 119, 39]]#727727![$[66, 4, 32]] Do you see it?[$/]"));

// Style anchor and also easy macros :O
vprintln!("{}Bold and Italic :O{}", "[$bold][$italic]", "[$/]");

// Background color
vprintln!("{}Backgroundssss{}[$/]", "[bg$magenta]", "[bg$[188, 188, 188]]World![$/]")
```
### Fun
```rust
fn main() {
    // Rainbow colors!
    println!("{}", vari::fun::rainbow("Rainbow!!!"));
}
```
## License
This crate is under [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html) license.
