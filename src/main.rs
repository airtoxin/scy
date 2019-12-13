use crate::lexer::Lexer;
use crate::token::TokenType;

mod lexer;
mod token;

fn main() -> () {
    let input = "27529 + 97248".to_string();
    let mut lexer = Lexer::new(&input);

    let mut token = lexer.read();
    while token.token_type != TokenType::EOF {
        println!("{:?}", token);
        token = lexer.read();
    }
}
