mod ast;
mod interpreter;
mod parser;
mod scanner;
mod token;
mod vitus;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::println;

use crate::ast::AST;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() > 0 {
        for file in args {
            run_file(&file);
        }
    } else {
        repl();
    }
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path).unwrap();

    run(&contents)
}

fn repl() {
    let mut line = String::new();
    println!("Vitus {}", env!("CARGO_PKG_VERSION"));

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();
        if line.len() > 0 {
            run(&line)
        }

        line.clear();
    }
}

fn run(program: &str) {
    let tokens_result = Scanner::scan(program);

    match tokens_result {
        Ok(tokens) => {
            let mut parser = Parser::new(tokens);
            let ast_result = AST::new(&mut parser);

            match ast_result {
                Ok(ast) => {
                    let interpreter_result = Interpreter::interpret(ast);

                    match interpreter_result {
                        Ok(result) => println!("{}", result),
                        Err(e) => eprintln!("{:#?}", e),
                    }
                }
                Err(errors) => {
                    for error in errors {
                        eprintln!("{}", error);
                        std::process::exit(1);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}
