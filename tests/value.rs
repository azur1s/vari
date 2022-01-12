#![allow(unused_imports)]

use vari::*;

mod test_one {
    
    #[test]
    fn test_split_anchor() {
        let message = "Cool array [$cyan][1, 2, 3][$/]";
        let anchored = vari::split_anchor(message);
        assert_eq!(anchored, vec!["Cool array ", "[$cyan]", "[1, 2, 3]", "[$/]"]);
    }

    #[test]
    fn test_compile_anchor() {
        let message = "[$cyan]This message is cyan[$/]";
        let compiled = vari::compile_anchor(vari::split_anchor(message));
        assert_eq!(compiled, "\x1b[36mThis message is cyan\x1b[0m");
        println!("Original: {}", message);
        println!("Compiled: {}", compiled);
    }

    #[test]
    #[should_panic]
    fn test_compile_anchor_error() {
        let message = "[$woops]An invalid color anchor!";
        vari::compile_anchor(vari::split_anchor(message));
    }

    #[test]
    fn format() {
        let message = vari::format("[$blue]Hello, [$green]World![$/]");
        assert_eq!(message, "\x1b[34mHello, \x1b[32mWorld!\x1b[0m");
        println!("{}", message);
    }
}