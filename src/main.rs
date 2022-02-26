use std::{env::args, process::exit};

const SINGLE_CHARS: &[char] = &['+', '-', '*', '/', ',', '}', '{', ')', '(', '.', ';'];
const DOUBLE_CHARS: &[char] = &['!', '=', '>', '<'];

#[derive(Debug)]
enum TokenKind {
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
struct Token {
    kind: TokenKind,
    text: String,
    line: usize,
}

fn lox_error(line: usize, text: &str) {
    println!("Error [line {}]: {}\n", line, text);
}

fn scan_single_char(c: char, line: usize) -> Token {
    Token {
        line,
        text: String::from(c),
        kind: match c {
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
    }
}

fn scan_double_char(c: char, line: usize) -> Token {
    Token {
        line,
        text: format!("{}=", String::from(c)),
        kind: match c {
            '>' => TokenKind::GreaterEqual,
            '<' => TokenKind::LessEqual,
            '=' => TokenKind::EqualEqual,
            _ => TokenKind::BangEqual, // '!'
        },
    }
}

fn scan_text(text: String, line: usize) -> Token {
    Token {
        line,
        kind: match text.as_str() {
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
    }
}

fn scan(text: Vec<char>) -> Vec<Token> {
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
            tokens.push(Token {
                line,
                text: buffer,
                kind: TokenKind::Number,
            });
            cindex -= 1;
        } else if c == '"' {
            let mut buffer = String::from(c);
            cindex += 1;
            loop {
                if cindex >= text.len() {
                    lox_error(line, "unbalanced quotes");
                    break;
                } else if text[cindex] == '"' {
                    buffer.push('"');
                    tokens.push(Token {
                        line,
                        text: buffer,
                        kind: TokenKind::String,
                    });
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
    tokens.push(Token {
        text: String::new(),
        kind: TokenKind::EOF,
        line,
    });
    tokens
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("help: lox [script]");
        exit(1);
    }
    let file = &args[1];
    let text = std::fs::read_to_string(file).expect(&format!("Error: cant open file {}", file));
    let tokens = scan(text.chars().collect::<Vec<char>>());
    println!("Tokens={:#?}", tokens);
}
