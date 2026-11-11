use crate::ast::{Expr, Stmt};
use crate::environment::Environment;
use crate::lexer::Token;
use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct RuntimeError {
    pub message: String,
}

impl RuntimeError {
    pub fn new(msg: &str) -> Self {
        Self {
            message: msg.to_string(),
        }
    }
}

pub fn eval_program(stmts: &[Stmt], env: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
    for stmt in stmts {
        eval_statement(stmt, Rc::clone(&env))?;
    }
    Ok(())
}

fn eval_statement(stmt: &Stmt, env: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
    match stmt {
        Stmt::Let { name, value } => {
            let val = eval_expr(value, Rc::clone(&env))?;
            env.borrow_mut().set(name.clone(), val);
            Ok(())
        }
        Stmt::Print(expr) => {
            let val = eval_expr(expr, Rc::clone(&env))?;
            println!("{}", value_to_string(&val));
            Ok(())
        }
        Stmt::If {
            condition,
            consequence,
            alternative,
        } => {
            let cond_val = eval_expr(condition, Rc::clone(&env))?;

            let is_truthy = match cond_val {
                Value::Bool(b) => b,
                _ => {
                    return Err(RuntimeError::new(
                        "условие 'if' должно быть логическим значением (Bool)",
                    ));
                }
            };

            if is_truthy {
                eval_block_statement(consequence, Rc::clone(&env))?;
            } else if let Some(alt) = alternative {
                eval_block_statement(alt, Rc::clone(&env))?;
            }
            Ok(())
        }
    }
}

fn eval_block_statement(stmts: &[Stmt], env: Rc<RefCell<Environment>>) -> Result<(), RuntimeError> {
    let block_env = Rc::new(RefCell::new(Environment::new_enclosed(env)));
    for stmt in stmts {
        eval_statement(stmt, Rc::clone(&block_env))?;
    }
    Ok(())
}

fn eval_expr(expr: &Expr, env: Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
    match expr {
        Expr::Int(i) => Ok(Value::Int(*i)),
        Expr::Bool(b) => Ok(Value::Bool(*b)),
        Expr::String(s) => Ok(Value::String(s.clone())),
        Expr::Ident(name) => env
            .borrow()
            .get(name)
            .ok_or_else(|| RuntimeError::new(&format!("неопределенная переменная: {}", name))),
        Expr::Prefix { operator, right } => {
            let r = eval_expr(right, Rc::clone(&env))?;
            eval_prefix_expr(operator, &r)
        }
        Expr::Infix {
            left,
            operator,
            right,
        } => {
            let l = eval_expr(left, Rc::clone(&env))?;
            let r = eval_expr(right, Rc::clone(&env))?;
            eval_infix_expr(operator, &l, &r)
        }
    }
}

fn eval_prefix_expr(operator: &Token, right: &Value) -> Result<Value, RuntimeError> {
    match (operator, right) {
        (Token::Sub, Value::Int(i)) => Ok(Value::Int(-i)),
        (Token::Bang, Value::Bool(b)) => Ok(Value::Bool(!b)),
        _ => Err(RuntimeError::new("неподдерживаемый унарный оператор")),
    }
}

fn eval_infix_expr(operator: &Token, left: &Value, right: &Value) -> Result<Value, RuntimeError> {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => match operator {
            Token::Add => Ok(Value::Int(l + r)),
            Token::Sub => Ok(Value::Int(l - r)),
            Token::Mul => Ok(Value::Int(l * r)),
            Token::Div => {
                if *r == 0 {
                    Err(RuntimeError::new("деление на ноль"))
                } else {
                    Ok(Value::Int(l / r))
                }
            }
            Token::Mod => {
                if *r == 0 {
                    Err(RuntimeError::new("деление на ноль при взятии остатка"))
                } else {
                    Ok(Value::Int(l % r))
                }
            }
            Token::Eq => Ok(Value::Bool(l == r)),
            Token::NotEq => Ok(Value::Bool(l != r)),
            Token::Greater => Ok(Value::Bool(l > r)),
            Token::Less => Ok(Value::Bool(l < r)),
            Token::GreaterEq => Ok(Value::Bool(l >= r)),
            Token::LessEq => Ok(Value::Bool(l <= r)),
            _ => Err(RuntimeError::new("неизвестный инфиксный оператор для чисел")),
        },
        (Value::Bool(l), Value::Bool(r)) => match operator {
            Token::Eq => Ok(Value::Bool(l == r)),
            Token::NotEq => Ok(Value::Bool(l != r)),
            Token::And => Ok(Value::Bool(*l && *r)),
            _ => Err(RuntimeError::new("неизвестный инфиксный оператор для bool")),
        },
        _ => Err(RuntimeError::new("несоответствие типов в инфиксном выражении")),
    }
}

fn value_to_string(val: &Value) -> String {
    match val {
        Value::Int(i) => i.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::String(s) => s.clone(),
    }
}
