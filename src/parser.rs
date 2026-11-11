use crate::ast::{Expr, Stmt};
use crate::lexer::{Lexer, Token};

#[derive(PartialEq, PartialOrd)]
enum Precedence {
    Lowest,
    And,         // &&
    Equals,      // == !=
    LessGreater, // > < >= <=
    Sum,         // + -
    Product,     // * / %
    Prefix,      // -X или !X
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();
        Self {
            lexer,
            current_token,
            peek_token,
        }
    }

    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    fn current_token_is(&self, t: &Token) -> bool {
        std::mem::discriminant(&self.current_token) == std::mem::discriminant(t)
    }

    fn peek_token_is(&self, t: &Token) -> bool {
        std::mem::discriminant(&self.peek_token) == std::mem::discriminant(t)
    }

    fn expect_peek(&mut self, t: &Token) -> bool {
        if self.peek_token_is(t) {
            self.next_token();
            true
        } else {
            false
        }
    }

    fn peek_precedence(&self) -> Precedence {
        Self::token_precedence(&self.peek_token)
    }

    fn current_precedence(&self) -> Precedence {
        Self::token_precedence(&self.current_token)
    }

    fn token_precedence(tok: &Token) -> Precedence {
        match tok {
            Token::And => Precedence::And,
            Token::Eq | Token::NotEq => Precedence::Equals,
            Token::Less | Token::Greater | Token::LessEq | Token::GreaterEq => {
                Precedence::LessGreater
            }
            Token::Add | Token::Sub => Precedence::Sum,
            Token::Mul | Token::Div | Token::Mod => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }

    pub fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts = Vec::new();
        while !self.current_token_is(&Token::EOF) {
            if let Some(stmt) = self.parse_statement()? {
                stmts.push(stmt);
            }
            self.next_token();
        }
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Option<Stmt>, String> {
        match self.current_token {
            Token::Let => Ok(Some(self.parse_let_statement()?)),
            Token::Print => Ok(Some(self.parse_print_statement()?)),
            Token::If => Ok(Some(self.parse_if_statement()?)),
            Token::Semicolon => Ok(None),
            _ => Err(format!("Неожиданный токен: {:?}", self.current_token)),
        }
    }

    fn parse_let_statement(&mut self) -> Result<Stmt, String> {
        let name = match &self.peek_token {
            Token::Ident(n) => n.clone(),
            _ => return Err("Ожидалось имя переменной после 'let'".to_string()),
        };
        self.next_token();

        if !self.expect_peek(&Token::Assign) {
            return Err("Ожидался знак '=' после имени переменной".to_string());
        }
        self.next_token();

        let value = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(Stmt::Let { name, value })
    }

    fn parse_print_statement(&mut self) -> Result<Stmt, String> {
        self.next_token(); // съедаем 'print'
        let expr = self.parse_expression(Precedence::Lowest)?;

        if self.peek_token_is(&Token::Semicolon) {
            self.next_token();
        }

        Ok(Stmt::Print(expr))
    }

    fn parse_if_statement(&mut self) -> Result<Stmt, String> {
        self.next_token(); // съедаем 'if'
        let condition = self.parse_expression(Precedence::Lowest)?;

        if !self.expect_peek(&Token::LBrace) {
            return Err("Ожидалась '{' после условия 'if'".to_string());
        }

        let consequence = self.parse_block_statement()?;

        let alternative = if self.peek_token_is(&Token::Else) {
            self.next_token(); // съедаем 'else'
            if self.peek_token_is(&Token::If) {
                self.next_token(); // переходим на 'if'
                Some(vec![self.parse_if_statement()?])
            } else if self.expect_peek(&Token::LBrace) {
                Some(self.parse_block_statement()?)
            } else {
                return Err("Ожидалась '{' или 'if' после 'else'".to_string());
            }
        } else {
            None
        };

        Ok(Stmt::If {
            condition,
            consequence,
            alternative,
        })
    }

    fn parse_block_statement(&mut self) -> Result<Vec<Stmt>, String> {
        self.next_token(); // съедаем '{'
        let mut stmts = Vec::new();

        while !self.current_token_is(&Token::RBrace) && !self.current_token_is(&Token::EOF) {
            if let Some(stmt) = self.parse_statement()? {
                stmts.push(stmt);
            }
            self.next_token();
        }
        Ok(stmts)
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Result<Expr, String> {
        let mut left = self.parse_prefix()?;

        while !self.peek_token_is(&Token::Semicolon) && precedence < self.peek_precedence() {
            self.next_token();
            left = self.parse_infix(left)?;
        }

        Ok(left)
    }

    fn parse_prefix(&mut self) -> Result<Expr, String> {
        match &self.current_token {
            Token::Int(val) => Ok(Expr::Int(*val)),
            Token::True => Ok(Expr::Bool(true)),
            Token::False => Ok(Expr::Bool(false)),
            Token::String(s) => Ok(Expr::String(s.clone())),
            Token::Ident(name) => Ok(Expr::Ident(name.clone())),
            Token::Sub | Token::Bang => {
                let operator = self.current_token.clone();
                self.next_token();
                let right = self.parse_expression(Precedence::Prefix)?;
                Ok(Expr::Prefix {
                    operator,
                    right: Box::new(right),
                })
            }
            Token::LParen => {
                self.next_token();
                let expr = self.parse_expression(Precedence::Lowest)?;
                if !self.expect_peek(&Token::RParen) {
                    return Err("Ожидалась закрывающая скобка ')'".to_string());
                }
                Ok(expr)
            }
            _ => Err(format!("Нет префиксного обработчика для {:?}", self.current_token)),
        }
    }

    fn parse_infix(&mut self, left: Expr) -> Result<Expr, String> {
        let operator = self.current_token.clone();
        let precedence = self.current_precedence();
        self.next_token();
        let right = self.parse_expression(precedence)?;

        Ok(Expr::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }
}
