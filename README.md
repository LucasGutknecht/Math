# Math

## Core Structure setup
File: src/evaluator.rs

1 - First we define what tokens types we will have in our evaluator.

```rust
number: f64,
operator: char,
function: String,
variable: String,
left_parenthesis: char,
right_parenthesis: char,
```

number -> represents numerical values in the expression.
operator -> represents mathematical operators like +, -, *, /.
function -> represents mathematical functions like sin, cos, log.
variable -> represents variables that can hold values.
left_parenthesis -> represents the opening parenthesis '('.
right_parenthesis -> represents the closing parenthesis ')'.

2 - Next we define the structure for DetailedEvaluationResult which will hold the result of the evaluation, the steps taken during evaluation, and any errors encountered.

```rust
struct DetailedEvaluationResult {
    value: Result<f64, EvaluationError>,
    steps: Vec<String>,
}
```

value -> holds the final result of the evaluation or an error if one occurred, its either a floating-point number or an EvaluationError, the evaluation is terminated when an error occurs.
steps -> a vector of strings that records each step taken during the evaluation process for debugging or tracing.

3 - Finally we define the EvaluationError enum which lists possible errors that can occur during evaluation.

```rust
enum EvaluationError {
    DivisionByZero,
    UndefinedVariable(String),
    UndefinedFunction(String),
    SyntaxError(String),
}
```

DivisionByZero -> error when there is an attempt to divide by zero.
UndefinedVariable(String) -> error when a variable used in the expression is not defined in the context.
UndefinedFunction(String) -> error when a function used in the expression is not defined in the context.
SyntaxError(String) -> error when there is a syntax error in the expression being evaluated.

## AST Node Structure
File: src/evaluator.rs

Here we define the structure for ASTNode which represents nodes in the Abstract Syntax Tree (AST) used for parsing mathematical expressions.

It has the declaration os its variants:

1 - Number node which holds a floating-point number of type f64.
2 - Operator node which holds a character representing a mathematical operator, it holds on char for the operator and two boxed ASTNode for the left and right operands.
3 - Function node which holds a string representing the function name and a boxed ASTNode representing the function arguments.
4 - The Variable node which holds a string representing the variable.
5 - The UnaryOperator node which holds a character representing the unary operator and a boxed ASTNode representing the operand, unary like: -5, !X etc.

```rustenum ASTNode {
    Number(f64),
    Operator {
        operator: char,
        left: Box<ASTNode>,
        right: Box<ASTNode>,
    },
    Function {
        name: String,
        argument: Box<ASTNode>,
    },
    Variable(String),
    UnaryOperator {
        operator: char,
        operand: Box<ASTNode>,
    },
}

