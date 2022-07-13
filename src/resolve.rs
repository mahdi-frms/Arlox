use crate::{
    ast::{AstNodeRef, LiteralExpr, NodeVisitor},
    environment::{Env, Environment},
    interpret::Value,
    token::{Token, TokenKind},
};

pub struct Resolver {
    errs: Vec<Token>,
    env: Env,
}

impl Resolver {
    fn func(&mut self, block: AstNodeRef, params: &Vec<Token>) -> <Self as NodeVisitor>::Retval {
        let prev = self.env.clone();
        self.env = Environment::new(Some(prev.clone()));
        for p in params {
            self.env.borrow_mut().init(p.text().to_string(), Value::Nil)
        }
        block.visit(self)?;
        self.env = prev;
        Ok(())
    }

    fn new() -> Resolver {
        Resolver {
            errs: vec![],
            env: Environment::new(None),
        }
    }
}

impl NodeVisitor for Resolver {
    type Retval = Result<(), ()>;

    fn visit_literal(&mut self, node: &LiteralExpr) -> <Self as NodeVisitor>::Retval {
        let token = node.token();
        if token.kind() == TokenKind::Identifier {
            match self.env.borrow().depth(token.text()) {
                None => self.errs.push(token.clone()),
                Some(depth) => node.set_depth(depth),
            }
        }
        Ok(())
    }

    fn visit_group(&mut self, node: &crate::ast::GroupExpr) -> <Self as NodeVisitor>::Retval {
        node.expr().visit(self)
    }

    fn visit_assignment(&mut self, node: &crate::ast::AssignExpr) -> <Self as NodeVisitor>::Retval {
        let token = node.variable();
        if token.kind() == TokenKind::Identifier {
            match self.env.borrow().depth(token.text()) {
                None => self.errs.push(token.clone()),
                Some(depth) => node.set_depth(depth),
            }
        }
        Ok(())
    }

    fn visit_if_stmt(&mut self, node: &crate::ast::IfStmt) -> <Self as NodeVisitor>::Retval {
        node.expr().visit(self)?;
        node.stmt().visit(self)
    }

    fn visit_while_stmt(&mut self, node: &crate::ast::WhileStmt) -> <Self as NodeVisitor>::Retval {
        node.expr().visit(self)?;
        node.stmt().visit(self)
    }

    fn visit_break_stmt(&mut self, _: &crate::ast::BreakStmt) -> <Self as NodeVisitor>::Retval {
        Ok(())
    }

    fn visit_return_stmt(
        &mut self,
        node: &crate::ast::ReturnStmt,
    ) -> <Self as NodeVisitor>::Retval {
        if let Some(expr) = node.expr() {
            expr.visit(self)?;
        }
        Ok(())
    }

    fn visit_unary(&mut self, node: &crate::ast::UnaryExpr) -> <Self as NodeVisitor>::Retval {
        node.expr().visit(self)
    }

    fn visit_binary(&mut self, node: &crate::ast::BinaryExpr) -> <Self as NodeVisitor>::Retval {
        node.lexpr().visit(self)?;
        node.rexpr().visit(self)
    }

    fn visit_print_stmt(&mut self, node: &crate::ast::PrintStmt) -> <Self as NodeVisitor>::Retval {
        node.expr().visit(self)
    }

    fn visit_expr_stmt(&mut self, node: &crate::ast::ExprStmt) -> <Self as NodeVisitor>::Retval {
        node.expr().visit(self)
    }

    fn visit_var_decl(&mut self, node: &crate::ast::VarDecl) -> <Self as NodeVisitor>::Retval {
        if let Some(expr) = node.expr() {
            expr.visit(self)?;
        }
        self.env
            .borrow_mut()
            .init(node.name().text().clone(), Value::Nil);
        Ok(())
    }

    fn visit_fun_decl(&mut self, node: &crate::ast::FunDecl) -> <Self as NodeVisitor>::Retval {
        self.func(node.block().clone(), node.params())
    }

    fn visit_fun_def(&mut self, node: &crate::ast::FunDef) -> <Self as NodeVisitor>::Retval {
        self.func(node.block().clone(), node.params())
    }

    fn visit_fun_call(&mut self, node: &crate::ast::FunCall) -> <Self as NodeVisitor>::Retval {
        node.callee().visit(self)?;
        for a in node.args() {
            a.visit(self)?;
        }
        Ok(())
    }

    fn visit_program(&mut self, node: &crate::ast::Program) -> <Self as NodeVisitor>::Retval {
        for d in node.decs() {
            d.visit(self)?;
        }
        Ok(())
    }

    fn visit_block(&mut self, node: &crate::ast::Block) -> <Self as NodeVisitor>::Retval {
        let prev = self.env.clone();
        self.env = Environment::new(Some(prev.clone()));
        for d in node.decs() {
            d.visit(self)?;
        }
        self.env = prev;
        Ok(())
    }
}
