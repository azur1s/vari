use regex::Regex;

const ANCHOR_REGEX: &str = r"(?:\[\$(?:\w*|\S*)\])";
const RGB_ANCHOR_REGEX: &str = r"(?:\[\$\[(?:\s*?\d{1,3}\s*?,\s*?\d{1,3}\s*?,\s*?\d{1,3}\s*?)\]\])";

/// Parse color anchor by detecting if there is
/// a "\[" and a color name and a "]" at the end.
/// For example: "\[$red]Hello\[/]"       will be \[ "\[$red]", "Hello", "\[$/]" ].
///              "\[$cyan]\[1, 2, 3]\[$/]" will be \[ "\[$cyan]", "\[1, 2, 3]", "\[$/]" ].
pub fn split_anchor(message: &str) -> Vec<&str> {
    let raw_regexs = [ANCHOR_REGEX, "|", RGB_ANCHOR_REGEX];
    let regex = Regex::new(&raw_regexs.join("")).unwrap();

    let mut result = Vec::new();
    let mut last = 0;

    for cap in regex.captures_iter(message) {
        let start = cap.get(0).unwrap().start();
        let end = cap.get(0).unwrap().end();

        // Add the part before the anchor
        if start > last {
            result.push(&message[last..start]);
        }

        // Add the anchor
        result.push(&message[start..end]);

        // Update the last position
        last = end;
    }

    result
}

/// Compile RGB anchor to ANSI escape sequence.
/// Will panic if the values can't be parsed to integer.
/// For example: "\[$\[114, 119, 39]]Custom color!" will be "\x1b[38;2;114;119;39mCustom color!".
pub fn compile_rgb_anchor(anchor: &str) -> String {
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

/// Compile a color anchor to a ANSI escape sequence.
/// For example: "\[$red]Hello\[/]" will be "\x1b[31mHello\x1b[0m".
pub fn compile_anchor(messages: Vec<&str>) -> String {
    let mut result = String::new();

    for message in messages {
        match message.to_lowercase().as_str() {
            "[$red]" => result.push_str("\x1b[31m"),
            "[$green]" => result.push_str("\x1b[32m"),
            "[$yellow]" => result.push_str("\x1b[33m"),
            "[$blue]" => result.push_str("\x1b[34m"),
            "[$magenta]" => result.push_str("\x1b[35m"),
            "[$cyan]" => result.push_str("\x1b[36m"),
            "[$white]" => result.push_str("\x1b[37m"),
            "[$reset]" | "[$/]" => result.push_str("\x1b[0m"),

            _ => {
                if message.starts_with("[$") && message.ends_with("]") {
                    let rgb_anchor_regex = Regex::new(RGB_ANCHOR_REGEX).unwrap();

                    if rgb_anchor_regex.is_match(message) {
                        let compiled = compile_rgb_anchor(message);
                        result.push_str(&compiled);
                    } else {
                        panic!("Unknown color anchor: {}", message);
                    }
                } else {
                    result.push_str(message);
                }
            }
        }
    }

    result
}