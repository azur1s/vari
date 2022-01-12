# Vari
Vari (Väri) is a Rust library for formatting strings with colors and cosmetic stuff to the terminal. Like [Rich](https://github.com/Textualize/rich) library for Python.

## Installing
```rust
[dependencies]
vari = 0.1.2
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
    println!("{}", vari::format("[$bold][$italic]Bold and Italic :O[$/]"));
}
```

## License
This crate is under [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html) license.

