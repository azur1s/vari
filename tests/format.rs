mod format {
    #[test]
    fn rainbow_format() {
        let rainbow = vari::fun::rainbow("Rainbow!!");
        println!("{}", rainbow);
    }

    #[test]
    fn test_bg() {
        let bg = vari::anchor::split_anchor("[bg$red]Test[$/]".to_string());
        println!("{:?}", bg);
    }

    #[test]
    fn test_hex() {
        let f = vari::vformat!("[$#ffffff]Test[$/]");
        println!("{}", f);
    }

    #[test]
    fn vprintln_no_args() {
        vari::vprintln!("[$cyan]Hi![$/]");
    }

    #[test]
    fn format() {
        let hello = vari::format("[$blue]Hello, [$green]World![$/]");
        assert_eq!(hello, "\x1b[34mHello, \x1b[32mWorld!\x1b[0m");
        println!("{}", hello);

        let custom_rgb = vari::format("[$[114, 119, 39]]#727727![$[66, 4, 32]] Do you see it?[$/]");
        assert_eq!(custom_rgb, "\x1b[38;2;114;119;39m#727727!\x1b[38;2;66;4;32m Do you see it?\x1b[0m");
        println!("{}", custom_rgb);

        let styled = vari::format("[$brightmagenta][$underline]Underline!![$/]");
        assert_eq!(styled, "\x1b[95m\x1b[4mUnderline!!\x1b[0m");
        println!("{}", styled);

        let reversed = vari::format("[$reverse][$bright_red]Reverseee!!![$/]");
        assert_eq!(reversed, "\x1b[7m\x1b[91mReverseee!!!\x1b[0m");
        println!("{}", reversed);

        let chained = vari::format("[$strikethrough][$reverse][$bold][$italic][$bright_green][$underline]😎 Big chains 😎[$/]");
        assert_eq!(chained, "\x1b[9m\x1b[7m\x1b[1m\x1b[3m\x1b[92m\x1b[4m😎 Big chains 😎\x1b[0m");
        println!("{}", chained);

        let background = vari::format("[bg$red]Hello,[bg$white] [bg$[188, 188, 188]]World![$/]");
        assert_eq!(background, "\u{1b}[7m\u{1b}[31mHello,\u{1b}[7m\u{1b}[37m \u{1b}[7m\u{1b}[38;2;188;188;188mWorld!\u{1b}[0m");
        println!("{}", background);

        let hex = vari::format("[$#ffffff]Hello,[$#000000] World![$/]");
        assert_eq!(hex, "\x1b[38;2;255;255;255mHello,\x1b[38;2;0;0;0m World!\x1b[0m");
        println!("{}", hex);
    }
}