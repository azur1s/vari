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

Color anchor are a bbcode-like markup for colors and styles (eg. "[\$red]", "[bg\$yellow]", "[\$bold]")

<details>
<summary>Anchors</summary>
<p>Colors:</p>
<ul>
    <li>[$black]</li>
    <li>[$red]</li>
    <li>[$green]</li>
    <li>[$yellow]</li>
    <li>[$blue]</li>
    <li>[$magenta]</li>
    <li>[$cyan]</li>
    <li>[$white]</li>
    <li>[$reset] or [$r] or [$/]</li>
</ul>
<p>Bright colors:</p>
<ul>
    <li>[$bright_black] or [$brightblack]</li>
    <li>[$bright_red] or [$brightred]</li>
    <li>[$bright_green] or [$brightgreen]</li>
    <li>[$bright_yellow] or [$brightyellow]</li>
    <li>[$bright_blue] or [$brightblue]</li>
    <li>[$bright_magenta] or [$brightmagenta]</li>
    <li>[$bright_cyan] or [$brightcyan]</li>
    <li>[$bright_white] or [$brightwhite]</li>
</ul>
<p>Styles</p>
<ul>
    <li>[$regular]</li>
    <li>[$bold]</li>
    <li>[$dim] or [$low] or [$low_intensity] or [$lowintensity]</li>
    <li>[$italic]</li>
    <li>[$underline]</li>
    <li>[$blink] or [$blinking]</li>
    <li>[$reverse] or [$reversed]</li>
    <li>[$invisible] or [$hidden]</li>
    <li>[$strikethrough] or [$strike_through]</li>
</ul>
<p>Note: [bg$colors] is a valid anchors, it will be translated to [$reversed][$color]</p>
</details>

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
### Colorize
Colorize string directly by calling colorize() method, like [colored](https://github.com/mackwic/colored) crate.

For example: "red".colorize("red") is the same as "[\$red]red[$/]"

Note: Chaining is not yet implemented, because .colorize() adds [$/] so you can't chain styles

The argument should be the color's name (the same name as the anchor colors).
```rust
use vari::colorize::Colorize;

fn main() {
    println!("{}", "Hello, World".colorize("cyan"));
    println!("{}", "This is red".colorize("brightred"));
    println!("{}", "Bold.".colorize("bold"));
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
