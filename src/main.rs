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
                    Ok(_) => {}
                    Err(error) => {
                        println!("Error: {}", error);
                    }
                }
            }
        }
        input.clear();
    }
}

type GenError = Box<std::error::Error>;
type GenResult<T> = Result<T, GenError>;

fn handle_input(input: &str) -> GenResult<()> {
    use tokenizer::Tokenizer;
    use parser::Parser;
    use evaluator;

    let tokens = input.tokenize()?;
    let parser = Parser::new(tokens);
    let expr = parser.parse()?;
    let result = evaluator::evaluate(expr)?;
    println!("Result: {}", result);
    Ok(())
}