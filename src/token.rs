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

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match *self {
            TokenKind::And => "keyword 'and'",
            TokenKind::Or => "keyword 'or'",
            TokenKind::Bang => "'!'",
            TokenKind::Comma => "','",
            TokenKind::LeftParen => "'('",
            TokenKind::RightParen => "')'",
            TokenKind::LeftBrace => "'{'",
            TokenKind::RightBrace => "'}'",
            TokenKind::Dot => "'.'",
            TokenKind::Minus => "'-'",
            TokenKind::Plus => "'+'",
            TokenKind::Semicolon => "';'",
            TokenKind::Slash => "'/'",
            TokenKind::Star => "'*'",
            TokenKind::BangEqual => "'!='",
            TokenKind::Equal => "'='",
            TokenKind::EqualEqual => "'=='",
            TokenKind::Greater => "'>'",
            TokenKind::GreaterEqual => "'>='",
            TokenKind::Less => "'<'",
            TokenKind::LessEqual => "'<='",
            TokenKind::Identifier => "identifier",
            TokenKind::String => "string literal",
            TokenKind::Number => "numeric literal",
            TokenKind::Class => "keyword 'class'",
            TokenKind::Else => "keyword 'else'",
            TokenKind::False => "keyword 'false'",
            TokenKind::Fun => "keyword 'fun'",
            TokenKind::For => "keyword 'for'",
            TokenKind::If => "keyword 'if'",
            TokenKind::Nil => "keyword 'nil'",
            TokenKind::Print => "keyword 'print'",
            TokenKind::Return => "keyword 'return'",
            TokenKind::Super => "keyword 'super'",
            TokenKind::This => "keyword 'this'",
            TokenKind::True => "keyword 'true'",
            TokenKind::Var => "keyword 'var'",
            TokenKind::While => "keyword 'while'",
            TokenKind::EOF => "keyword 'eof'",
        };
        write!(f, "{}", s)
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
