// The evaluator.rs file

use std::collections::HashMap;

/*

I need a way to recontruct the original question, using struct doesnt fit it:

Let give the expression "3 + 5 * (2 - 8)" and evaluate it:

If ->

struct expression {
    numbers: Vec<f64>,
    operators: Vec<char>,
    functions: Vec<String>,
    variables: Vec<String>,
    left_parentheses: Vec<char>,
    right_parentheses: Vec<char>,
}

then -> when evaluating the expression, how do I know which number goes with which operator, function, variable or parenthesis?

Is it 3 + 5 * (2 - 8), 5 * (2 - 8 + 3) or (3 + 5) * (2 - 8)?

I need to keep the order of the tokens, so I can reconstruct the expression correctly.

Firt, i need a pure and simple enumeration to hold the tokens.

The value should hold the types directly, not a vector of types:

number: f64,
operator: char,
function: String,
variable: String,
left_parenthesis: char,
right_parenthesis: char,


After that, I can create an AST node struct that holds a vector of these tokens. Hence, I would evaluate (3 + 5 * (2 - 8)) by creating an AST node with the following tokens:

first token: number(3)
second token: operator('+')
third token: number(5)
fourth token: operator('*')
fifth token: left_parenthesis('(')
sixth token: number(2)
seventh token: operator('-')
eighth token: number(8)
ninth token: right_parenthesis(')')

On the Rust convention, I should use Pascal case for struct and enum names, and snake_case for variable and function names.
*/
enum ExpressionTokens {
    Number(f64),
    Operator(char),
    Function(String),
    Variable(String),
    LeftParenthesis,
    RightParenthesis,
}

/*
  To keep track of the precedence we need a struct that holds the structural relationships between the tokens to be more than a flat list: which only cares about what comes after the actual token.

  What we need is a tree-like structure that represents the hierarchy and precedence of operations.


  Example of flat list vs AST (3 + 5 * (2 - 8)):

  Flat list: [3, +, 5, *, (, 2, -, 8, )]

  AST:
          (+)
         /   \
       (3)   (*)
             /   \
           (5)   (-)
                 /   \
               (2)   (8)
  Reconstruction with the flat list would mean:

  1 -> 3 + 5 * (2 - 8) = -33
  2 -> 5 * (2 - 8 + 3) = -15
  3 -> (3 + 5) * (2 - 8) = -48

  With the AST, the structure inherently defines the order of operations and relationships between tokens, making it clear how to evaluate the expression correctly.

  Searching the desing approach for this maybe Pratt parsing:

  https://en.wikipedia.org/wiki/Pratt_parser


  Hence, we can define an AST node enum that represents different types of nodes in the AST.

  The definition will work as follows:

  1. Number node: Represents a numeric value.
  2. Operator node: Represents a binary operation (e.g., +, -, *, /) and holds references to its left and right child nodes.
  3. Function node: Represents a function call and holds a reference to its argument node.
  4. Variable node: Represents a variable and holds its name.

  The pseudocode for the AST node enum is as follows:

  enum -> represents the different types of AST nodes.
  ASTNode -> the name of the enum.
  Number(f64) -> a variant representing a numeric value.
     -> f64 is the type of the numeric value.
  Operator { operator: char, left: Box<ASTNode>, right: Box<ASTNode> } -> a variant representing a binary operation.
     -> operator: char is the operator character (e.g., '+', '-', '*', '/').
     -> left: Box<ASTNode> is a boxed reference to the left child node.
     -> right: Box<ASTNode> is a boxed reference to the right child node.
        -> Box is used to allocate the child nodes on the heap, allowing for recursive data structures.
  Function { name: String, argument: Box<ASTNode> } -> a variant representing a function call.
     -> name: String is the name of the function.
     -> argument: Box<ASTNode> is a boxed reference to the argument node.
  Variable(String) -> a variant representing a variable.
     -> String is the name of the variable.

  The step-by-step breakdown in time of execution of the AST node enum definition is as follow:
  1. Define the enum ASTNode to represent different types of nodes in the AST.
  2. Define the Number variant to represent numeric values.
  3. Define the Operator variant to represent binary operations, including fields for the operator character and references to left and right child nodes.
  4. Define the Function variant to represent function calls, including fields for the function name and a reference to the argument node.
  5. Define the Variable variant to represent variables, including a field for the variable name.
  6. Use Box to allocate child nodes on the heap, enabling recursive structures.
*/
enum ASTNode {
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

/*
  The context represents the 'What' and 'How' of the evaluation process.

  We need to define which variables hold what and using that in mind, assing a type to get back.

  Using hashmaps to hold variable names and their corresponding values.

  Hashmaps work like this:
   1 -> Put key-value pairs into the hashmap.
   1.1 -> It will store the variable name as the key and its value as the value.
   2 -> When evaluating an expression with variables, look up the variable name in the hashmap
   3 -> Return values using their keys.

*/
struct EvaluationContext {
    variables: HashMap<String, f64>,
    functions: HashMap<String, fn(Vec<f64>) -> f64>,
}

/*
  Here we should define what is the end result of the evaluation or better the direction we want to go with it.

  We have value: f64 -> the final evaluated result of the expression.
  steps: Vec<String> -> a vector of strings representing the steps taken during the evaluation process.
  errors: Vec<String> -> a vector of strings representing any errors encountered during the evaluation process
*/
struct DetailedEvaluationResult {
    value: Result<f64, EvaluationError>,
    steps: Vec<String>,
}


/*
  Here we define possible errors that can occur during the evaluation process:
   1 - DivisionByZero -> error when there is an attempt to divide by zero.
   2 - UndefinedVariable(String) -> error when a variable used in the expression is not defined in the context.
   3 - UndefinedFunction(String) -> error when a function used in the expression is not defined in the context.
   4 - SyntaxError(String) -> error when there is a syntax error in the expression being evaluated.
*/
enum EvaluationError {
    DivisionByZero,
    UndefinedVariable(String),
    UndefinedFunction(String),
    SyntaxError(String),
}


/*
* This is EvaluationContext implementation. The objective is to tell the evaluator the context
* necessary for the evaluation. If the expression has no need for context, the method would just
* pass as it is... while expressions with contexts like Variables or Functions, the methods of the
* EvaluationContext should be able to tell the evaluator the needed context. 
*
* &String vs &str:
*   For now: &String cannot be used for subtrings, i.e. they are slices, while &Strings always
*   references the whole thing (See: https://users.rust-lang.org/t/whats-the-difference-between-string-and-str/10177/2).
*
* */
impl EvaluationContext {
    fn new(variables: HashMap<String, f64>, functions: HashMap<String, fn(Vec<f64>) -> f64>) -> Self {      
        Self {variables, functions}
    }

    fn set_variable(&mut self, variable: String, value: f64) {
        self.variables.insert(variable, value);
    }

    fn set_function(&mut self, function: String, value: fn(Vec<f64>) -> f64) {
        self.functions.insert(function, value);
    }

    fn get_variable(&self, variable: &str) -> Option<f64>{
        self.variables.get(variable).copied()
    }
    fn get_function(&self, function: &str) -> Option<fn(Vec<f64>) -> f64> {
        self.functions.get(function).copied()
    }

}


impl DetailedEvaluationResult {
    fn ok(value: f64) -> Self {
        Self { value: Ok(value), steps: Vec::new() }
    }
    fn err(error: EvaluationError) -> Self {
        Self { value: Err(error), steps: Vec::new() }
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

fn evaluate(node: &ASTNode, context: &EvaluationContext) -> DetailedEvaluationResult {
    match node {
        ASTNode::Number(n) => {
            DetailedEvaluationResult::ok(*n)
        },
        ASTNode::Variable(name) => {
            if let Some(value) = context.get_variable(name) {
                DetailedEvaluationResult::ok(value)
            } else {
                DetailedEvaluationResult::err(EvaluationError::UndefinedVariable(name.clone()))
            }

        }
        _ => todo!()
    }
}
