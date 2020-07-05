use crate::token::*;

fn lex(source: &str) -> Vec<Token> {
    Lexer { chars: source.chars().collect() }.lex()
}

struct Lexer {
    chars: Vec<char>,
}

impl Lexer {
    fn lex(&mut self) -> Vec<Token> {
        Vec::new()
    }
}