pub mod anchor;
pub mod fun;

/// Parse color anchor by detecting if there is a color anchor
/// and then replace it with ANSI escape sequence.
/// If there is a invalid color anchor (eg. "[$not_a_color]"), it will panic.
/// Example: "[$cyan]Hi![/]" will be "\x1b[36mHi!\x1b[0m".
pub fn format(message: &str) -> String {
    anchor::compile_anchor(anchor::split_anchor(message))
}

#[macro_export]
macro_rules! vformat {
    ($message:expr) => {
        $crate::format($message)
    };
}