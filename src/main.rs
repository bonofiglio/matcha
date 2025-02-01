mod environment;
// mod interpreter;
mod matcha;
mod parser;
mod scanner;
mod source;
mod statement;
mod tests;
mod token;

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::println;
use std::rc::Rc;

use environment::Environment;
use matcha::Literal;
use matcha::NumberLiteral;
use matcha::Value;
use source::Source;

// use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

#[cfg_attr(test, derive(Default))]
pub struct Options {
    pub ast: bool,
    pub lexer_out: bool,
}

fn main() {
    let args: Vec<String> = env::args()
        .skip(1)
        .filter(|arg| arg.starts_with("--"))
        .collect();
    let files: Vec<String> = env::args()
        .skip(1)
        .filter(|arg| !arg.starts_with("--"))
        .collect();
    let mut options = Options {
        ast: false,
        lexer_out: false,
    };

    for arg in args {
        match arg.as_str() {
            "--ast" => {
                options.ast = true;
            }
            "--lexer-out" => {
                options.lexer_out = true;
            }
            _ => {
                eprintln!("Unknown argument {}", arg.split_at(2).1)
            }
        }
    }

    if !files.is_empty() {
        for file in files {
            run_file(&options, &file);
        }
    } else {
        repl(&options);
    }
}

fn run_file(options: &Options, path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let environment = Rc::new(RefCell::new(Environment::new()));

    let exit_code = run(options, &contents, environment);

    if exit_code != 0 {
        std::process::exit(1);
    }
}

#[derive(Clone)]
pub enum OwnedValue {
    Empty,
    Optional(Option<OwnedLiteral>),
    Literal(OwnedLiteral),
}

impl<'a> From<&'a OwnedValue> for Value<'a> {
    fn from(value: &'a OwnedValue) -> Value<'a> {
        match value {
            OwnedValue::Empty => Value::Empty,
            OwnedValue::Literal(l) => Value::Literal(match l {
                OwnedLiteral::Boolean(v) => Literal::Boolean(*v),
                OwnedLiteral::Number(n) => Literal::Number(n.clone()),
                OwnedLiteral::String(s) => Literal::String(s),
            }),
            OwnedValue::Optional(o) => Value::Optional(match o {
                None => None,
                Some(OwnedLiteral::Boolean(v)) => Some(Literal::Boolean(*v)),
                Some(OwnedLiteral::Number(n)) => Some(Literal::Number(n.clone())),
                Some(OwnedLiteral::String(s)) => Some(Literal::String(s)),
            }),
        }
    }
}

impl From<&Value<'_>> for OwnedValue {
    fn from(value: &Value<'_>) -> OwnedValue {
        match value {
            Value::Empty => OwnedValue::Empty,
            Value::Literal(l) => OwnedValue::Literal(match l {
                Literal::Boolean(v) => OwnedLiteral::Boolean(*v),
                Literal::Number(n) => OwnedLiteral::Number(n.clone()),
                Literal::String(s) => OwnedLiteral::String(s.to_string()),
            }),
            Value::Optional(o) => OwnedValue::Optional(match o {
                None => None,
                Some(Literal::Boolean(v)) => Some(OwnedLiteral::Boolean(*v)),
                Some(Literal::Number(n)) => Some(OwnedLiteral::Number(n.clone())),
                Some(Literal::String(s)) => Some(OwnedLiteral::String(s.to_string())),
            }),
        }
    }
}

#[derive(Clone)]
pub enum OwnedLiteral {
    String(String),
    Number(NumberLiteral),
    Boolean(bool),
}

fn repl(options: &Options) {
    println!("Matcha 🍵 {}", env!("CARGO_PKG_VERSION"));
    let mut line = String::new();
    let mut prev_environment = HashMap::<String, OwnedValue>::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();

        if !line.is_empty() {
            let _env = prev_environment.clone();

            let environment = Environment {
                values: _env
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.into()))
                    .collect(),
                parent: None,
            };

            let environment = Rc::new(RefCell::new(environment));

            run(options, &line, Rc::clone(&environment));

            prev_environment = environment
                .borrow()
                .values
                .iter()
                .map(|(k, v)| (k.to_string(), v.into()))
                .collect();
        }

        line.clear()
    }
}

pub fn run<'a>(
    options: &Options,
    program: &'a str,
    environment: Rc<RefCell<Environment<'a>>>,
) -> u8 {
    let mut scanner = Scanner {
        source: Source::new(program),
    };

    let tokens_result = scanner.scan();

    match tokens_result {
        Ok(tokens) => {
            if options.lexer_out {
                println!("{:#?}", tokens);
            }

            let parser = Parser::new(tokens);
            let parser_result = parser.parse();

            match parser_result {
                Ok(statements) => {
                    if options.ast {
                        for statement in &statements {
                            println!("{}", statement.format(0));
                        }
                    }

                    0

                    // let interpreter_result = Interpreter::interpret(environment, &statements);

                    // match interpreter_result {
                    //     Ok(result) => {
                    //         println!("{}", result);
                    //         0
                    //     }
                    //     Err(e) => {
                    //         eprintln!("{:#?}", e);
                    //         1
                    //     }
                    // }
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{}", error);
                    }
                    1
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    }
}
