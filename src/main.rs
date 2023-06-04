mod ast;
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
use crate::scanner::Scanner;
use crate::vitus::Vitus;

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
    let keywords = Vitus::keywords();

    let tokens = Scanner::scan(program, &keywords);

    println!("{:#?}", tokens);

    let ast = AST::new(&tokens);

    match ast {
        Ok(ast) => println!("{}", ast),
        Err(e) => println!("{:#?}", e),
    }
}
