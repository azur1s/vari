use regex::Regex;

/// Parse color anchor by detecting if there is
/// a "[" and a color name and a "]" at the end.
/// For example: "[$red]Hello[/]"       will be [ "[$red]", "Hello", "[$/]" ].
///              "[$cyan][1, 2, 3][$/]" will be [ "[$cyan]", "[1, 2, 3]", "[$/]" ].
pub fn split_anchor(message: &str) -> Vec<&str> {
    let regex = Regex::new(r"(?:\[\$(?:\w*|/)\])").expect("Failed to compile regex");

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

/// Compile a color anchor to a ANSI escape sequence.
/// For example: "[$red]Hello[/]" will be "\x1b[31mHello\x1b[0m".
pub fn compile_anchor(messages: Vec<&str>) -> String {
    let mut result = String::new();

    for message in messages {
        match message {
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
                    panic!("Unknown color anchor: {}", message);
                } else {
                    result.push_str(message);
                }
            }
        }
    }

    result
}

pub fn format(message: &str) -> String {
    compile_anchor(split_anchor(message))
}