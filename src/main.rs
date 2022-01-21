mod tokens;
mod lexer;
mod parser;
mod nodes;

use std::io::{Read, Write};
use parser::Parser;
use lexer::Lexer;

fn main() {
    let mut code = String::new();
    let mut file = std::fs::File::open("samples/short.ro").unwrap();
    file.read_to_string(&mut code).unwrap();

    let mut lexer = Lexer::new(code);
    let tokens = lexer.scan();
    file = std::fs::File::create("tokens.txt").unwrap();
    write!(file, "{:#?}", tokens).unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    file = std::fs::File::create("ast.txt").unwrap();
    write!(file, "{:#?}", ast).unwrap();
}
