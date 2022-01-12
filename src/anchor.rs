use regex::Regex;

const ANCHOR_REGEX: &str = r"(?:\[(?:bg)?\$(?:\w*|\S*)\])";
const RGB_ANCHOR_REGEX: &str = r"(?:\[(?:bg)?\$\[(?:\s*?\d{1,3}\s*?,\s*?\d{1,3}\s*?,\s*?\d{1,3}\s*?)\]\])";
const BG_CHECK: &str = r"\[bg\$";

/// Split a string at the color anchors.
/// 
/// Parse color anchor by detecting if there is
/// a "\[$" and any characters and a "]" at the end.
/// 
/// # Example:
/// ```
/// let f = vari::anchor::split_anchor("[$cyan]Hi![$/]");
/// assert_eq!(f, ["[$cyan]", "Hi!", "[$/]"]);
/// 
/// let fs = vari::anchor::split_anchor("[$red]Vector: [1, 2, 3, 4][$/]");
/// assert_eq!(fs, ["[$red]", "Vector: [1, 2, 3, 4]", "[$/]"]);
/// ```
pub fn split_anchor(message: String) -> Vec<String> {
    let raw_regexs = [ANCHOR_REGEX, "|", RGB_ANCHOR_REGEX];
    let regex = Regex::new(&raw_regexs.join("")).unwrap();

    let mut result: Vec<String> = Vec::new();
    let mut last = 0;

    for cap in regex.captures_iter(&message) {
        let start = cap.get(0).unwrap().start();
        let end = cap.get(0).unwrap().end();

        // Add the part before the anchor
        if start > last {
            result.push(message[last..start].to_string());
        }

        let anchor = &message[start..end];

        let bg_regex = Regex::new(BG_CHECK).unwrap();
        if bg_regex.is_match(&anchor) {
            result.push("[$reverse]".to_string());

            let r = format!("{}", anchor.replace("[bg$", "[$").as_str());

            result.push(r);
        } else {
            result.push(anchor.to_string());
        }

        // Update the last position
        last = end;
    }

    result
}

/// Compile (or replace) the color anchors with ANSI escape sequences.
/// 
/// Compile RGB anchor to ANSI escape sequence.
/// Will panic if the values can't be parsed to integer.
/// 
/// # Example:
/// ```
/// let rgb = vari::anchor::compile_rgb_anchor("[$[255, 255, 255]]");
/// assert_eq!(rgb, "\x1b[38;2;255;255;255m");
/// ```
pub fn compile_rgb_anchor(anchor: String) -> String {
    // Trim out "[$[" and "]]"
    let trimmed = anchor.trim_start_matches("[$[").trim_end_matches("]]");

    let rgb = trimmed.split(",").map(|s| s.trim()).collect::<Vec<&str>>();

    if rgb.len() != 3 {
        panic!("Invalid RGB anchor: {}", anchor);
    }

    let r = rgb[0].parse::<u8>();
    let g = rgb[1].parse::<u8>();
    let b = rgb[2].parse::<u8>();

    if r.is_err() || g.is_err() || b.is_err() {
        panic!("Invalid RGB anchor: {}", anchor);
    } else {
        let r = r.unwrap();
        let g = g.unwrap();
        let b = b.unwrap();

        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }
}

/// Compile (or replace) the color and style anchors with ANSI escape sequences.
/// 
/// Compile a color and style anchor to their respective ANSI escape sequence.
/// If the color is not recognized, it will try to parse it as RGB anchor,
/// and if that fails, it will panic.
/// 
/// # Example:
/// ```
/// let bold = vari::anchor::compile_anchor(["[$bold]", "This is bold!", "[$/]"].to_vec());
/// assert_eq!(bold, "\x1b[1mThis is bold!\x1b[0m");
/// 
/// let chain = vari::anchor::compile_anchor(["[$bold]", "[$red]", "This is bold red!", "[$/]"].to_vec());
/// assert_eq!(chain, "\u{1b}[1m\u{1b}[31mThis is bold red!\u{1b}[0m");
/// 
/// let f7bae0 = vari::anchor::compile_anchor(["[$[247, 186, 224]]", "[$reverse]", "This is f7bae0!", "[$/]"].to_vec());
/// assert_eq!(f7bae0, "\u{1b}[38;2;247;186;224m\u{1b}[7mThis is f7bae0!\u{1b}[0m");
/// ```
pub fn compile_anchor(messages: Vec<String>) -> String {
    let mut result = String::new();

    for message in messages {
        match message.to_lowercase().as_str() {
            // Normal color variants
            "[$black]"   => result.push_str("\x1b[30m"),
            "[$red]"     => result.push_str("\x1b[31m"),
            "[$green]"   => result.push_str("\x1b[32m"),
            "[$yellow]"  => result.push_str("\x1b[33m"),
            "[$blue]"    => result.push_str("\x1b[34m"),
            "[$magenta]" => result.push_str("\x1b[35m"),
            "[$cyan]"    => result.push_str("\x1b[36m"),
            "[$white]"   => result.push_str("\x1b[37m"),
            "[$reset]" | "[$/]" => result.push_str("\x1b[0m"),

            // Bright color variants
            "[$bright_black]"   | "[$brightblack]"   => result.push_str("\x1b[90m"),
            "[$bright_red]"     | "[$brightred]"     => result.push_str("\x1b[91m"),
            "[$bright_green]"   | "[$brightgreen]"   => result.push_str("\x1b[92m"),
            "[$bright_yellow]"  | "[$brightyellow]"  => result.push_str("\x1b[93m"),
            "[$bright_blue]"    | "[$brightblue]"    => result.push_str("\x1b[94m"),
            "[$bright_magenta]" | "[$brightmagenta]" => result.push_str("\x1b[95m"),
            "[$bright_cyan]"    | "[$brightcyan]"    => result.push_str("\x1b[96m"),
            "[$bright_white]"   | "[$brightwhite]"   => result.push_str("\x1b[97m"),

            // Style variants
            "[$regular]" => result.push_str("\x1b[0m"),
            "[$bold]" => result.push_str("\x1b[1m"),
            "[$low]" | "[$low_intensity]" | "[$lowintensity]" | "$[dim]" => result.push_str("\x1b[2m"),
            "[$italic]" => result.push_str("\x1b[3m"),
            "[$underline]" => result.push_str("\x1b[4m"),
            "[$blink]" | "[$blinking]" => result.push_str("\x1b[5m"),
            "[$reverse]" | "[$reversed]" => result.push_str("\x1b[7m"),
            "[$invisible]" => result.push_str("\x1b[8m"),
            "[$strikethrough]" | "[$strike_through]" => result.push_str("\x1b[9m"),

            _ => {
                if message.starts_with("[$") && message.ends_with("]") {
                    let rgb_anchor_regex = Regex::new(RGB_ANCHOR_REGEX).unwrap();

                    if rgb_anchor_regex.is_match(&message) {
                        let compiled = compile_rgb_anchor(message.to_string());
                        result.push_str(&compiled);
                    } else {
                        panic!("Unknown color anchor: {}", message);
                    }
                } else {
                    result.push_str(&message);
                }
            }
        }
    }

    result
}