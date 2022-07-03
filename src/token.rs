use std::fmt::Display;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TokenKind {
    // Single-character tokens.
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
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

    EOF,
}

#[derive(Debug, Clone)]
#[allow(unused)]
pub struct Token {
    kind: TokenKind,
    text: String,
    line: usize,
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.text)
    }
}

impl Token {
    pub fn new(kind: TokenKind, text: String, line: usize) -> Token {
        Token { kind, text, line }
    }
    pub fn kind(&self) -> TokenKind {
        self.kind
    }
    pub fn line(&self) -> usize {
        self.line
    }
    pub fn text(&self) -> &String {
        &self.text
    }
}
