use crate::ast::{
    Ast, AstNodeRef, BinaryExpr, GroupExpr, LiteralExpr, Token, TokenKind, UnaryExpr,
};

struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            current: 0,
            tokens: vec![],
        }
    }
    fn parse(&mut self, tokens: Vec<Token>) -> Option<Ast> {
        self.tokens = tokens;
        let rsl = self.parse_expression();
        self.current = 0;
        self.tokens.clear();
        match rsl {
            Ok(node) => Some(Ast::create(node)),
            Err(_) => None,
        }
    }
    fn parse_expression(&mut self) -> Result<AstNodeRef, ()> {
        self.parse_equality()
    }
    fn parse_equality(&mut self) -> Result<AstNodeRef, ()> {
        let mut expr = self.parse_comparison()?;
        while self.match_kinds(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
            let operator = self.previous();
            expr = BinaryExpr::create(operator, expr, self.parse_comparison()?);
        }
        Ok(expr)
    }
    fn parse_comparison(&mut self) -> Result<AstNodeRef, ()> {
        let mut expr = self.parse_term()?;
        while self.match_kinds(&[
            TokenKind::GreaterEqual,
            TokenKind::Greater,
            TokenKind::Less,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous();
            expr = BinaryExpr::create(operator, expr, self.parse_term()?);
        }
        Ok(expr)
    }
    fn parse_term(&mut self) -> Result<AstNodeRef, ()> {
        let mut expr = self.parse_factor()?;
        while self.match_kinds(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous();
            expr = BinaryExpr::create(operator, expr, self.parse_factor()?);
        }
        Ok(expr)
    }
    fn parse_factor(&mut self) -> Result<AstNodeRef, ()> {
        let mut expr = self.parse_unary()?;
        while self.match_kinds(&[TokenKind::Slash, TokenKind::Star]) {
            let operator = self.previous();
            expr = BinaryExpr::create(operator, expr, self.parse_unary()?);
        }
        Ok(expr)
    }
    fn parse_unary(&mut self) -> Result<AstNodeRef, ()> {
        let node = if self.match_kinds(&[TokenKind::Minus, TokenKind::Bang]) {
            let operator = self.previous();
            UnaryExpr::create(operator, self.parse_unary()?)
        } else {
            self.parse_primary()?
        };
        Ok(node)
    }
    fn parse_primary(&mut self) -> Result<AstNodeRef, ()> {
        if self.match_kinds(&[
            TokenKind::True,
            TokenKind::False,
            TokenKind::Number,
            TokenKind::String,
            TokenKind::Nil,
        ]) {
            Ok(LiteralExpr::create(self.previous()))
        } else if self.match_kinds(&[TokenKind::LeftParen]) {
            let expr = self.parse_expression()?;
            self.consume(TokenKind::RightParen)?;
            Ok(GroupExpr::create(expr))
        } else {
            super::lox_error(self.peek().line(), &format!("expression expected"));
            Err(())
        }
    }
    fn consume(&mut self, kind: TokenKind) -> Result<Token, ()> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            super::lox_error(self.peek().line(), &format!("Unbalanced parentheses"));
            Err(())
        }
    }

    fn match_kinds(&mut self, kinds: &[TokenKind]) -> bool {
        for tk in kinds {
            if self.check(*tk) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&mut self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind() == kind
    }
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }
    fn is_at_end(&mut self) -> bool {
        self.peek().kind() == TokenKind::EOF
    }
    fn peek(&mut self) -> Token {
        self.tokens[self.current].clone()
    }
    fn previous(&mut self) -> Token {
        self.tokens[self.current - 1].clone()
    }
}

pub fn parse(tokens: Vec<Token>) -> Option<Ast> {
    let mut parser = Parser::new();
    parser.parse(tokens)
}
