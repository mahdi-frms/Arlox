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
}

pub trait AstNode: Display {}
pub type AstNodeRef = Box<dyn AstNode>;

pub struct BinaryExpr {
    token: Token,
    rexpr: AstNodeRef,
    lexpr: AstNodeRef,
}
pub struct UnaryExpr {
    token: Token,
    expr: AstNodeRef,
}
pub struct LiteralExpr {
    token: Token,
}
pub struct GroupExpr {
    expr: AstNodeRef,
}

pub struct Ast {
    root: AstNodeRef,
}

impl BinaryExpr {
    pub fn create(token: Token, lexpr: AstNodeRef, rexpr: AstNodeRef) -> AstNodeRef {
        Box::new(BinaryExpr {
            lexpr,
            rexpr,
            token,
        })
    }
}
impl UnaryExpr {
    pub fn create(token: Token, expr: AstNodeRef) -> AstNodeRef {
        Box::new(UnaryExpr { expr, token })
    }
}
impl LiteralExpr {
    pub fn create(token: Token) -> AstNodeRef {
        Box::new(LiteralExpr { token })
    }
}
impl GroupExpr {
    pub fn create(expr: AstNodeRef) -> AstNodeRef {
        Box::new(GroupExpr { expr })
    }
}
impl Ast {
    pub fn create(expr: AstNodeRef) -> Ast {
        Ast { root: expr }
    }
}

impl Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {})", self.token, self.lexpr, self.rexpr)
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
impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}
impl AstNode for BinaryExpr {}
impl AstNode for UnaryExpr {}
impl AstNode for GroupExpr {}
impl AstNode for LiteralExpr {}
