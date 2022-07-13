mod ast;
mod environment;
mod function;
mod interpret;
mod parse;
mod scan;
mod token;

use interpret::interpret;
use parse::{parse_expresssion, parse_source};
use scan::scan;
use std::{
    env::args,
    io::{stdin, Write},
    process::exit,
};

fn lox_error(line: usize, text: &str) {
    println!("Error [line {}]: {}\n", line + 1, text);
}

fn interpret_source(mut text: String) -> Option<interpret::Value> {
    let tokens = scan(text.drain(..).collect::<Vec<char>>()).ok()?;
    let ast = parse_source(tokens)?;
    interpret(ast)
}

fn interpret_line(mut text: String) -> Option<interpret::Value> {
    let tokens = scan(text.drain(..).collect::<Vec<char>>()).ok()?;
    let ast = parse_expresssion(tokens)?;
    interpret(ast)
}

fn run_file(args: Vec<String>) {
    if args.len() != 2 {
        eprintln!("help: lox [script]");
        exit(1);
    }
    let file = &args[1];
    let text = std::fs::read_to_string(file).expect(&format!("Error: cant open file {}", file));
    interpret_source(text);
}

fn repl() {
    loop {
        print!("> ");
        std::io::stdout().flush().expect("failed to flush stdout");
        let mut line = String::new();
        let rsl = stdin()
            .read_line(&mut line)
            .expect("failed to read from stdin");
        if rsl == 0 {
            break;
        }
        if line.trim().len() > 0 {
            if let Some(output) = interpret_line(line) {
                println!("{}", output);
            }
        }
    }
}

fn main() {
    let args = args().collect::<Vec<String>>();
    if args.len() == 1 {
        repl();
    } else {
        run_file(args);
    }
}
