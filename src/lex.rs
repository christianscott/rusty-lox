use crate::token::{Literal, Range, Token, TokenKind};

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
        self.add_basic_token(TokenKind::Eof);

        self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_basic_token(TokenKind::LeftParen),
            ')' => self.add_basic_token(TokenKind::RightParen),
            '{' => self.add_basic_token(TokenKind::LeftBrace),
            '}' => self.add_basic_token(TokenKind::RightBrace),
            ',' => self.add_basic_token(TokenKind::Comma),
            '.' => self.add_basic_token(TokenKind::Dot),
            '-' => self.add_basic_token(TokenKind::Minus),
            '+' => self.add_basic_token(TokenKind::Plus),
            ';' => self.add_basic_token(TokenKind::Semicolon),
            '*' => self.add_basic_token(TokenKind::Star),
            '!' => {
                if self.eat('=') {
                    self.add_basic_token(TokenKind::BangEqual);
                } else {
                    self.add_basic_token(TokenKind::Bang);
                }
            }
            '=' => {
                if self.eat('=') {
                    self.add_basic_token(TokenKind::EqualEqual);
                } else {
                    self.add_basic_token(TokenKind::Equal);
                }
            }
            '>' => {
                if self.eat('=') {
                    self.add_basic_token(TokenKind::GreaterEqual);
                } else {
                    self.add_basic_token(TokenKind::Greater);
                }
            }
            '<' => {
                if self.eat('=') {
                    self.add_basic_token(TokenKind::LessEqual);
                } else {
                    self.add_basic_token(TokenKind::Less);
                }
            }
            '/' => {
                if self.eat('/') {
                    self.eat_while(|&c| c != '\n');
                } else {
                    self.add_basic_token(TokenKind::Slash);
                }
            }
            ' ' | '\t' | '\r' => {} // skip whitespace
            '\n' => {
                self.line += 1;
            }
            '"' => self.string(),
            _ => {
                if c.is_numeric() {
                    self.number();
                } else if c.is_alphabetic() {
                    self.identifier();
                } else {
                    // TODO: error out here
                }
            }
        }
    }

    fn string(&mut self) {
        self.eat_while(|&ch| ch != '"');
        if self.is_at_end() {
            // TODO: error out here
            return;
        }
        self.advance();
        let text = self.get_lexeme();
        self.add_token(self.token(TokenKind::String, Some(Literal::Str(text))));
    }

    fn number(&mut self) {
        self.eat_while(|&c| c.is_numeric());
        let literal = if self.next_is('.') && self.peek_nth(1).map_or(false, |ch| ch.is_numeric()) {
            println!("branch with dot");
            self.advance();
            self.eat_while(|ch| ch.is_numeric());
            self.get_lexeme().parse().expect("TODO: real error handling")
        } else {
            println!("branch without dot");
            self.get_lexeme().parse::<u64>().expect("TODO: real error handling") as f64
        };
        self.add_token(self.token(TokenKind::Number, Some(Literal::Number(literal))))
    }

    fn identifier(&mut self) {
        self.eat_while(|c| c.is_alphanumeric());
        self.add_token(self.token(TokenKind::Identifier, None));
    }

    fn get_lexeme(&self) -> String {
        self.chars[self.start..self.current].iter().clone().collect()
    }

    fn eat(&mut self, c: char) -> bool {
        if self.next_is(c) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn eat_while(&mut self, predicate: fn(&char) -> bool) {
        while self.peek().map_or(false, predicate) {
            self.advance();
        }
    }

    fn next_is(&self, c: char) -> bool {
        self.peek().map_or(false, |&ch| ch == c)
    }

    fn advance(&mut self) -> &char {
        self.current += 1;
        unsafe { self.chars.get_unchecked(self.current - 1) }
    }

    fn peek(&self) -> Option<&char> {
        self.peek_nth(0)
    }

    fn peek_nth(&self, n: usize) -> Option<&char> {
        self.chars.get(self.current + n)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    fn add_basic_token(&mut self, kind: TokenKind) {
        self.add_token(self.token(kind, None));
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn token(&self, kind: TokenKind, literal: Option<Literal>) -> Token {
        Token {
            kind,
            lexeme: Range(self.start, self.current),
            literal,
            line: self.line,
        }
    }
}
