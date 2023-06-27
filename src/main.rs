mod environment;
mod interpreter;
mod matcha;
mod parser;
mod scanner;
mod statement;
mod token;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::println;

use environment::Environment;

use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

struct Options {
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
            "--lexer_out" => {
                options.lexer_out = true;
            }
            _ => {
                eprintln!("Unknown argument {}", arg.split_at(2).1)
            }
        }
    }

    if files.len() > 0 {
        for file in files {
            run_file(&options, &file);
        }
    } else {
        repl(&options);
    }
}

fn run_file(options: &Options, path: &str) {
    let contents = fs::read_to_string(path).unwrap();
    let mut environment = Environment::new();

    let exit_code = run(options, &contents, &mut environment);

    if exit_code != 0 {
        std::process::exit(1);
    }
}

fn repl(options: &Options) {
    let mut line = String::new();
    println!("Matcha 🍵 {}", env!("CARGO_PKG_VERSION"));

    let mut environment = Environment::new();

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        if line.len() > 0 {
            run(options, &line, &mut environment);
        }

        line.clear();
    }
}

fn run(options: &Options, program: &str, environment: &mut Environment) -> u8 {
    let tokens_result = Scanner::scan(program);

    match tokens_result {
        Ok(tokens) => {
            if options.lexer_out {
                println!("{:#?}", tokens);
            }

            let mut parser = Parser::new(tokens);
            let parser_result = parser.parse();

            match parser_result {
                Ok(statements) => {
                    if options.ast {
                        for statement in &statements {
                            println!("{}", statement.format(0));
                        }
                    }

                    let interpreter_result = Interpreter::interpret(environment, &statements);

                    match interpreter_result {
                        Ok(result) => println!("{}", result),
                        Err(e) => eprintln!("{:#?}", e),
                    }
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{}", error);
                        return 1;
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            return 1;
        }
    }

    return 0;
}
