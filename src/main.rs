mod ast;
mod interpret;
mod parse;
mod scan;

use interpret::interpret;
use parse::parse;
use scan::scan;
use std::{
    env::args,
    io::{stdin, Write},
    process::exit,
};

fn lox_error(line: usize, text: &str) {
    println!("Error [line {}]: {}\n", line, text);
}

fn interpret_text(mut text: String) -> Option<interpret::Value> {
    let tokens = scan(text.drain(..).collect::<Vec<char>>()).ok()?;
    let ast = parse(tokens)?;
    interpret(ast)
}

#[allow(unused)]
fn interpret_file() {
    let args = args().collect::<Vec<String>>();
    if args.len() != 2 {
        eprintln!("help: lox [script]");
        exit(1);
    }
    let file = &args[1];
    let text = std::fs::read_to_string(file).expect(&format!("Error: cant open file {}", file));
    if let Some(output) = interpret_text(text) {
        println!("{}", output);
    }
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
            if let Some(output) = interpret_text(line) {
                println!("{}", output);
            }
        }
    }
}

fn main() {
    repl();
}
