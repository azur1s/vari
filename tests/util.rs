mod util {
    use vari::util::NoAnsi;

    #[test]
    fn test_hex_to_rgb() {
        let hex = "00ff00";
        let (r, g, b) = vari::util::hex_to_rgb(hex);
        assert_eq!(r, 0);
        assert_eq!(g, 255);
        assert_eq!(b, 0);
    }

    #[test]
    fn no_ansi() {
        let original = "\x1b[1mTest\x1b[0m";
        let result = original.no_ansi();

        assert_eq!(result, "Test");
    }
}