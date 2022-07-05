use crate::token::Token;
use std::fmt::Display;

use crate::interpret::{self};

pub enum AstNodeKind {
    BinaryExpr,
    UnaryExpr,
    GroupExpr,
    LiteralExpr(Token),
    ExprStmt,
    PrintStmt,
    VarDecl,
    Block,
    Program,
    IfStmt,
    WhileStmt,
    AssignExpr,
    BreakStmt,
    FunCall,
}

pub trait AstNode: Display {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()>;
    fn kind(&self) -> AstNodeKind;
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
    variable: Token,
    expr: Option<AstNodeRef>,
}
pub struct AssignExpr {
    variable: Token,
    expr: AstNodeRef,
}
pub struct Program {
    decs: Vec<AstNodeRef>,
}
pub struct Block {
    decs: Vec<AstNodeRef>,
}
pub struct IfStmt {
    expr: AstNodeRef,
    stmt: AstNodeRef,
    elstmt: Option<AstNodeRef>,
}
pub struct WhileStmt {
    expr: AstNodeRef,
    stmt: AstNodeRef,
}
pub struct BreakStmt {
    token: Token,
}
pub struct FunCall {
    line: usize,
    callee: AstNodeRef,
    args: Vec<AstNodeRef>,
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
impl AssignExpr {
    pub fn create(variable: Token, expr: AstNodeRef) -> AstNodeRef {
        Box::new(AssignExpr { variable, expr })
    }
    pub fn variable(&self) -> &Token {
        &self.variable
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
    pub fn create(variable: Token, expr: Option<AstNodeRef>) -> AstNodeRef {
        Box::new(VarDecl { variable, expr })
    }
    pub fn expr(&self) -> Option<&AstNodeRef> {
        self.expr.as_ref()
    }
    pub fn name(&self) -> &Token {
        &self.variable
    }
}
impl Program {
    pub fn create(stmts: Vec<AstNodeRef>) -> AstNodeRef {
        Box::new(Program { decs: stmts })
    }
    pub fn decs(&self) -> &Vec<AstNodeRef> {
        &self.decs
    }
}
impl Block {
    pub fn create(decs: Vec<AstNodeRef>) -> AstNodeRef {
        Box::new(Block { decs })
    }
    pub fn decs(&self) -> &Vec<AstNodeRef> {
        &self.decs
    }
}
impl IfStmt {
    pub fn create(expr: AstNodeRef, stmt: AstNodeRef, elstmt: Option<AstNodeRef>) -> AstNodeRef {
        Box::new(IfStmt { expr, stmt, elstmt })
    }
    pub fn expr(&self) -> &AstNodeRef {
        &self.expr
    }
    pub fn stmt(&self) -> &AstNodeRef {
        &self.stmt
    }
    pub fn elstmt(&self) -> Option<&AstNodeRef> {
        self.elstmt.as_ref()
    }
}
impl WhileStmt {
    pub fn create(expr: AstNodeRef, stmt: AstNodeRef) -> AstNodeRef {
        Box::new(WhileStmt { expr, stmt })
    }
    pub fn expr(&self) -> &AstNodeRef {
        &self.expr
    }
    pub fn stmt(&self) -> &AstNodeRef {
        &self.stmt
    }
}
impl BreakStmt {
    pub fn create(token: Token) -> AstNodeRef {
        Box::new(BreakStmt { token })
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
}
impl FunCall {
    pub fn create(callee: AstNodeRef, args: Vec<AstNodeRef>, line: usize) -> AstNodeRef {
        Box::new(FunCall { callee, args, line })
    }
    pub fn callee(&self) -> &AstNodeRef {
        &self.callee
    }
    pub fn args(&self) -> &Vec<AstNodeRef> {
        &self.args
    }
    pub fn line(&self) -> usize {
        self.line
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
impl Display for AssignExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}={})", self.variable, self.expr)
    }
}
impl Display for VarDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.expr() {
            Some(e) => write!(f, "(var {}={});", self.variable, e),
            None => write!(f, "(var {});", self.variable,),
        }
    }
}
impl Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{{");
        for s in self.decs.iter() {
            write!(f, "{}\n", s)?;
        }
        write!(f, "}}")
    }
}
impl Display for IfStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.elstmt() {
            Some(el) => write!(f, "(if {} => {} | {})\n", self.expr, self.stmt, el),
            None => write!(f, "(if {} => {})\n", self.expr, self.stmt),
        }
    }
}
impl Display for WhileStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(while {} => {})\n", self.expr, self.stmt)
    }
}
impl Display for BreakStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "break")
    }
}
impl Display for FunCall {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}", self.callee)?;
        for a in self.args() {
            write!(f, " {},", a)?;
        }
        write!(f, ")")
    }
}
impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for s in self.decs.iter() {
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
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::BinaryExpr
    }
}
impl AstNode for UnaryExpr {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_unary(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::UnaryExpr
    }
}
impl AstNode for GroupExpr {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_group(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::GroupExpr
    }
}
impl AstNode for AssignExpr {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_assignment(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::AssignExpr
    }
}
impl AstNode for LiteralExpr {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_literal(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::LiteralExpr(self.token.clone())
    }
}
impl AstNode for ExprStmt {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_expr_stmt(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::ExprStmt
    }
}
impl AstNode for PrintStmt {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_print_stmt(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::PrintStmt
    }
}
impl AstNode for VarDecl {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_var_decl(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::VarDecl
    }
}
impl AstNode for IfStmt {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_if_stmt(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::IfStmt
    }
}
impl AstNode for WhileStmt {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_while_stmt(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::WhileStmt
    }
}
impl AstNode for BreakStmt {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_break_stmt(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::BreakStmt
    }
}
impl AstNode for FunCall {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_fun_call(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::FunCall
    }
}
impl AstNode for Program {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_program(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::Program
    }
}
impl AstNode for Block {
    fn interpret(&self, interpretor: &mut interpret::Interpretor) -> Result<interpret::Value, ()> {
        interpretor.interpret_block(self)
    }
    fn kind(&self) -> AstNodeKind {
        AstNodeKind::Block
    }
}
