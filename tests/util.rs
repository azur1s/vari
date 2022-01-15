mod util {
    use vari::{util::NoAnsi, vformat};

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
        let from = vformat!("[$dim]src/main.rs:1[$/]");
        println!("{}", vari::util::log_from(&vformat!("[$green]GET[$/] src/something.js"), &from));
        println!("{}", vari::util::log_from(&vformat!("[$green]GET[$/] src/assets/cool_pictures.png"), &from));
        println!("{}", vari::util::log_from(&vformat!("[$green]GET[$/] src/assets/wallpaper.png"), &from));
        println!("{}", vari::util::log_from(&vformat!("[$green]GET[$/] src/css.css"), &from));
        println!("{}", vari::util::log_from(&vformat!("[$green]GET[$/] src/index.js"), &from));
        println!("{}", vari::util::log_from(&vformat!("[$green]GET[$/] src/favicon.png"), &from));
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