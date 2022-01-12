mod compile {
    #[test]
    fn split_anchor() {
        let message = vari::anchor::split_anchor("Cool array [$cyan][1, 2, 3][$/]");
        let custom_rgb = vari::anchor::split_anchor("[$[114, 119, 39]]#727727![$[66, 4, 32]]Do you see it?[$/]");
        assert_eq!(message, vec!["Cool array ", "[$cyan]", "[1, 2, 3]", "[$/]"]);
        assert_eq!(custom_rgb, vec!["[$[114, 119, 39]]", "#727727!", "[$[66, 4, 32]]", "Do you see it?", "[$/]"]);
    }

    #[test]
    fn compile_anchor() {
        let message = "[$cyan]This message is cyan[$/]";
        let compiled = vari::anchor::compile_anchor(vari::anchor::split_anchor(message));
        assert_eq!(compiled, "\x1b[36mThis message is cyan\x1b[0m");
    }

    #[test]
    #[should_panic]
    fn compile_anchor_error() {
        vari::format("[$woops]An invalid color anchor!");
    }

    #[test]
    #[should_panic]
    fn compile_rgb_anchor_error() {
        vari::format("[$[str,ing,???]]Invalid color anchor :(");
    }

    #[test]
    #[should_panic]
    fn compile_rgb_overflow_error() {
        vari::format("[$[256, 256, 256]]Overflow!!!");
    }
}