use crate::{
    ast::{
        AssignExpr, Ast, AstNodeKind, AstNodeRef, BinaryExpr, Block, BreakStmt, ExprStmt,
        GroupExpr, IfStmt, LiteralExpr, PrintStmt, Program, UnaryExpr, VarDecl, WhileStmt,
    },
    lox_error,
    token::{Token, TokenKind},
};

struct Parser {
    current: usize,
    tokens: Vec<Token>,
}

fn desugar_for(
    initialize: Option<AstNodeRef>,
    condition: Option<AstNodeRef>,
    increment: Option<AstNodeRef>,
    stmt: AstNodeRef,
) -> AstNodeRef {
    let mut stmt = stmt;
    let condition = condition
        .unwrap_or_else(|| LiteralExpr::create(Token::new(TokenKind::True, "true".to_string(), 0)));
    if let Some(inc) = increment {
        stmt = Block::create(vec![stmt, inc]);
    }
    let mut whloop = WhileStmt::create(condition, stmt);
    if let Some(init) = initialize {
        whloop = Block::create(vec![init, whloop]);
    }
    whloop
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
        let mut decs = vec![];
        while !self.check(TokenKind::EOF) {
            let stmt = self.parse_declaration()?;
            decs.push(stmt);
        }
        Ok(Program::create(decs))
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
        if self.check(TokenKind::If) {
            node = self.parse_if_stmt();
        } else if self.check(TokenKind::While) {
            node = self.parse_while_stmt();
        } else if self.check(TokenKind::Break) {
            node = Ok(BreakStmt::create(self.advance()));
            self.consume(TokenKind::Semicolon)?;
        } else if self.check(TokenKind::For) {
            node = self.parse_for_stmt();
        } else if self.check(TokenKind::Print) {
            self.advance();
            node = Ok(PrintStmt::create(self.parse_expression()?));
            self.consume(TokenKind::Semicolon)?;
        } else if self.check(TokenKind::LeftBrace) {
            node = self.parse_block();
        } else {
            node = Ok(ExprStmt::create(self.parse_expression()?));
            self.consume(TokenKind::Semicolon)?;
        }
        node
    }
    fn parse_if_stmt(&mut self) -> Result<AstNodeRef, ()> {
        self.advance();
        self.consume(TokenKind::LeftParen)?;
        let expr = self.parse_expression()?;
        self.consume(TokenKind::RightParen)?;
        let stmt = self.parse_stmt()?;
        let mut elstmt = None;
        if self.check(TokenKind::Else) {
            self.advance();
            elstmt = Some(self.parse_stmt()?);
        }
        Ok(IfStmt::create(expr, stmt, elstmt))
    }
    fn parse_while_stmt(&mut self) -> Result<AstNodeRef, ()> {
        self.advance();
        self.consume(TokenKind::LeftParen)?;
        let expr = self.parse_expression()?;
        self.consume(TokenKind::RightParen)?;
        let stmt = self.parse_stmt()?;
        Ok(WhileStmt::create(expr, stmt))
    }
    fn parse_for_stmt(&mut self) -> Result<AstNodeRef, ()> {
        self.advance();
        self.consume(TokenKind::LeftParen)?;

        let initialize;
        if self.check(TokenKind::Semicolon) {
            initialize = None;
            self.consume(TokenKind::Semicolon)?;
        } else if self.check(TokenKind::Var) {
            initialize = Some(self.parse_var_decl()?);
        } else {
            initialize = Some(self.parse_expression()?);
            self.consume(TokenKind::Semicolon)?;
        }

        let mut condition = None;
        if !self.check(TokenKind::Semicolon) {
            condition = Some(self.parse_expression()?);
        }
        self.consume(TokenKind::Semicolon)?;

        let mut increment = None;
        if !self.check(TokenKind::RightParen) {
            increment = Some(self.parse_expression()?);
        }

        self.consume(TokenKind::RightParen)?;
        let stmt = self.parse_stmt()?;

        Ok(desugar_for(initialize, condition, increment, stmt))
    }
    fn parse_block(&mut self) -> Result<AstNodeRef, ()> {
        self.advance();
        let mut decs = vec![];
        while !self.check(TokenKind::RightBrace) {
            decs.push(self.parse_declaration()?)
        }
        self.advance();
        Ok(Block::create(decs))
    }
    fn parse_expression(&mut self) -> Result<AstNodeRef, ()> {
        self.parse_assignment()
    }
    fn parse_assignment(&mut self) -> Result<AstNodeRef, ()> {
        let mut nodes = vec![self.parse_logic_or()?];
        let mut lines = vec![self.peek().line()];
        while self.check(TokenKind::Equal) {
            let tkn = self.advance();
            lines.push(tkn.line());
            nodes.push(self.parse_logic_or()?);
        }
        let mut expr = nodes.pop().ok_or(())?;
        lines.pop();
        while nodes.len() > 0 {
            let node = nodes.pop().ok_or(())?;
            let line = lines.pop().ok_or(())?;
            match node.kind() {
                AstNodeKind::LiteralExpr(tkn) => {
                    expr = match tkn.kind() {
                        TokenKind::Identifier => AssignExpr::create(tkn.clone(), expr),
                        _ => {
                            lox_error(line, "invalid l-value");
                            return Err(());
                        }
                    }
                }
                _ => {
                    lox_error(line, "invalid l-value");
                    return Err(());
                }
            }
        }
        Ok(expr)
    }
    fn parse_logic_or(&mut self) -> Result<AstNodeRef, ()> {
        let mut lexpr = self.parse_logic_and()?;
        while self.check(TokenKind::Or) {
            let opr = self.advance();
            let rexpr = self.parse_logic_and()?;
            lexpr = BinaryExpr::create(opr, lexpr, rexpr);
        }
        Ok(lexpr)
    }
    fn parse_logic_and(&mut self) -> Result<AstNodeRef, ()> {
        let mut lexpr = self.parse_equality()?;
        while self.check(TokenKind::And) {
            let opr = self.advance();
            let rexpr = self.parse_equality()?;
            lexpr = BinaryExpr::create(opr, lexpr, rexpr);
        }
        Ok(lexpr)
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
