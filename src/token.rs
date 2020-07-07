use std::fmt::Debug;
use std::rc::Rc;

#[derive(Debug, Hash)]
pub struct Source {
    name: String,
    chars: Vec<char>,
}

impl Source {
    pub fn new(name: String, chars: Vec<char>) -> Self {
        Self { name, chars }
    }

    pub fn range(&self, range: &Range) -> &[char] {
        &self.chars[range.0..range.1]
    }

    pub fn get_unchecked(&self, index: usize) -> &char {
        unsafe { self.chars.get_unchecked(index) }
    }

    pub fn get(&self, index: usize) -> Option<&char> {
        self.chars.get(index)
    }

    pub fn len(&self) -> usize {
        self.chars.len()
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Range(pub usize, pub usize);

#[derive(Clone, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Range,
    pub line: usize,
    pub source: Rc<Source>,
}

impl Token {
    pub fn name(&self) -> String {
        self.source.range(&self.span).iter().collect()
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Token) -> bool {
        self.kind == other.kind
            && self.span == other.span
            && self.line == other.line
            && self.source.name == other.source.name
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    Str(String),
    Number(f64),

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

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    Bool(bool),
    Nil,
    Number(f64),
    Str(String),
}

impl Literal {
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Bool(val) => *val,
            _ => true,
        }
    }

    pub fn kind_name(&self) -> &str {
        match self {
            Literal::Bool(_) => "bool",
            Literal::Nil => "nil",
            Literal::Number(_) => "number",
            Literal::Str(_) => "string",
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Literal::Bool(true) => "true",
            Literal::Bool(false) => "false",
            Literal::Nil => "nil",
            Literal::Number(_) => "<number>",
            Literal::Str(text) => text,
        })
    }
}
