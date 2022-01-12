const RAINBOW_ESCAPE_SEQUENCE: [&str; 6] = ["\x1b[31m", "\x1b[33m", "\x1b[32m", "\x1b[36m", "\x1b[34m", "\x1b[35m"];

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