mod util {
    use vari::{util::NoAnsi, vformat, here};

    #[test]
    fn test_hex_to_rgb() {
        let hex = "00ff00";
        let (r, g, b) = vari::util::hex_to_rgb(hex);
        assert_eq!(r, 0);
        assert_eq!(g, 255);
        assert_eq!(b, 0);
    }

    #[test]
    fn from() {
        let text = vformat!("[$cyan]Test[$/]");
        let from = vformat!("[$dim]src/main.rs:1[$/]");

        // Terminal width is 80, for example
        let width = term_size::dimensions().unwrap().0;

        println!("\n---------- Normal .len() padding calculation ----------");
        println!("{}", vformat!("[$dim]\" \".repeat(width - from.len() - text.len()[$/]"));
        println!("{}{}{}", text, " ".repeat(width - from.len() - text.len()), from);
        println!("--------- .no_ansi().len() padding calculation --------");
        println!("{}", vformat!("[$dim]\" \".repeat(width - from.no_ansi().len() - text.no_ansi().len()[$/]"));
        println!("{}{}{}", text, " ".repeat(width - from.no_ansi().len() - text.no_ansi().len()), from);
        println!("-------------------------------------------------------");
        println!("Text: {}, From: {}\n", text, from);
    }

    #[test]
    fn readme_from() {
        let log_message = vformat!("[$green]This message is send by main.rs![$/]");
        let log_file = vformat!("[$dim]src/main.rs[$/]");
        vari::util::log(&log_message, &log_file);
        vari::util::log(&log_message, here!());
    }

    #[test]
    fn no_ansi() {
        let a = "\x1b[1mAAAA\x1b[0m";
        let b = "BBBB";
        let c = "\x1b[1mCCCC";
        let d = "DDDD\x1b[0m";
        let ar = a.no_ansi();
        let br = b.no_ansi();
        let cr = c.no_ansi();
        let dr = d.no_ansi();

        assert_eq!(ar, "AAAA");
        assert_eq!(br, "BBBB");
        assert_eq!(cr, "CCCC");
        assert_eq!(dr, "DDDD");
    }
}