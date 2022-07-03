use crate::token::Token;
use std::fmt::Display;

use crate::interpret::{self};

pub trait AstNode: Display {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()>;
}
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
pub struct ExprStmt {
    expr: AstNodeRef,
}
pub struct PrintStmt {
    expr: AstNodeRef,
}
pub struct VarDecl {
    name: Token,
    expr: Option<AstNodeRef>,
}
pub struct Program {
    stmts: Vec<AstNodeRef>,
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
    pub fn rexpr(&self) -> &AstNodeRef {
        &self.rexpr
    }
    pub fn lexpr(&self) -> &AstNodeRef {
        &self.lexpr
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
}
impl UnaryExpr {
    pub fn create(token: Token, expr: AstNodeRef) -> AstNodeRef {
        Box::new(UnaryExpr { expr, token })
    }
    pub fn expr(&self) -> &AstNodeRef {
        &self.expr
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
}
impl LiteralExpr {
    pub fn create(token: Token) -> AstNodeRef {
        Box::new(LiteralExpr { token })
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
}
impl GroupExpr {
    pub fn create(expr: AstNodeRef) -> AstNodeRef {
        Box::new(GroupExpr { expr })
    }
    pub fn expr(&self) -> &AstNodeRef {
        &self.expr
    }
}
impl ExprStmt {
    pub fn create(expr: AstNodeRef) -> AstNodeRef {
        Box::new(ExprStmt { expr })
    }
    pub fn expr(&self) -> &AstNodeRef {
        &self.expr
    }
}
impl PrintStmt {
    pub fn create(expr: AstNodeRef) -> AstNodeRef {
        Box::new(PrintStmt { expr })
    }
    pub fn expr(&self) -> &AstNodeRef {
        &self.expr
    }
}
impl VarDecl {
    pub fn create(name: Token, expr: Option<AstNodeRef>) -> AstNodeRef {
        Box::new(VarDecl { name, expr })
    }
    pub fn expr(&self) -> Option<&AstNodeRef> {
        self.expr.as_ref()
    }
    pub fn name(&self) -> &Token {
        &self.name
    }
}
impl Program {
    pub fn create(stmts: Vec<AstNodeRef>) -> AstNodeRef {
        Box::new(Program { stmts })
    }
    pub fn stmts(&self) -> &Vec<AstNodeRef> {
        &self.stmts
    }
}
impl Ast {
    pub fn create(expr: AstNodeRef) -> Ast {
        Ast { root: expr }
    }
    pub fn root(&self) -> &AstNodeRef {
        &self.root
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
impl Display for ExprStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.expr)
    }
}
impl Display for PrintStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};", self.expr)
    }
}
impl Display for VarDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.expr() {
            Some(e) => write!(f, "(var {}={});", self.name, e),
            None => write!(f, "(var {});", self.name,),
        }
    }
}
impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.stmts.iter() {
            write!(f, "{}\n", s)?;
        }
        Ok(())
    }
}

impl Display for Ast {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.root)
    }
}

impl AstNode for BinaryExpr {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_binary(self)
    }
}
impl AstNode for UnaryExpr {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_unary(self)
    }
}
impl AstNode for GroupExpr {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_group(self)
    }
}
impl AstNode for LiteralExpr {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_literal(self)
    }
}
impl AstNode for ExprStmt {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_expr_stmt(self)
    }
}
impl AstNode for PrintStmt {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_print_stmt(self)
    }
}
impl AstNode for VarDecl {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_var_decl(self)
    }
}
impl AstNode for Program {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_program(self)
    }
}
