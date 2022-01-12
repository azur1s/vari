#![allow(unused_imports)]

mod macros {
    
    use vari::vformat;

    #[test]
    fn format() {
        let text = vformat!("[$blue]Hello, [$green]World![$/]");
        assert_eq!(text, "\x1b[34mHello, \x1b[32mWorld!\x1b[0m");
        println!("{}", text);
    }
}