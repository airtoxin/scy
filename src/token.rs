#[derive(Debug, PartialEq, Eq)]
pub struct Token {
    pub row: u32,
    pub col: u32,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(row: u32, col: u32, token_type: TokenType) -> Token {
        Token {
            row,
            col,
            token_type,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenType {
    Plus,      // +
    Minus,     // -
    Star,      // *
    Slash,     // /
    SemiColon, // ;
    Int(String),
    Float(String),
    EOF,
    ILLEGAL,
}
