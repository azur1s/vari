#![allow(unused_imports)]

mod macros {

    use vari::vformat;

    #[test]
    fn vformat_like_format() {
        let formatted = vformat!("[$italic]{}{}[$bold]{}{}{}", "[$blue]", "Hi! ", "[$red]", "Bye!", "[$/]");
        assert_eq!(formatted, "\u{1b}[3m\u{1b}[34mHi! \u{1b}[1m\u{1b}[31mBye!\u{1b}[0m");
        println!("{}", formatted);
    }

    #[test]
    fn format() {
        let text = vformat!("[$blue]Hello, [$green]World![$/]");
        assert_eq!(text, "\x1b[34mHello, \x1b[32mWorld!\x1b[0m");
        println!("{}", text);
    }
}