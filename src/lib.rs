pub mod anchor;
pub mod fun;

/// Parse color anchor by detecting if there is a color anchor
/// and then replace it with ANSI escape sequence.
/// If there is a invalid color anchor (eg. "[$not_a_color]"), it will panic.
/// Example: "[$cyan]Hi![/]" will be "\x1b[36mHi!\x1b[0m".
pub fn format(message: &str) -> String {
    anchor::compile_anchor(anchor::split_anchor(message))
}

/// Like vari::format() but condensed into a macro for convenience.
#[macro_export]
macro_rules! vformat {
    ($($arg:tt)*) => {{
        let formatted = std::fmt::format(std::format_args!($($arg)*));
        let result = vari::format(formatted.as_str());
        result
    }}
}

/// vformat!() macro but also print the result to stdout.
#[macro_export]
macro_rules! vprint {
    ($($arg:tt)*) => {{
        let formatted = std::fmt::format(std::format_args!($($arg)*));
        let result = vari::format(formatted.as_str());
        print!("{}", result);
    }}
}

/// vprint!() but with a newlines.
#[macro_export]
macro_rules! vprintln {
    ($($arg:tt)*) => {{
        let formatted = std::fmt::format(std::format_args!($($arg)*));
        let result = vari::format(formatted.as_str());
        println!("{}", result);
    }}
}