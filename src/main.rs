use std::fs;
use Rust_task_remake::{
    environment::Environment,
    evaluator::eval_program,
    lexer::Lexer,
    parser::Parser,
};
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Использование: Rust_task_remake <файл_программы>");
        return;
    }

    let source = fs::read_to_string(&args[1]).expect("Не удалось прочитать файл");

    let lexer = Lexer::new(&source);
    let mut parser = Parser::new(lexer);
    let program = parser.parse_program().expect("Ошибка парсинга");

    let env = Rc::new(RefCell::new(Environment::new()));

    match eval_program(&program, env) {
    Ok(_) => {}
    Err(e) => {
        eprintln!("Ошибка выполнения: {}", e.message);
    }
}
}
