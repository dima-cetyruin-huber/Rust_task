use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Environment {
    store: HashMap<String, Value>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// Создает глобальное (корневое) окружение
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            outer: None,
        }
    }

    /// Создает вложенную область видимости для блоков { ... }
    pub fn new_enclosed(outer: Rc<RefCell<Environment>>) -> Self {
        Self {
            store: HashMap::new(),
            outer: Some(outer),
        }
    }

    /// Ищет переменную сначала в текущей области, затем во внешних
    pub fn get(&self, name: &str) -> Option<Value> {
        match self.store.get(name) {
            Some(val) => Some(val.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().get(name),
                None => None,
            },
        }
    }

    /// Сохраняет переменную в текущую область видимости
    pub fn set(&mut self, name: String, val: Value) {
        self.store.insert(name, val);
    }
}
