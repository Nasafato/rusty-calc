use std::io;
use std::io::prelude::*;

mod tokenizer;
mod evaluator;
mod parser;

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
                match handle_input(some_input) {
                    Ok(_) => {
                        println!("Token success!");
                    }
                    Err(error) => {
                        println!("Error: {}", error);
                        std::process::exit(0);
                    }
                }
            }
        }
        input.clear();
    }
}

fn handle_input(input: &str) -> Result<(), tokenizer::TokenizerError>{
    use tokenizer::Tokenizer;
    let tokens = match input.tokenize() {
        Ok(tokens) => tokens,
        Err(error) => {
            return Err(error);
        }
    };
    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}