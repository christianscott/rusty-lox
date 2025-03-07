use crate::token::{Range, Source, Token, TokenKind};
use std::rc::Rc;

pub fn lex(name: String, source: &str) -> Vec<Token> {
    Lexer::new(name, source.chars().collect()).lex()
}

struct Lexer {
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    source: Rc<Source>,
}

impl Lexer {
    fn new(name: String, chars: Vec<char>) -> Self {
        let source = Source::new(name, chars);
        Self {
            source: Rc::new(source),
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
                    panic!("unexpected char {}", c)
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
        let text = self.get_lexeme(&Range(self.start + 1, self.current - 1));
        self.add_basic_token(TokenKind::Str(text))
    }

    fn number(&mut self) {
        self.eat_while(|&c| c.is_numeric());
        let literal = if self.next_is('.') && self.peek_nth(1).map_or(false, |ch| ch.is_numeric()) {
            // floating point, e.g. 3.14
            self.advance();
            self.eat_while(|ch| ch.is_numeric());
            self.get_current_lexeme()
                .parse()
                .expect("TODO: real error handling")
        } else {
            // natural number, e.g. 69
            self.get_current_lexeme()
                .parse::<u64>()
                .expect("TODO: real error handling") as f64 // cast int to floating point num
        };
        self.add_basic_token(TokenKind::Number(literal));
    }

    fn identifier(&mut self) {
        self.eat_while(|c| c.is_alphanumeric());
        let text = self.get_current_lexeme();
        let kind = token_kind_for_text(&text);
        self.add_basic_token(kind);
    }

    fn get_current_lexeme(&self) -> String {
        self.get_lexeme(&Range(self.start, self.current))
    }

    fn get_lexeme(&self, range: &Range) -> String {
        self.source.range(range).iter().clone().collect()
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
        self.source.get_unchecked(self.current - 1)
    }

    fn peek(&self) -> Option<&char> {
        self.peek_nth(0)
    }

    fn peek_nth(&self, n: usize) -> Option<&char> {
        self.source.get(self.current + n)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_basic_token(&mut self, kind: TokenKind) {
        self.add_token(self.token(kind));
    }

    fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }

    fn token(&self, kind: TokenKind) -> Token {
        Token {
            source: Rc::clone(&self.source),
            kind,
            span: Range(self.start, self.current),
            line: self.line,
        }
    }
}

/// get the token kind (sans literal) for a piece of text. falls back to "identifier"
fn token_kind_for_text(text: &str) -> TokenKind {
    match text.as_ref() {
        "true" => TokenKind::True,
        "false" => TokenKind::False,
        "nil" => TokenKind::Nil,
        "and" => TokenKind::And,
        "class" => TokenKind::Class,
        "else" => TokenKind::Else,
        "for" => TokenKind::For,
        "fun" => TokenKind::Fun,
        "if" => TokenKind::If,
        "or" => TokenKind::Or,
        "print" => TokenKind::Print,
        "return" => TokenKind::Return,
        "super" => TokenKind::Super,
        "this" => TokenKind::This,
        "var" => TokenKind::Var,
        "while" => TokenKind::While,
        _ => TokenKind::Identifier,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::TokenKind::*;

    fn to_token_kinds(source: &str) -> Vec<TokenKind> {
        lex("<for testing>".to_string(), source)
            .iter()
            .map(|token| token.kind.clone())
            .collect()
    }

    #[test]
    fn test_var() {
        assert_eq!(
            to_token_kinds("var a;"),
            vec![Var, Identifier, Semicolon, Eof],
        );
    }

    #[test]
    fn test_var_with_init() {
        assert_eq!(
            to_token_kinds("var a = 1;"),
            vec![Var, Identifier, Equal, Number(1.0), Semicolon, Eof],
        );
    }
}
