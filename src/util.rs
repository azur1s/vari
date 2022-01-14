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