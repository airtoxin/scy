pub struct Lexer {
    input: String,
    position: u32,
    row: u32,
    col: u32,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer { input: input.chars(), position: 0, row: 0, col: 0 }
    }
}
