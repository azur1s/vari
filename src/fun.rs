const RAINBOW_ESCAPE_SEQUENCE: [&str; 6] = ["\x1b[31m", "\x1b[33m", "\x1b[32m", "\x1b[36m", "\x1b[34m", "\x1b[35m"];

/// Colorize a string into a rainbow-colored string.
/// 
/// This function split a string into a vector of chars, and then colorize each char
/// to the index of the rainbow color. If the char index is out of rainbow color range,
/// it will simply wrap it back to the beginning.
/// 
/// # Examples
/// ```
/// use vari::fun::rainbow;
/// 
/// let rainbow_string = rainbow("Rainbow!!");
/// assert_eq!(rainbow_string, "\u{1b}[31mR\u{1b}[33ma\u{1b}[32mi\u{1b}[36mn\u{1b}[34mb\u{1b}[35mo\u{1b}[31mw\u{1b}[33m!\u{1b}[32m!\u{1b}[0m");
/// // that is pretty long assertion...
/// ```
pub fn rainbow(message: &str) -> String {
    let mut result = String::new();

    // Iterate through each character
    for (i, c) in message.chars().enumerate() {
        // Get the color index
        let index = i % RAINBOW_ESCAPE_SEQUENCE.len();

        // Add the color escape sequence
        result.push_str(&RAINBOW_ESCAPE_SEQUENCE[index]);

        // Add the character
        result.push(c);
    }
    result.push_str("\x1b[0m");

    result
}