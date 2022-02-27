mod ast;
mod scan;

use scan::scan;
use std::{env::args, process::exit};

fn lox_error(line: usize, text: &str) {
    println!("Error [line {}]: {}\n", line, text);
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("help: lox [script]");
        exit(1);
    }
    let file = &args[1];
    let text = std::fs::read_to_string(file).expect(&format!("Error: cant open file {}", file));
    let tokens = scan(text.chars().collect::<Vec<char>>());
    println!("Tokens={:#?}", tokens);
}
