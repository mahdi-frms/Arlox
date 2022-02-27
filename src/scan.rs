use crate::ast::{Token, TokenKind};

const SINGLE_CHARS: &[char] = &['+', '-', '*', '/', ',', '}', '{', ')', '(', '.', ';'];
const DOUBLE_CHARS: &[char] = &['!', '=', '>', '<'];

fn scan_single_char(c: char, line: usize) -> Token {
    Token::new(
        match c {
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '{' => TokenKind::LeftBrace,
            '}' => TokenKind::RightBrace,
            '(' => TokenKind::LeftParen,
            ')' => TokenKind::RightParen,
            ';' => TokenKind::Semicolon,
            '.' => TokenKind::Dot,
            '>' => TokenKind::Greater,
            '<' => TokenKind::Less,
            '=' => TokenKind::Equal,
            '!' => TokenKind::Bang,
            _ => TokenKind::Comma, // ','
        },
        String::from(c),
        line,
    )
}

fn scan_double_char(c: char, line: usize) -> Token {
    Token::new(
        match c {
            '>' => TokenKind::GreaterEqual,
            '<' => TokenKind::LessEqual,
            '=' => TokenKind::EqualEqual,
            _ => TokenKind::BangEqual, // '!'
        },
        format!("{}=", String::from(c)),
        line,
    )
}

fn scan_text(text: String, line: usize) -> Token {
    Token::new(
        match text.as_str() {
            "this" => TokenKind::This,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "class" => TokenKind::Class,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "nil" => TokenKind::Nil,
            "print" => TokenKind::Print,
            "var" => TokenKind::Var,
            "super" => TokenKind::Super,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "fun" => TokenKind::Fun,
            _ => TokenKind::Identifier,
        },
        text,
        line,
    )
}

pub fn scan(text: Vec<char>) -> Vec<Token> {
    let mut line = 0_usize;
    let mut tokens: Vec<Token> = vec![];
    let mut cindex = 0;
    while cindex < text.len() {
        let c = text[cindex];
        if c == '\n' {
            line += 1;
        } else if SINGLE_CHARS.contains(&c) {
            tokens.push(scan_single_char(c, line));
        } else if DOUBLE_CHARS.contains(&c) {
            if text.len() > cindex + 1 && text[cindex + 1] == '=' {
                tokens.push(scan_double_char(c, line));
                cindex += 1;
            } else {
                tokens.push(scan_single_char(c, line));
            }
        } else if c.is_ascii_alphabetic() || c == '_' {
            let mut buffer = String::new();
            while cindex < text.len()
                && (text[cindex].is_ascii_alphanumeric() || text[cindex] == '_')
            {
                buffer.push(text[cindex]);
                cindex += 1;
            }
            tokens.push(scan_text(buffer, line));
            cindex -= 1;
        } else if c.is_ascii_digit() {
            let mut dot_flag = false;
            let mut buffer = String::new();
            while (cindex < text.len() && text[cindex].is_ascii_digit())
                || (cindex + 1 < text.len()
                    && !dot_flag
                    && text[cindex] == '.'
                    && text[cindex + 1].is_ascii_digit())
            {
                if text[cindex] == '.' {
                    dot_flag = true;
                }
                buffer.push(text[cindex]);
                cindex += 1;
            }
            tokens.push(Token::new(TokenKind::Number, buffer, line));
            cindex -= 1;
        } else if c == '"' {
            let mut buffer = String::from(c);
            cindex += 1;
            loop {
                if cindex >= text.len() {
                    super::lox_error(line, "unbalanced quotes");
                    break;
                } else if text[cindex] == '"' {
                    buffer.push('"');
                    tokens.push(Token::new(TokenKind::String, buffer, line));

                    break;
                } else {
                    buffer.push(text[cindex]);
                    cindex += 1;
                }
            }
        } else if c != ' ' && c != '\t' {
        }
        cindex += 1;
    }
    tokens.push(Token::new(TokenKind::EOF, String::new(), line));
    tokens
}
