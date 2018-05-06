use tokenizer::*;
use std::fmt;
use std::error;

#[derive(Debug, Clone, PartialEq)]
pub struct ParserError {
    pub message: String
}

impl fmt::Display for ParserError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for ParserError{
    fn description(&self) -> &str {
        &self.message
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Integer(u32),
    Expression(Box<Expr>, Symbol, Box<Expr>),
}

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser{ tokens: tokens }
    }

    pub fn parse(&self) -> Result<Expr, ParserError> {
        let mut waiting_stack: Vec<Expr> = Vec::new();
        let mut operator_stack: Vec<Symbol> = Vec::new();

        for i in 0..self.tokens.len() {
            let token = self.tokens[i];
            match (token, operator_stack.len()) {
                (Token::Operator(op), 0) => {
                    operator_stack.push(op);
                }
                (Token::Operator(op), _) => {
                    if op < operator_stack[operator_stack.len()-1] {
                        let right = waiting_stack.pop().unwrap();
                        let left = waiting_stack.pop().unwrap();
                        let higher_op = operator_stack.pop().unwrap();

                        waiting_stack.push(Expr::Expression(
                            Box::new(left),
                            higher_op,
                            Box::new(right)
                        ));
                    }
                    operator_stack.push(op);
                }
                (Token::Integer(num), _) => {
                    waiting_stack.push(Expr::Integer(num));
                },
            }
        }

        for _ in 0..operator_stack.len() {
            let op = operator_stack.pop().unwrap();
            let right = waiting_stack.pop().unwrap();
            let left = waiting_stack.pop().unwrap();

            waiting_stack.push(Expr::Expression(
                Box::new(left),
                op,
                Box::new(right)
            ));
        }
        waiting_stack.pop().ok_or(ParserError{ message: String::from("No expression left at end of parsing")})
    }
}

#[test]
fn test_parser() {
    let tokens = vec![
        Token::Integer(2), 
        Token::Operator(Symbol::Multiply), 
        Token::Integer(3), 
        Token::Operator(Symbol::Plus), 
        Token::Integer(5)
    ];
    let parser = Parser::new(tokens);
    let result = parser.parse();
    assert_eq!(result.is_ok(), true);
    assert_eq!(parser.parse().unwrap(),
        Expr::Expression(
            Box::new(Expr::Expression(
                Box::new(Expr::Integer(2)),
                Symbol::Multiply,
                Box::new(Expr::Integer(3)))),
            Symbol::Plus,
            Box::new(Expr::Integer(5)))
    );
}