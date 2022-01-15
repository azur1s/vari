#![doc = include_str!("../README.md")]
#![doc(html_logo_url = "https://raw.githubusercontent.com/azur1s/vari/master/assets/vari_logo.png")]

pub mod anchor;
pub mod fun;
pub mod colorize;
pub mod util;

#[cfg(feature = "log")]
pub mod log;

/// Format a string with ANSI escape sequences.
///
/// Parse color anchor by detecting if there is a color anchor
/// and then replace it with ANSI escape sequence.
/// If there is a invalid color anchor (eg. "\[$not_a_color]"), it will panic.
/// 
/// # Example:
/// ```
/// let f = vari::format("[$cyan]Hi![$/]");
/// assert_eq!(f, "\x1b[36mHi!\x1b[0m");
/// ```
pub fn format(message: &str) -> String {
    anchor::compile_anchor(anchor::split_anchor(message.to_string()))
}

/// Like vari::format() but condensed into a macro for convenience, and can also format strings.
/// 
/// Format a strings like Rust's format!() macro, and then run it through
/// vari::format() function.
///
/// # Example:
/// ```
/// let f = vformat!("[$cyan]Hi![$/]");
/// assert_eq!(f, "\x1b[36mHi!\x1b[0m");
/// ```
#[macro_export]
macro_rules! vformat {
    ($($arg:tt)*) => {{
        let formatted = std::fmt::format(std::format_args!($($arg)*));
        let result = vari::format(formatted.as_str());
        result
    }}
}

/// vformat!() macro but also print the result to stdout.
/// 
/// A macro for formatting and printting string with vari::format() function.
/// Kind of like Rust's print!() macro but with colored strings.
/// 
/// # Example:
/// ```
/// vprintln!("[$cyan]Hi![$/]");
/// ```
#[macro_export]
macro_rules! vprint {
    ($($arg:tt)*) => {{
        let formatted = std::fmt::format(std::format_args!($($arg)*));
        let result = vari::format(formatted.as_str());
        print!("{}", result);
    }}
}

/// vprint!() but with a newlines. If no arguments are given, it will print a newline.
/// 
/// A macro for formatting and printing string with vari::format() function.
/// It's just vari::vprint!() but with a newline at the end. For convenience.
/// 
/// # Example:
/// ```
/// vprintln!("[$cyan]Hi![$/]");
/// ```
#[macro_export]
macro_rules! vprintln {
    () => {{
        println();
    }};
    ($($arg:tt)*) => {{
        let formatted = std::fmt::format(std::format_args!($($arg)*));
        let result = vari::format(formatted.as_str());
        println!("{}", result);
    }}
}

/// Expands to the file, line and column on where it was called.
/// 
/// Return the file, line and column on where it was called with dimmed style.
/// Used for debugging (with vari::util::log()).
/// 
/// # Example:
/// ```
/// log!("Debugging!", here!());
/// ```
#[macro_export]
macro_rules! here {
    () => {{
        &vformat!("[$dim]{}:{}:{}[$/]", file!(), line!(), column!())
    }}
}