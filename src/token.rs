pub struct Token {
    pub kind: TokenType,
    pub lexeme: Option<String>,
    pub literal: Option<Literal>,
    pub line: usize,
}

pub enum TokenType {
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

pub enum Literal {
    Bool { value: bool },
    Nil,
    Number { value: f64 },
    Str { value: String },
}
