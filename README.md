# Vari
[![crates.io](https://img.shields.io/crates/v/vari.svg)](https://crates.io/crates/vari)
[![crates.io](https://docs.rs/chumsky/badge.svg)](https://docs.rs/vari/)
[![License](https://img.shields.io/crates/l/vari.svg)](https://github.com/azur1s/vari#license)

Vari (Väri) is a Rust library for formatting strings with colors and cosmetic stuff to the terminal. Like [Rich](https://github.com/Textualize/rich) library for Python.

## Installing
```toml
[dependencies]
vari = 0.1.3
```

## Features
### Color Anchor
```rust
fn main() {
    // [$/] is shorthand for [$reset]
    let message = vari::format("[$blue]Hello, [$green]World![$/]");
    println!("{}", message);

    // Custom RGB!
    println!("{}", vari::format("[$[114, 119, 39]]#727727![$[66, 4, 32]] Do you see it?[$/]"));

    // Style anchor!
    vprintln!("{}Bold and Italic :O{}", "[$bold][$italic]", "[$/]");
}
```
### Macros
```rust
fn main() {
    // We are using `vformat!()` because `format!()` is from Rust and we can't replace it
    let f = vformat!("{}Hello!{}", "[$bright_magenta]", "[$/]");
    println!("{}", f);

    // Feeling lazy? there is vprint! and vprintln! macros!
    vprintln!("{}I'm feeling {}{}", "[$italic][$yellow]", "lazy" "$[/]");
}
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
