use crate::token::{Literal, Token, TokenType};

pub fn lex(source: &str) -> Vec<Token> {
    Lexer::new(source.chars().collect()).lex()
}

struct Lexer {
    chars: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Lexer {
    fn new(chars: Vec<char>) -> Self {
        Self {
            chars,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn lex(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens.push(self.token(TokenType::Eof, None, None));

        self.tokens
    }

    fn scan_token(&self) {}

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn token(&self, kind: TokenType, lexeme: Option<String>, literal: Option<Literal>) -> Token {
        Token {
            kind,
            lexeme,
            literal,
            line: self.line,
        }
    }
}
