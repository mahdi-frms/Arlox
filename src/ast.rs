use crate::token::Token;
use std::fmt::Display;
use std::sync::Arc;

pub enum AstNode {
    BinaryExpr(BinaryExpr),
    UnaryExpr(UnaryExpr),
    GroupExpr(GroupExpr),
    LiteralExpr(LiteralExpr),
    ExprStmt(ExprStmt),
    PrintStmt(PrintStmt),
    VarDecl(VarDecl),
    Block(Block),
    Program(Program),
    IfStmt(IfStmt),
    WhileStmt(WhileStmt),
    AssignExpr(AssignExpr),
    BreakStmt(BreakStmt),
    ReturnStmt(ReturnStmt),
    FunCall(FunCall),
    FunDecl(FunDecl),
    FunDef(FunDef),
}

impl Display for AstNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AstNode::BinaryExpr(n) => write!(f, "{}", n),
            AstNode::UnaryExpr(n) => write!(f, "{}", n),
            AstNode::GroupExpr(n) => write!(f, "{}", n),
            AstNode::LiteralExpr(n) => write!(f, "{}", n),
            AstNode::ExprStmt(n) => write!(f, "{}", n),
            AstNode::PrintStmt(n) => write!(f, "{}", n),
            AstNode::VarDecl(n) => write!(f, "{}", n),
            AstNode::Block(n) => write!(f, "{}", n),
            AstNode::Program(n) => write!(f, "{}", n),
            AstNode::IfStmt(n) => write!(f, "{}", n),
            AstNode::WhileStmt(n) => write!(f, "{}", n),
            AstNode::AssignExpr(n) => write!(f, "{}", n),
            AstNode::BreakStmt(n) => write!(f, "{}", n),
            AstNode::ReturnStmt(n) => write!(f, "{}", n),
            AstNode::FunCall(n) => write!(f, "{}", n),
            AstNode::FunDecl(n) => write!(f, "{}", n),
            AstNode::FunDef(n) => write!(f, "{}", n),
        }
    }
}

impl AstNode {
    pub fn visit<V>(&self, visitor: &mut V) -> <V as NodeVisitor>::Retval
    where
        V: NodeVisitor,
    {
        match self {
            AstNode::BinaryExpr(n) => visitor.visit_binary(&n),
            AstNode::UnaryExpr(n) => visitor.visit_unary(&n),
            AstNode::GroupExpr(n) => visitor.visit_group(&n),
            AstNode::LiteralExpr(n) => visitor.visit_literal(&n),
            AstNode::ExprStmt(n) => visitor.visit_expr_stmt(&n),
            AstNode::PrintStmt(n) => visitor.visit_print_stmt(&n),
            AstNode::VarDecl(n) => visitor.visit_var_decl(&n),
            AstNode::Block(n) => visitor.visit_block(&n),
            AstNode::Program(n) => visitor.visit_program(&n),
            AstNode::IfStmt(n) => visitor.visit_if_stmt(&n),
            AstNode::WhileStmt(n) => visitor.visit_while_stmt(&n),
            AstNode::AssignExpr(n) => visitor.visit_assignment(&n),
            AstNode::BreakStmt(n) => visitor.visit_break_stmt(&n),
            AstNode::ReturnStmt(n) => visitor.visit_return_stmt(&n),
            AstNode::FunCall(n) => visitor.visit_fun_call(&n),
            AstNode::FunDecl(n) => visitor.visit_fun_decl(&n),
            AstNode::FunDef(n) => visitor.visit_fun_def(&n),
        }
    }
}

pub trait NodeVisitor {
    type Retval;
    fn visit_literal(&mut self, node: &LiteralExpr) -> <Self as NodeVisitor>::Retval;
    fn visit_group(&mut self, node: &GroupExpr) -> <Self as NodeVisitor>::Retval;
    fn visit_assignment(&mut self, node: &AssignExpr) -> <Self as NodeVisitor>::Retval;
    fn visit_if_stmt(&mut self, node: &IfStmt) -> <Self as NodeVisitor>::Retval;
    fn visit_while_stmt(&mut self, node: &WhileStmt) -> <Self as NodeVisitor>::Retval;
    fn visit_break_stmt(&mut self, node: &BreakStmt) -> <Self as NodeVisitor>::Retval;
    fn visit_return_stmt(&mut self, node: &ReturnStmt) -> <Self as NodeVisitor>::Retval;
    fn visit_unary(&mut self, node: &UnaryExpr) -> <Self as NodeVisitor>::Retval;
    fn visit_plus(&mut self, node: &BinaryExpr) -> <Self as NodeVisitor>::Retval;
    fn visit_math(&mut self, node: &BinaryExpr) -> <Self as NodeVisitor>::Retval;
    fn visit_and(&mut self, node: &BinaryExpr) -> <Self as NodeVisitor>::Retval;
    fn visit_or(&mut self, node: &BinaryExpr) -> <Self as NodeVisitor>::Retval;
    fn visit_binary(&mut self, node: &BinaryExpr) -> <Self as NodeVisitor>::Retval;
    fn visit_print_stmt(&mut self, node: &PrintStmt) -> <Self as NodeVisitor>::Retval;
    fn visit_expr_stmt(&mut self, node: &ExprStmt) -> <Self as NodeVisitor>::Retval;
    fn visit_var_decl(&mut self, node: &VarDecl) -> <Self as NodeVisitor>::Retval;
    fn visit_fun_decl(&mut self, node: &FunDecl) -> <Self as NodeVisitor>::Retval;
    fn visit_fun_def(&mut self, node: &FunDef) -> <Self as NodeVisitor>::Retval;
    fn visit_fun_call(&mut self, node: &FunCall) -> <Self as NodeVisitor>::Retval;
    fn visit_program(&mut self, node: &Program) -> <Self as NodeVisitor>::Retval;
    fn visit_block(&mut self, node: &Block) -> <Self as NodeVisitor>::Retval;
}

pub type AstNodeRef = Arc<AstNode>;

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
pub struct ReturnStmt {
    token: Token,
    expr: Option<AstNodeRef>,
}
pub struct FunCall {
    line: usize,
    callee: AstNodeRef,
    args: Vec<AstNodeRef>,
}
pub struct FunDecl {
    name: Token,
    params: Vec<Token>,
    block: AstNodeRef,
}
pub struct FunDef {
    params: Vec<Token>,
    block: AstNodeRef,
}

pub struct Ast {
    root: AstNodeRef,
}

impl BinaryExpr {
    pub fn create(token: Token, lexpr: AstNodeRef, rexpr: AstNodeRef) -> AstNodeRef {
        Arc::new(AstNode::BinaryExpr(BinaryExpr {
            lexpr,
            rexpr,
            token,
        }))
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
        Arc::new(AstNode::UnaryExpr(UnaryExpr { expr, token }))
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
        Arc::new(AstNode::LiteralExpr(LiteralExpr { token }))
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
}
impl GroupExpr {
    pub fn create(expr: AstNodeRef) -> AstNodeRef {
        Arc::new(AstNode::GroupExpr(GroupExpr { expr }))
    }
    pub fn expr(&self) -> &AstNodeRef {
        &self.expr
    }
}
impl AssignExpr {
    pub fn create(variable: Token, expr: AstNodeRef) -> AstNodeRef {
        Arc::new(AstNode::AssignExpr(AssignExpr { variable, expr }))
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
        Arc::new(AstNode::ExprStmt(ExprStmt { expr }))
    }
    pub fn expr(&self) -> &AstNodeRef {
        &self.expr
    }
}
impl PrintStmt {
    pub fn create(expr: AstNodeRef) -> AstNodeRef {
        Arc::new(AstNode::PrintStmt(PrintStmt { expr }))
    }
    pub fn expr(&self) -> &AstNodeRef {
        &self.expr
    }
}
impl VarDecl {
    pub fn create(variable: Token, expr: Option<AstNodeRef>) -> AstNodeRef {
        Arc::new(AstNode::VarDecl(VarDecl { variable, expr }))
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
        Arc::new(AstNode::Program(Program { decs: stmts }))
    }
    pub fn decs(&self) -> &Vec<AstNodeRef> {
        &self.decs
    }
}
impl Block {
    pub fn create(decs: Vec<AstNodeRef>) -> AstNodeRef {
        Arc::new(AstNode::Block(Block { decs }))
    }
    pub fn decs(&self) -> &Vec<AstNodeRef> {
        &self.decs
    }
}
impl IfStmt {
    pub fn create(expr: AstNodeRef, stmt: AstNodeRef, elstmt: Option<AstNodeRef>) -> AstNodeRef {
        Arc::new(AstNode::IfStmt(IfStmt { expr, stmt, elstmt }))
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
        Arc::new(AstNode::WhileStmt(WhileStmt { expr, stmt }))
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
        Arc::new(AstNode::BreakStmt(BreakStmt { token }))
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
}
impl ReturnStmt {
    pub fn create(token: Token, expr: Option<AstNodeRef>) -> AstNodeRef {
        Arc::new(AstNode::ReturnStmt(ReturnStmt { token, expr }))
    }
    pub fn token(&self) -> &Token {
        &self.token
    }
    pub fn expr(&self) -> Option<&AstNodeRef> {
        self.expr.as_ref()
    }
}
impl FunCall {
    pub fn create(callee: AstNodeRef, args: Vec<AstNodeRef>, line: usize) -> AstNodeRef {
        Arc::new(AstNode::FunCall(FunCall { callee, args, line }))
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
impl FunDecl {
    pub fn create(name: Token, args: Vec<Token>, block: AstNodeRef) -> AstNodeRef {
        Arc::new(AstNode::FunDecl(FunDecl {
            name,
            params: args,
            block,
        }))
    }

    pub fn name(&self) -> &Token {
        &self.name
    }

    pub fn params(&self) -> &Vec<Token> {
        &self.params
    }

    pub fn block(&self) -> &AstNodeRef {
        &self.block
    }
}

impl FunDef {
    pub fn create(args: Vec<Token>, block: AstNodeRef) -> AstNodeRef {
        Arc::new(AstNode::FunDef(FunDef {
            params: args,
            block,
        }))
    }

    pub fn params(&self) -> &Vec<Token> {
        &self.params
    }

    pub fn block(&self) -> &AstNodeRef {
        &self.block
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
impl Display for ReturnStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.expr() {
            Some(e) => write!(f, "(return {})", e),
            None => write!(f, "return"),
        }
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
impl Display for FunDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} ", self.name())?;
        for a in self.params.iter() {
            write!(f, "{} ", a.text())?;
        }
        write!(f, ")")?;
        write!(f, "{}", self.block())
    }
}
impl Display for FunDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "([fun] ")?;
        for a in self.params.iter() {
            write!(f, "{} ", a.text())?;
        }
        write!(f, ")")?;
        write!(f, "{}", self.block())
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
