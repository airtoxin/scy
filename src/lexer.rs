use std::str::Chars;

use crate::token::{Token, TokenType};
use std::iter::Peekable;

pub struct Lexer<'a> {
    input: &'a String,
    position: u32,
    row: u32,
    col: u32,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a String) -> Lexer<'a> {
        Lexer {
            input,
            position: 0,
            row: 0,
            col: 0,
        }
    }

    pub fn read(&mut self) -> Token {
        let mut chars = self.input[self.position as usize..].chars().peekable();
        match chars.peek() {
            None => {
                chars.next();
                Token::new(self.row, self.col, TokenType::EOF)
            }
            Some(ch) => match ch {
                ' ' | '\t' => {
                    self.col += 1;
                    self.position += 1;
                    chars.next();
                    self.read()
                }
                '\n' | '\r' => {
                    self.col = 0;
                    self.row += 1;
                    self.position += 1;
                    chars.next();
                    self.read()
                }
                '+' => {
                    self.col += 1;
                    self.position += 1;
                    chars.next();
                    Token::new(self.row, self.col, TokenType::Plus)
                }
                '-' => {
                    self.col += 1;
                    self.position += 1;
                    chars.next();
                    Token::new(self.row, self.col, TokenType::Minus)
                }
                _ => {
                    if ch.is_digit(10) {
                        Token::new(self.row, self.col, TokenType::Int(self.read_number(chars)))
                    } else {
                        self.col += 1;
                        self.position += 1;
                        chars.next();
                        Token::new(self.row, self.col, TokenType::ILLEGAL)
                    }
                }
            },
        }
    }

    fn read_number(&mut self, mut chars: Peekable<Chars<'a>>) -> String {
        let mut diff: usize = 0;

        while let Some(ch) = chars.peek() {
            if !ch.is_digit(10) {
                break;
            }
            chars.next();
            diff += 1;
        }

        let num =
            self.input.as_str()[(self.position as usize)..(self.position as usize + diff)].into();

        self.position += diff as u32;
        self.col += diff as u32;

        num
    }
}

#[test]
fn lexer_read() {
    let input = "27529 + 97248".to_string();
    let mut lexer = Lexer::new(&input);
    assert_eq!(
        lexer.read(),
        Token::new(0, 0, TokenType::Int("27529".to_string()))
    );
    assert_eq!(lexer.read(), Token::new(0, 6, TokenType::Plus));
    assert_eq!(
        lexer.read(),
        Token::new(0, 8, TokenType::Int("97248".to_string()))
    );
}
