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
        run_prompt();
    }
}

fn run_file(path: &str) {
    let contents = fs::read_to_string(path).unwrap();

    run(&contents)
}

fn run_prompt() {
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
    let mut scanner = Scanner::new(program.to_owned(), &keywords);

    let tokens = scanner.scan_tokens();

    let ast = AST::new(&tokens);

    println!("{}", ast);
}
