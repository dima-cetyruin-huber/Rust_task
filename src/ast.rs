use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum Stmt {
    Let { name: String, value: Expr },
    Print(Expr),
    If {
        condition: Expr,
        consequence: Vec<Stmt>,
        alternative: Option<Vec<Stmt>>,
    },
}

#[derive(Debug, Clone)]
pub enum Expr {
    Int(i64),
    Bool(bool),
    String(String),
    Ident(String),
    Prefix {
        operator: Token,
        right: Box<Expr>,
    },
    Infix {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
}
