#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal,
    EOF,

    Ident(String),
    Int(i64),
    String(String),

    Assign,    // =
    Add,       // +
    Sub,       // -
    Mul,       // *
    Div,       // /
    Mod,       // %
    Bang,      // !
    And,       // &&

    Eq,        // ==
    NotEq,     // !=
    Greater,   // >
    Less,      // <
    GreaterEq, // >=
    LessEq,    // <=

    Semicolon, // ;
    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }

    Let,
    Print,
    If,
    Else,
    True,
    False,
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut l = Self {
            input: input.chars().collect(),
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn peek_char(&self) -> char {
        if self.read_position >= self.input.len() {
            '\0'
        } else {
            self.input[self.read_position]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.position;
        while self.ch.is_ascii_alphanumeric() || self.ch == '_' {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    fn read_number(&mut self) -> i64 {
        let start = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char();
        }
        let s: String = self.input[start..self.position].iter().collect();
        s.parse().unwrap_or(0)
    }

    fn read_string(&mut self) -> String {
        self.read_char(); // пропускаем открывающую '"'
        let start = self.position;
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        self.input[start..self.position].iter().collect()
    }

    pub fn next_token(&mut self) -> Token {
        loop {
            self.skip_whitespace();
            if self.ch == '/' && self.peek_char() == '/' {
                while self.ch != '\n' && self.ch != '\0' {
                    self.read_char();
                }
            } else {
                break;
            }
        }

        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::Eq
                } else {
                    Token::Assign
                }
            }
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    Token::Bang
                }
            }
            '>' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::GreaterEq
                } else {
                    Token::Greater
                }
            }
            '<' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Token::LessEq
                } else {
                    Token::Less
                }
            }
            '&' => {
                if self.peek_char() == '&' {
                    self.read_char();
                    Token::And
                } else {
                    Token::Illegal
                }
            }
            '+' => Token::Add,
            '-' => Token::Sub,
            '*' => Token::Mul,
            '/' => Token::Div,
            '%' => Token::Mod,
            ';' => Token::Semicolon,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '"' => Token::String(self.read_string()),
            '\0' => Token::EOF,
            _ => {
                if self.ch.is_ascii_alphabetic() || self.ch == '_' {
                    let ident = self.read_identifier();
                    return match ident.as_str() {
                        "let" => Token::Let,
                        "print" => Token::Print,
                        "if" => Token::If,
                        "else" => Token::Else,
                        "true" => Token::True,
                        "false" => Token::False,
                        _ => Token::Ident(ident),
                    };
                } else if self.ch.is_ascii_digit() {
                    return Token::Int(self.read_number());
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        tok
    }
}
