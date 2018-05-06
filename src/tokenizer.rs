use std::fmt;
use std::error;
use std::cmp::PartialOrd;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Symbol {
    Plus,
    Minus,
    Multiply,
    Divide
}

impl PartialOrd for Symbol {
    fn partial_cmp(&self, other: &Symbol) -> Option<Ordering> {
        match (*self, *other) {
            (Symbol::Plus, Symbol::Plus) |
            (Symbol::Minus, Symbol::Minus) |
            (Symbol::Plus, Symbol::Minus) |
            (Symbol::Minus, Symbol::Plus) |
            (Symbol::Multiply, Symbol::Divide) |
            (Symbol::Divide, Symbol::Multiply) |
            (Symbol::Multiply, Symbol::Multiply) |
            (Symbol::Divide, Symbol::Divide) => {
                Some(Ordering::Equal)
            }
            (Symbol::Divide, Symbol::Minus) |
            (Symbol::Divide, Symbol::Plus) |
            (Symbol::Multiply, Symbol::Minus) |
            (Symbol::Multiply, Symbol::Plus) => {
                Some(Ordering::Greater)
            }
            (Symbol::Plus, Symbol::Multiply) |
            (Symbol::Minus, Symbol::Multiply) |
            (Symbol::Plus, Symbol::Divide) |
            (Symbol::Minus, Symbol::Divide) => {
                Some(Ordering::Less)
            }
        }
    }
}

impl Symbol {
    fn from_char(c: &char) -> Result<Self, TokenizerError> {
        match *c {
            '+' => Ok(Symbol::Plus),
            '-' => Ok(Symbol::Minus),
            '*' => Ok(Symbol::Multiply),
            '/' => Ok(Symbol::Divide),
            _ => Err(TokenizerError{
                message: String::from("Invalid char to symbol"),
                string: c.to_string()
            })
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
    Integer(u32),
    Operator(Symbol)
}


#[derive(Debug, Clone, PartialEq)]
pub struct TokenizerError {
    pub message: String,
    pub string: String
}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ({})", self.message, self.string)
    }
}

impl error::Error for TokenizerError {
    fn description(&self) -> &str {
        &self.message
    }
}

pub trait Tokenizer {
    fn tokenize(&self) -> Result<Vec<Token>, TokenizerError>;
}

impl Tokenizer for str {
    fn tokenize(&self) -> Result<Vec<Token>, TokenizerError> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut start = 0;
        let mut end = 0;
        for c in self.chars() { 
            match c {
                '0'...'9' => {
                    end += 1;
                }
                '+'|'*'|'/'|'-'  => {
                    parse_token(&self[start..end], &mut tokens)?;
                    tokens.push(Token::Operator(Symbol::from_char(&c)?));
                    start = end+1;
                    end = end+1;
                }
                _ => {
                    return Err(TokenizerError{
                        message: String::from("Invalid char to symbol"),
                        string: c.to_string()
                    })
                }
            }
        }
        parse_token(&self[start..end], &mut tokens)?;
        Ok(tokens)
    }
}

fn parse_token(token: &str, tokens: &mut Vec<Token>) -> Result<(), TokenizerError> {
    match token.parse::<u32>() {
        Ok(num) => {
            tokens.push(Token::Integer(num));
            Ok(())
        }
        Err(_) => {
            Err(TokenizerError {
                message: String::from("Error parsing u32"),
                string: token.clone().to_string()
            })
        }
    }
}

#[test]
fn test_baby_tokenizer() {
    let tests = vec![
        (String::from("12+34"), 
        Ok(vec![Token::Integer(12), Token::Operator(Symbol::Plus), Token::Integer(34)])),
        (String::from("12+34*56"), 
        Ok(vec![Token::Integer(12), Token::Operator(Symbol::Plus), Token::Integer(34), Token::Operator(Symbol::Multiply), Token::Integer(56)])),
        (String::from("asdf"),
            Err(TokenizerError{
                message: String::from("Invalid char to symbol"),
                string: String::from("a")
            })),
    ];

    for (input, solution) in tests {
        let result = input.tokenize();
        match (result, solution) {
            (Ok(tokens), Ok(solution_tokens)) => {
                assert_eq!(tokens.len(), solution_tokens.len());
                for (i, token) in tokens.iter().enumerate() {
                    assert_eq!(*token, solution_tokens[i]);
                }
            },
            (Err(tokenizer_error), Err(solution_error)) => {
                assert_eq!(tokenizer_error, solution_error);
            },
            (res, sol)  => {
                println!("{:?} is not {:?}", res, sol);
                assert!(false);
            }
        }
    }
}