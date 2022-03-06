use crate::ast::{Token, TokenKind};

const SINGLE_CHARS: &[char] = &['+', '-', '*', '/', ',', '}', '{', ')', '(', '.', ';'];
const DOUBLE_CHARS: &[char] = &['!', '=', '>', '<'];

#[derive(Default)]
struct Scanner {
    errflag: bool,
    line: usize,
    cindex: usize,
    tokens: Vec<Token>,
    text: Vec<char>,
}

impl Scanner {
    fn scan(&mut self, text: Vec<char>) -> Result<Vec<Token>, ()> {
        self.text = text;
        while self.cindex < self.text.len() {
            let c = self.text[self.cindex];
            if c == '\n' {
                self.line += 1;
            } else if SINGLE_CHARS.contains(&c) {
                self.tokens.push(Scanner::scan_single_char(c, self.line));
            } else if DOUBLE_CHARS.contains(&c) {
                if self.text.len() > self.cindex + 1 && self.text[self.cindex + 1] == '=' {
                    self.tokens.push(Scanner::scan_double_char(c, self.line));
                    self.cindex += 1;
                } else {
                    self.tokens.push(Scanner::scan_single_char(c, self.line));
                }
            } else if c.is_ascii_alphabetic() || c == '_' {
                self.scan_identifier();
            } else if c.is_ascii_digit() {
                self.scan_number();
            } else if c == '"' {
                self.scan_string();
            } else if c != ' ' && c != '\t' {
                super::lox_error(self.line, "unsupported characters");
                self.errflag = true;
            }
            self.cindex += 1;
        }

        self.tokens
            .push(Token::new(TokenKind::EOF, String::new(), self.line));
        if self.errflag {
            Err(())
        } else {
            Ok(std::mem::take(&mut self.tokens))
        }
    }
    fn scan_identifier(&mut self) {
        let mut buffer = String::new();
        while self.cindex < self.text.len()
            && (self.text[self.cindex].is_ascii_alphanumeric() || self.text[self.cindex] == '_')
        {
            buffer.push(self.text[self.cindex]);
            self.cindex += 1;
        }
        self.tokens.push(Scanner::scan_text(buffer, self.line));
        self.cindex -= 1;
    }
    fn scan_string(&mut self) {
        let mut buffer = String::from(self.text[self.cindex]);
        self.cindex += 1;
        loop {
            if self.cindex >= self.text.len() {
                super::lox_error(self.line, "unbalanced quotes");
                self.errflag = true;
                break;
            } else if self.text[self.cindex] == '"' {
                buffer.push('"');
                self.tokens
                    .push(Token::new(TokenKind::String, buffer, self.line));

                break;
            } else {
                buffer.push(self.text[self.cindex]);
                self.cindex += 1;
            }
        }
    }
    fn scan_number(&mut self) {
        let mut dot_flag = false;
        let mut buffer = String::new();
        while (self.cindex < self.text.len() && self.text[self.cindex].is_ascii_digit())
            || (self.cindex + 1 < self.text.len()
                && !dot_flag
                && self.text[self.cindex] == '.'
                && self.text[self.cindex + 1].is_ascii_digit())
        {
            if self.text[self.cindex] == '.' {
                dot_flag = true;
            }
            buffer.push(self.text[self.cindex]);
            self.cindex += 1;
        }
        self.tokens
            .push(Token::new(TokenKind::Number, buffer, self.line));
        self.cindex -= 1;
    }
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
}

pub fn scan(text: Vec<char>) -> Result<Vec<Token>, ()> {
    let mut scanner = Scanner::default();
    scanner.scan(text)
}
