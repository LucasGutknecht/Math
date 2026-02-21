/* The parser file*/

use crate::evaluator::{ASTNode, ExpressionTokens};

#[allow(dead_code)]
struct BindPower {
    operator: char,
    lbp: i32,
    rbp: i32,
}

pub struct Parser {
    position: usize,
    tokens: Vec<ExpressionTokens>,
}

impl BindPower {
    fn get_bind_power(ch: char) -> Option<(i32, i32)> {
        match ch {
            '+' | '-' => Some((10, 9)),
            '*' | '/' => Some((20, 19)),
            _ => None,
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<ExpressionTokens>) -> Self {
        Parser {
            position: 0,
            tokens,
        }
    }

    fn peek(&self) -> Option<&ExpressionTokens> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn parse_expression(&mut self, min_bp: i32) -> Option<ASTNode> {
        let mut left = match self.peek().cloned() {
            Some(ExpressionTokens::Number(token)) => {
                self.advance();
                ASTNode::Number(token)
            }
            Some(ExpressionTokens::Variable(name)) => {
                self.advance();
                if let Some(ExpressionTokens::LeftParenthesis) = self.peek().cloned() {
                    self.advance();
                    let argument = self.parse_expression(0)?;
                    match self.peek().cloned() {
                        Some(ExpressionTokens::RightParenthesis) => self.advance(),
                        _ => return None,
                    }
                    ASTNode::Function {
                        name,
                        argument: Box::new(argument),
                    }
                } else {
                    ASTNode::Variable(name)
                }
            }
            Some(ExpressionTokens::LeftParenthesis) => {
                self.advance();
                let inner = self.parse_expression(0)?;
                match self.peek().cloned() {
                    Some(ExpressionTokens::RightParenthesis) => self.advance(),
                    _ => return None,
                }
                inner
            }
            _ => return None,
        };

        loop {
            let token = self.peek().cloned();
            match token {
                Some(ExpressionTokens::Operator(token)) => {
                    let (lbp, rbp) = match BindPower::get_bind_power(token) {
                        Some(bp) => bp,
                        None => break,
                    };
                    if lbp <= min_bp {
                        break;
                    }

                    let op = token;
                    self.advance();

                    let right = self.parse_expression(rbp)?;

                    left = ASTNode::Operator {
                        operator: op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }
        Some(left)
    }

    pub fn parse(&mut self) -> Option<ASTNode> {
        self.parse_expression(0)
    }
}
