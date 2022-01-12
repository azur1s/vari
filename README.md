# Vari
Vari (Väri) is a Rust library for formatting strings with colors and cosmetic stuff to the terminal. Like [Rich](https://github.com/Textualize/rich) library for Python.

## Installing
```rs
[dependencies]
vari = 0.1
```

## Features
### Color Anchor
```rs
use vari::format;

fn main() {
    // [$/] is shorthand for [$reset]
    let message = vari::format("[$blue]Hello, [$green]World![$/]");
    println!("{}", message);
}
```

## License
This crate is under [AGPL-3.0](https://www.gnu.org/licenses/agpl-3.0.en.html) license.

