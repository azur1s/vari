use regex::Regex;

/// Converts a hex string to a RGB color.
/// 
/// Takes a hex string and returns a RGB color. Will panic if the string is not
/// a valid hex string.
/// This is only intended to use inside vari. But you can use it in your own code.
/// 
/// # Example:
/// ```
/// let hex = "[$#00ff00]";
/// let rgb = vari::util::hex_to_rgb(hex);
/// assert_eq!(rgb, (0, 255, 0));
/// ```
pub fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
    let f = hex.trim_start_matches("[$#").trim_end_matches("]").to_string();

    let r = u8::from_str_radix(&f[0..2], 16);
    let g = u8::from_str_radix(&f[2..4], 16);
    let b = u8::from_str_radix(&f[4..6], 16);

    // Check if the parsed value is unsuccessful.
    if r.is_err() || g.is_err() || b.is_err() {
        panic!("Invalid color hex string: {}", hex);
    }

    (r.unwrap(), g.unwrap(), b.unwrap())
}

/// A trait for removing ANSI escape sequence.
pub trait NoAnsi {
    fn no_ansi(&self) -> String;
}

/// Remove ANSI escape sequence from a string.
impl NoAnsi for str {
    /// Remove ANSI escape sequence from a string.
    /// 
    /// Remove ANSI escape sequence from a string by doing a regex match and then
    /// push the string without the ANSI escape sequence. This could be use in
    /// padding calculation because `.len()` only ignores `\x1b` and not `[..m`.
    /// 
    /// # Example:
    /// ```
    /// let original = "\x1b[1mTest\x1b[0m";
    /// let result = original.no_ansi();
    /// assert_eq!(result, "Test");
    /// ```
    fn no_ansi(&self) -> String {
        let mut result = String::new();

        let re = Regex::new(r"(?:\x1b\[[;\d]*m)").unwrap();
        let mut last = 0;

        if re.is_match(self) {
            for cap in re.captures_iter(self) {
                let start = cap.get(0).unwrap().start();
                let end = cap.get(0).unwrap().end();

                result.push_str(&self[last..start]);

                last = end;
            }
        } else {
            result.push_str(self);
        }

        result
    }
}
