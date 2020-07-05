#[derive(Clone, Debug)]
pub struct Range(pub usize, pub usize);

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: Range,
    pub literal: Option<Literal>,
    pub line: usize,
}

#[derive(Clone, Debug)]
pub enum TokenKind {
    // SingleCharacterTokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // OneOrTwoCharacterTokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Clone, Debug)]
pub enum Literal {
    Bool(bool),
    Nil,
    Number(f64),
    Str(String),
}
