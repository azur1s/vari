mod colorize {
    use vari::colorize::Colorize;

    #[test]
    #[should_panic]
    fn invalid_colorize() {
        let unknown = "unknown".colorize("???");
        println!("{}", unknown);
    }

    #[test]
    fn colorize() {
        let red = "red".colorize("red");
        assert_eq!(red, "\x1b[31mred\x1b[0m");
        println!("{}", red);

        let hello = format!("{}{}", "hello".colorize("green"), "world".colorize("magenta"));
        assert_eq!(hello, "\x1b[32mhello\x1b[0m\x1b[35mworld\x1b[0m");
        println!("{}", hello);

        println!("{}", "Bold.".colorize("bold"));
    }
}