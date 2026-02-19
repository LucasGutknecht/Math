mod evaluator;
mod lexer;
mod parser;

use evaluator::{EvaluationContext, evaluate};
use lexer::Lexer;
use parser::Parser;
use std::collections::HashMap;

fn main() {
    let expression: String = "(3 + (5 - (3 * sqrt(16)))) * 2".to_string();
    let mut lexer = Lexer::new(expression);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();
    match ast {
        Some(ast) => {
            let context = EvaluationContext::new(HashMap::new(), HashMap::new());
            let result = evaluate(&ast, &context);
            println!("Steps: {:?}", result.steps);
            println!("Result: {:?}", result.value);
        }
        None => {
            println!("Parse error")
        }
    }
}
