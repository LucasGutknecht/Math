// The evaluator.rs file

use std::collections::HashMap;

#[derive(Clone)]
#[allow(dead_code)]
pub enum ExpressionTokens {
    Number(f64),
    Operator(char),
    Function(String),
    Variable(String),
    LeftParenthesis,
    RightParenthesis,
}

#[allow(dead_code)]
pub enum ASTNode {
    // Leaf node representing a number
    Number(f64),

    // Node representing a binary operation
    Operator {
        operator: char,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    // Node representing a function call
    Function {
        name: String,
        argument: Box<ASTNode>,
    },
    // Node representing a variable
    Variable(String),
    // Node representing unary operation
    UnaryOperator {
        operator: char,
        operand: Box<ASTNode>,
    },
}

pub struct EvaluationContext {
    variables: HashMap<String, f64>,
    functions: HashMap<String, fn(Vec<f64>) -> f64>,
}

pub struct DetailedEvaluationResult {
    pub value: Result<f64, EvaluationError>,
    pub steps: Vec<String>,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum EvaluationError {
    DivisionByZero,
    UndefinedVariable(String),
    UndefinedFunction(String),
    SyntaxError(String),
}

#[allow(dead_code)]
impl EvaluationContext {
    pub fn new(
        variables: HashMap<String, f64>,
        functions: HashMap<String, fn(Vec<f64>) -> f64>,
    ) -> Self {
        Self {
            variables,
            functions,
        }
    }

    fn set_variable(&mut self, variable: String, value: f64) {
        self.variables.insert(variable, value);
    }

    pub fn set_function(&mut self, function: String, value: fn(Vec<f64>) -> f64) {
        self.functions.insert(function, value);
    }

    fn get_variable(&self, variable: &str) -> Option<f64> {
        self.variables.get(variable).copied()
    }
    fn get_function(&self, function: &str) -> Option<fn(Vec<f64>) -> f64> {
        self.functions.get(function).copied()
    }
}

impl DetailedEvaluationResult {
    pub fn ok(value: f64) -> Self {
        Self {
            value: Ok(value),
            steps: Vec::new(),
        }
    }
    fn err(error: EvaluationError) -> Self {
        Self {
            value: Err(error),
            steps: Vec::new(),
        }
    }
    fn with_step(mut self, step: String) -> Self {
        self.steps.push(step);
        self
    }
    fn with_steps(mut self, steps: Vec<String>) -> Self {
        self.steps.extend(steps);
        self
    }
}

pub fn evaluate(node: &ASTNode, context: &EvaluationContext) -> DetailedEvaluationResult {
    match node {
        ASTNode::Number(n) => DetailedEvaluationResult::ok(*n),
        ASTNode::Variable(name) => {
            if let Some(value) = context.get_variable(name) {
                DetailedEvaluationResult::ok(value)
            } else {
                DetailedEvaluationResult::err(EvaluationError::UndefinedVariable(name.clone()))
            }
        }
        ASTNode::Operator {
            operator,
            left,
            right,
        } => {
            let left_result = evaluate(left, context);

            match left_result.value {
                Ok(left_val) => {
                    let right_result = evaluate(right, context);
                    match right_result.value {
                        Ok(right_val) => match operator {
                            '+' => {
                                let result = left_val + right_val;
                                let step: String =
                                    format!("{} + {} = {}", left_val, right_val, result);
                                DetailedEvaluationResult::ok(result)
                                    .with_steps(left_result.steps)
                                    .with_steps(right_result.steps)
                                    .with_step(step)
                            }
                            '-' => {
                                let result = left_val - right_val;
                                let step: String =
                                    format!("{} - {} = {}", left_val, right_val, result);
                                DetailedEvaluationResult::ok(result)
                                    .with_steps(left_result.steps)
                                    .with_steps(right_result.steps)
                                    .with_step(step)
                            }
                            '*' => {
                                let result = left_val * right_val;
                                let step: String =
                                    format!("{} * {} = {}", left_val, right_val, result);
                                DetailedEvaluationResult::ok(result)
                                    .with_steps(left_result.steps)
                                    .with_steps(right_result.steps)
                                    .with_step(step)
                            }
                            '/' => {
                                if right_val == 0.0 {
                                    DetailedEvaluationResult::err(EvaluationError::DivisionByZero)
                                } else {
                                    let result = left_val / right_val;
                                    let step: String =
                                        format!("{} / {} = {}", left_val, right_val, result);
                                    DetailedEvaluationResult::ok(result)
                                        .with_steps(left_result.steps)
                                        .with_steps(right_result.steps)
                                        .with_step(step)
                                }
                            }
                            _ => DetailedEvaluationResult::err(EvaluationError::SyntaxError(
                                "Unknown operator".to_string(),
                            )),
                        },
                        Err(e) => DetailedEvaluationResult::err(e).with_steps(right_result.steps),
                    }
                }
                Err(e) => DetailedEvaluationResult::err(e).with_steps(left_result.steps),
            }
        }
        ASTNode::Function { name, argument } => {
            let arg_result = evaluate(argument, context);
            match arg_result.value {
                Ok(arg_val) => {
                    if let Some(func) = context.get_function(name) {
                        let result = func(vec![arg_val]);
                        let step = format!("{}({}) = {}", name, arg_val, result);
                        DetailedEvaluationResult::ok(result)
                            .with_steps(arg_result.steps)
                            .with_step(step)
                    } else {
                        DetailedEvaluationResult::err(EvaluationError::UndefinedFunction(
                            name.clone(),
                        ))
                    }
                }
                Err(e) => DetailedEvaluationResult::err(e).with_steps(arg_result.steps),
            }
        }

        ASTNode::UnaryOperator { operator, operand } => {
            let operand_result = evaluate(operand, context);
            match operand_result.value {
                Ok(operand_val) => match operator {
                    '-' => {
                        let result = -operand_val;
                        let step = format!("-{} = {}", operand_val, result);
                        DetailedEvaluationResult::ok(result)
                            .with_steps(operand_result.steps)
                            .with_step(step)
                    }
                    '+' => {
                        DetailedEvaluationResult::ok(operand_val).with_steps(operand_result.steps)
                    }
                    _ => DetailedEvaluationResult::err(EvaluationError::SyntaxError(
                        "Unknown operator".to_string(),
                    )),
                },
                Err(e) => DetailedEvaluationResult::err(e).with_steps(operand_result.steps),
            }
        }
    }
}
