#[derive(Debug)]
pub struct Token {
    pub row: u32,
    pub col: u32,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(row: u32, col: u32, token_type: TokenType) -> Token {
        Token { row, col, token_type }
    }
}

#[derive(Debug)]
pub enum TokenType {
    Plus,
    Minus,
    Int(String),
    EOF,
    ILLEGAL,
}
