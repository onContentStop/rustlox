use crate::{expr::Expr, token::Token};

pub trait Visitor<T> {
    fn visit_block_stmt(&mut self, stmt: &Block) -> T;
    fn visit_expression_stmt(&mut self, stmt: &Expression) -> T;
    fn visit_function_stmt(&mut self, stmt: &Function) -> T;
    fn visit_if_stmt(&mut self, stmt: &If) -> T;
    fn visit_print_stmt(&mut self, stmt: &Print) -> T;
    fn visit_var_stmt(&mut self, stmt: &Var) -> T;
    fn visit_while_stmt(&mut self, stmt: &While) -> T;
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Block(Block),
    Expression(Expression),
    Function(Function),
    If(If),
    Print(Print),
    Var(Var),
    While(While),
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut impl Visitor<T>) -> T {
        match self {
            Stmt::Block(b) => visitor.visit_block_stmt(b),
            Stmt::Expression(e) => visitor.visit_expression_stmt(e),
            Stmt::Function(f) => visitor.visit_function_stmt(f),
            Stmt::If(i) => visitor.visit_if_stmt(i),
            Stmt::Print(p) => visitor.visit_print_stmt(p),
            Stmt::Var(v) => visitor.visit_var_stmt(v),
            Stmt::While(w) => visitor.visit_while_stmt(w),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Expr,
}

#[derive(Debug, Clone)]
pub struct Var {
    pub name: Token,
    pub initializer: Option<Expr>,
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}
