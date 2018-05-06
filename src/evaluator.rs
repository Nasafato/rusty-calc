use std::fmt;
use std::error;

use tokenizer::Symbol;
use parser::Expr;

#[derive(Debug)]
struct EvaluationError {
    message: String
}

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for EvaluationError {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

fn evaluate(expr: Expr) -> Result<u32, EvaluationError> {
    match expr {
        Expr::Expression(left, op, right) => {
            let left = evaluate(*left).unwrap();
            let right = evaluate(*right).unwrap();
            match op {
                Symbol::Plus => {
                    Ok(left + right)
                }
                Symbol::Minus=> {
                    Ok(left - right)
                }
                Symbol::Multiply => {
                    Ok(left * right)
                }
                Symbol::Divide => {
                    Err(EvaluationError{ message: format!("Divided {} by 0", left)})
                }
            }
        }
        Expr::Integer(num) => {
            Ok(num)
        }
    }
}

#[test]
fn test_evaluate() {
    let expr = 
        Expr::Expression(
            Box::new(Expr::Expression(
                Box::new(Expr::Integer(2)),
                Symbol::Multiply,
                Box::new(Expr::Integer(3)))),
            Symbol::Plus,
            Box::new(Expr::Integer(5)));

    let result = evaluate(expr).unwrap();
    assert_eq!(result, 11);
}

#[test]
fn test_divide_by_zero() {
    let expr = 
        Expr::Expression(
            Box::new(Expr::Integer(5)),
            Symbol::Divide,
            Box::new(Expr::Integer(0))
        );
    assert_eq!(evaluate(expr).is_err(), true);
}