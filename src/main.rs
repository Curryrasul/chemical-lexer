mod lexer;

use std::fs;
use lexer::lex;

static PATH: &'static str = "./src/test.txt";

fn main() {
    let my_file = fs::read_to_string(PATH).unwrap();

    let tokens = lex(my_file);

    for i in tokens {
        println!("{:#?}", i);
    }
}
