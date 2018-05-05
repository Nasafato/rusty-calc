use std::io;
use std::io::prelude::*;

mod tokenizer;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();
        stdin.read_line(&mut input).unwrap();
        match input.trim() {
            "quit" => {
                std::process::exit(0);
            }
            some_input  => {
                handle_input(some_input);
            }
        }
        input.clear();
    }
}

fn handle_input(input: &str) {
    use tokenizer::Tokenizer;
    match input.tokenize() {
        Ok(tokens) => {
            for token in tokens {
                println!("{:?}", token);
            }
        },
        Err(error) => {
            println!("{}", error);
        }
    }
}