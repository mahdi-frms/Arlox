use std::fmt::Display;

#[derive(Debug)]
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

#[derive(Debug)]
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
}

trait AstNode: Display {}

struct BinaryExpr {
    token: Token,
    rexptr: Box<dyn AstNode>,
    lexpr: Box<dyn AstNode>,
}
struct UnaryExpr {
    token: Token,
    expr: Box<dyn AstNode>,
}
struct LiteralExpr {
    token: Token,
}
struct GroupExpr {
    expr: Box<dyn AstNode>,
}

impl Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.token, self.lexpr, self.rexptr)
    }
}
impl Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {})", self.token, self.expr)
    }
}
impl Display for GroupExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(group {})", self.expr)
    }
}
impl Display for LiteralExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.token)
    }
}
impl AstNode for BinaryExpr {}
impl AstNode for UnaryExpr {}
impl AstNode for GroupExpr {}
impl AstNode for LiteralExpr {}
