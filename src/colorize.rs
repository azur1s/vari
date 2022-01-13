/// Colorize trait for .colorize() method.
/// 
/// This trait is used to colorize strings by doing .colorize("...") on them.
/// This might be more convenient for some people who doesn't like vari::format()
/// or color-anchors family macros.
/// 
/// # Example:
/// ```
/// use vari::colorize::Colorize;
/// 
/// fn main() {
///     let red = "red".colorize("red");
///     assert_eq!(red, "\x1b[31mred\x1b[0m");
///     println!("{}", red);
/// }
/// ```
pub trait Colorize {
    fn colorize(&self, color: &str) -> String;
    fn from_string(&self, color: &str) -> Colors;
}

/// An enum representing a color.
/// 
/// An enum representing a color, this is only used in the Colorize trait
/// and is not intended to be used directly.
pub enum Colors {
    Reset,
    Black, Red, Green, Yellow, Blue, Magenta, Cyan, White,
    BrightBlack, BrightRed, BrightGreen, BrightYellow, BrightBlue, BrightMagenta, BrightCyan, BrightWhite,
    Regular, Bold, Dim, Italic, Underline, Blink, Reverse, Hidden, Strikethrough,
}

/// Colorize trait implementation for string.
impl Colorize for str {
    /// Colorize a string with ANSI escape sequences.
    /// 
    /// Take a string with color names and replace them with ANSI escape sequences.
    /// If there is a invalid color name (eg. .colorize("not_a_color")), it will panic.
    fn colorize(&self, color: &str) -> String {
        let mut result = String::new();
        
        result.push_str(&self.from_string(color).to_string());
        result.push_str(self);
        result.push_str(&Colors::Reset.to_string());

        result
    }

    /// Convert a string to a Colors enum.
    /// 
    /// Convert a string to a Colors enum, this is only used in the Colorize trait.
    fn from_string(&self, color: &str) -> Colors {
        match color.to_lowercase().as_str() {
            "reset" | "/" | "r" => Colors::Reset,
            "black" => Colors::Black,
            "red" => Colors::Red,
            "green" => Colors::Green,
            "yellow" => Colors::Yellow,
            "blue" => Colors::Blue,
            "magenta" => Colors::Magenta,
            "cyan" => Colors::Cyan,
            "white" => Colors::White,
            "brightblack" | "bright_black" => Colors::BrightBlack,
            "brightred" | "bright_red" => Colors::BrightRed,
            "brightgreen" | "bright_green" => Colors::BrightGreen,
            "brightyellow" | "bright_yellow" => Colors::BrightYellow,
            "brightblue" | "bright_blue" => Colors::BrightBlue,
            "brightmagenta" | "bright_magenta" => Colors::BrightMagenta,
            "brightcyan" | "bright_cyan" => Colors::BrightCyan,
            "brightwhite" | "bright_white" => Colors::BrightWhite,
            "regular" => Colors::Regular,
            "bold" => Colors::Bold,
            "dim" | "low" | "low_intensity" | "lowintensity" => Colors::Dim,
            "italic" => Colors::Italic,
            "underline" => Colors::Underline,
            "blink" => Colors::Blink,
            "reverse" | "reversed" => Colors::Reverse,
            "hidden" | "invisible" => Colors::Hidden,
            "strikethrough" | "strike_through" => Colors::Strikethrough,
            _ => panic!("Unknown color: {}", color),
        }
    }
}

/// Colorize trait implementation.
impl Colors {
    /// Convert a Colors enum to a string.
    /// 
    /// Convert a Colors enum to a string, this is only used in the colorize() function.
    /// You might be able to use this directly, but not recommended.
    fn to_string(&self) -> String {
        match self {
            Colors::Reset => "\x1b[0m",
            Colors::Black => "\x1b[30m",
            Colors::Red => "\x1b[31m",
            Colors::Green => "\x1b[32m",
            Colors::Yellow => "\x1b[33m",
            Colors::Blue => "\x1b[34m",
            Colors::Magenta => "\x1b[35m",
            Colors::Cyan => "\x1b[36m",
            Colors::White => "\x1b[37m",
            Colors::BrightBlack => "\x1b[90m",
            Colors::BrightRed => "\x1b[91m",
            Colors::BrightGreen => "\x1b[92m",
            Colors::BrightYellow => "\x1b[93m",
            Colors::BrightBlue => "\x1b[94m",
            Colors::BrightMagenta => "\x1b[95m",
            Colors::BrightCyan => "\x1b[96m",
            Colors::BrightWhite => "\x1b[97m",
            Colors::Regular => "\x1b[39m",
            Colors::Bold => "\x1b[1m",
            Colors::Dim => "\x1b[2m",
            Colors::Italic => "\x1b[3m",
            Colors::Underline => "\x1b[4m",
            Colors::Blink => "\x1b[5m",
            Colors::Reverse => "\x1b[7m",
            Colors::Hidden => "\x1b[8m",
            Colors::Strikethrough => "\x1b[9m",
        }.to_string()
    }
}