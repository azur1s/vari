use crate::util::NoAnsi;

/// Println-ish style function for logging.
/// 
/// This function is similar to `println!` but it will print the message with
/// a message on the right side of the terminal.
/// Note: This function will panic if it can't get the terminal width.
/// 
/// # Example:
/// ```
/// let log_message = vformat!("[$bold][$green]Hello, world![$/]");
/// let log_file = vformat!("[$dim][$cyan]src/something.js[$/]");
/// log(&log_message, &log_file);
/// ```
pub fn log(string: &str, from: &str){
    match from.len() {
        0 => {
            println!("{}", string);
        }
        // If file are specified
        _ => {
            if let Some((w, _)) = term_size::dimensions() {
            
                let padding_amount = w - from.no_ansi().len() - string.no_ansi().len();
                let padding = " ".repeat(padding_amount);
                
                let mut result = String::new();
                
                result.push_str(string);
                result.push_str(&padding);
                result.push_str(from);
                
                println!("{}", result);
            } else {
                panic!("Failed to get terminal size");
            }
        }
    }
}