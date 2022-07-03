use crate::{
    ast::{
        AssignExpr, Ast, AstNodeRef, BinaryExpr, ExprStmt, GroupExpr, LiteralExpr, PrintStmt,
        Program, UnaryExpr, VarDecl,
    },
    lox_error,
    token::{Token, TokenKind},
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
    fn parse(&mut self, tokens: Vec<Token>, expr: bool) -> Option<Ast> {
        self.tokens = tokens;
        let rsl = if expr {
            self.parse_expression()
        } else {
            self.parse_program()
        };
        if expr && !self.is_at_end() {
            lox_error(0, "unexpected end of expression")
        }
        self.current = 0;
        self.tokens.clear();
        match rsl {
            Ok(node) => Some(Ast::create(node)),
            Err(_) => None,
        }
    }
    fn parse_program(&mut self) -> Result<AstNodeRef, ()> {
        let mut stmts = vec![];
        while !self.check(TokenKind::EOF) {
            let stmt = self.parse_declaration()?;
            stmts.push(stmt);
        }
        Ok(Program::create(stmts))
    }
    fn parse_declaration(&mut self) -> Result<AstNodeRef, ()> {
        if self.check(TokenKind::Var) {
            return self.parse_var_decl();
        }
        return self.parse_stmt();
    }
    fn parse_var_decl(&mut self) -> Result<AstNodeRef, ()> {
        self.advance();
        let id = self.consume(TokenKind::Identifier)?;
        let mut expr = None;
        if self.check(TokenKind::Equal) {
            self.advance();
            expr = Some(self.parse_expression()?);
        }
        self.consume(TokenKind::Semicolon)?;
        Ok(VarDecl::create(id, expr))
    }
    fn parse_stmt(&mut self) -> Result<AstNodeRef, ()> {
        let node;
        if self.check(TokenKind::Print) {
            self.advance();
            node = Ok(PrintStmt::create(self.parse_expression()?));
        } else {
            node = Ok(ExprStmt::create(self.parse_expression()?));
        }
        self.consume(TokenKind::Semicolon)?;
        node
    }
    fn parse_expression(&mut self) -> Result<AstNodeRef, ()> {
        self.parse_assignment()
    }
    fn parse_assignment(&mut self) -> Result<AstNodeRef, ()> {
        let mut nodes = vec![self.parse_equality()?];
        let mut lines = vec![self.peek().line()];
        while self.check(TokenKind::Equal) {
            let tkn = self.advance();
            lines.push(tkn.line());
            nodes.push(self.parse_equality()?);
        }
        let mut expr = nodes.pop().ok_or(())?;
        lines.pop().ok_or(())?;
        while nodes.len() > 0 {
            let node = nodes.pop().ok_or(())?;
            let line = lines.pop().ok_or(())?;
            match node.identifier() {
                Some(id) => expr = AssignExpr::create(id.clone(), expr),
                None => {
                    lox_error(line, "invalid l-value");
                    return Err(());
                }
            }
        }
        Ok(expr)
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
            TokenKind::Identifier,
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
            super::lox_error(self.peek().line(), &format!("expedted {}", kind));
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
            return kind == TokenKind::EOF;
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

pub fn parse_expresssion(tokens: Vec<Token>) -> Option<Ast> {
    let mut parser = Parser::new();
    parser.parse(tokens, true)
}

pub fn parse_source(tokens: Vec<Token>) -> Option<Ast> {
    let mut parser = Parser::new();
    parser.parse(tokens, false)
}
