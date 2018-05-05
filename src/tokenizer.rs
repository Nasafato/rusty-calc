use std::fmt::Error;

#[derive(Debug, PartialEq)]
pub enum Symbol {
    Plus,
    Minus,
    Multiply,
    Divide
}

impl Symbol {
    fn from_char(c: &char) -> Result<Self, Error> {
        match *c {
            '+' => Ok(Symbol::Plus),
            '-' => Ok(Symbol::Minus),
            '*' => Ok(Symbol::Multiply),
            '/' => Ok(Symbol::Divide),
            _ => Err(Error{})
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Integer(u32),
    Operator(Symbol)
}


pub trait Tokenizer {
    fn tokenize(&self) -> Result<Vec<Token>, Error>;
}

impl Tokenizer for str {
    fn tokenize(&self) -> Result<Vec<Token>, Error> {
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
                    return Err(Error{})
                }
            }
        }
        parse_token(&self[start..end], &mut tokens)?;
        Ok(tokens)
    }
}

fn parse_token(token: &str, tokens: &mut Vec<Token>) -> Result<(), Error> {
    match token.parse::<u32>() {
        Ok(num) => {
            tokens.push(Token::Integer(num));
            Ok(())
        }
        Err(_) => {
            Err(Error{})
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
        Err(Error{}))
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
            _  => {
                assert!(false);
            }
        }
    }
}