# Math

A Mathematical Expression Evaluator in Rust

---

## Introduction

### What is Math?

Math is a mathematical expression evaluator written in Rust. It takes a string representing a mathematical expression, parses it, and evaluates it to produce a numerical result.

Given an input like:

$$3 + 5 \times (2 - 8)$$

The tool will:

$1$ $\rightarrow$ Break the string into tokens (numbers, operators, parentheses).

$2$ $\rightarrow$ Build a tree structure that respects operator precedence.

$3$ $\rightarrow$ Walk the tree to compute the final result.

$4$ $\rightarrow$ Return the result along with a trace of evaluation steps.

---

## 1. Motivation: Why a Flat Struct Doesn't Work

File: src/evaluator.rs

First, before anything... we need a way to reconstruct the original expression when evaluating. There are some ways to achieve that; firstly we could lay out what is not fit:

Using a struct wouldn't achieve that, the reason being:

- Let's get the following expression $3 + 5 \times (2 - 8)$ and evaluate it:

If we have a struct like this:

```rust
struct Expression {
    numbers: Vec<f64>,
    operators: Vec<char>,
    functions: Vec<String>,
    variables: Vec<String>,
    left_parentheses: Vec<char>,
    right_parentheses: Vec<char>,
}
```

Then, when evaluating the expression, how would we know which number goes with which operator, function, variable or parenthesis?

Which is it, when reconstructing?

- a) $3 + 5 \times (2 - 8)$
- b) $5 \times (2 - 8 + 3)$
- c) $(3 + 5) \times (2 - 8)$?

Looking back on the aforementioned option, it is clear that we need to keep the order of the tokens in order to reconstruct the expression correctly.

---

## 2. Token Types: ExpressionTokens

File: src/evaluator.rs

First, we need a pure and simple enumeration to hold the tokens.

The value should hold the types directly, not a vector of types. The enum is called `ExpressionTokens` and derives `Clone` so tokens can be duplicated when needed:

```
Number(f64)           -- holds the numeric value directly.
Operator(char)        -- holds the operator character directly.
Function(String)      -- holds the function name directly.
Variable(String)      -- holds the variable name directly.
LeftParenthesis       -- represents '(', no value needed.
RightParenthesis      -- represents ')', no value needed.
```

- $\text{Number}(f64)$ $\rightarrow$ represents numerical values in the expression.
- $\text{Operator}(\text{char})$ $\rightarrow$ represents mathematical operators like $+, -, \times, \div$.
- $\text{Function}(\text{String})$ $\rightarrow$ represents mathematical functions like $\sin, \cos, \log$.
- $\text{Variable}(\text{String})$ $\rightarrow$ represents variables that can hold values.
- $\text{LeftParenthesis}$ $\rightarrow$ represents the opening parenthesis $($.
- $\text{RightParenthesis}$ $\rightarrow$ represents the closing parenthesis $)$.

Note that `LeftParenthesis` and `RightParenthesis` are unit variants -- they don't hold a value because there is only one kind of each.

OBS: On the Rust convention, we should use PascalCase for struct and enum names, and snake_case for variable and function names.

### Token Example

After that, we can create an AST node struct that holds a vector of these tokens. Hence, we would evaluate $3 + 5 \times (2 - 8)$ by creating an AST node with the following tokens:

|Order|Token|
|---|---|
|first|$\text{Number}(3)$|
|second|$\text{Operator}('+')$|
|third|$\text{Number}(5)$|
|fourth|$\text{Operator}('*')$|
|fifth|$\text{LeftParenthesis}$|
|sixth|$\text{Number}(2)$|
|seventh|$\text{Operator}('-')$|
|eighth|$\text{Number}(8)$|
|ninth|$\text{RightParenthesis}$|

### Implementation

```rust
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
```

---

## 3. From Flat List to AST

File: src/evaluator.rs

To keep track of the precedence we need a struct that holds the structural relationships between the tokens to be more than a flat list, which only cares about what comes after the actual token.

What we need is a tree-like structure that represents the hierarchy and precedence of operations.

### Flat List vs AST

Example for $3 + 5 \times (2 - 8)$:

**Flat list:** $[3, +, 5, \times, (, 2, -, 8, )]$

**AST:**

```
        (+)
       /   \
     (3)   (*)
           /   \
         (5)   (-)
               /   \
             (2)   (8)
```

### The Ambiguity Problem

Reconstruction with the flat list would mean:

$$ \begin{aligned} 1 &\to 3 + 5 \times (2 - 8) = 3 + 5 \times (-6) = 3 + (-30) = -27 \ 2 &\to 5 \times (2 - 8 + 3) = 5 \times (-3) = -15 \ 3 &\to (3 + 5) \times (2 - 8) = 8 \times (-6) = -48 \end{aligned} $$

With the AST, the structure inherently defines the order of operations and relationships between tokens, making it clear how to evaluate the expression correctly.

### Design Approach

Searching the design approach for this, maybe Pratt parsing:

https://en.wikipedia.org/wiki/Pratt_parser

---

## 4. AST Node Structure: ASTNode

File: src/evaluator.rs

Hence, we can define an AST node enum that represents different types of nodes in the AST.

### Variant Overview

Here we define the structure for $\text{ASTNode}$ which represents nodes in the Abstract Syntax Tree (AST) used for parsing mathematical expressions.

It has the declaration of its variants:

1 - **Number node** -- a leaf node which holds a floating-point number of type $f64$, representing $n \in \mathbb{R}$.

2 - **Operator node** -- holds a character representing a mathematical operator; it holds one char for the operator and two boxed ASTNodes for the left and right operands, representing a binary expression $\text{left} ; \text{op} ; \text{right}$.

3 - **Function node** -- holds a string representing the function name and a boxed ASTNode representing the function argument, representing $f(x)$.

4 - **Variable node** -- holds a string representing the variable $x$.

5 - **UnaryOperator node** -- holds a character representing the unary operator and a boxed ASTNode representing the operand, unary like: $-5$, $+x$ etc.

### Pseudocode

The pseudocode for the AST node enum is as follows:

$\text{enum}$ $\rightarrow$ represents the different types of AST nodes. $\text{ASTNode}$ $\rightarrow$ the name of the enum.

---

**$\text{Number}(f64)$** $\rightarrow$ a leaf variant representing a numeric value.

|Field|Type|Description|
|---|---|---|
|value|$f64$|the type of the numeric value.|

---

**$\text{Operator} \lbrace \text{operator},\ \text{left},\ \text{right} \rbrace$** $\rightarrow$ a variant representing a binary operation.

|Field|Type|Description|
|---|---|---|
|operator|$\text{char}$|the operator character (e.g., $'+', '-', '*', '/'$).|
|left|$\text{Box}\langle\text{ASTNode}\rangle$|a boxed reference to the left child node.|
|right|$\text{Box}\langle\text{ASTNode}\rangle$|a boxed reference to the right child node.|

> $\text{Box}$ is used to allocate the child nodes on the heap, allowing for recursive data structures.

---

**$\text{Function} \lbrace \text{name},\ \text{argument} \rbrace$** $\rightarrow$ a variant representing a function call.

|Field|Type|Description|
|---|---|---|
|name|$\text{String}$|the name of the function.|
|argument|$\text{Box}\langle\text{ASTNode}\rangle$|a boxed reference to the argument node.|

---

**$\text{Variable}(\text{String})$** $\rightarrow$ a variant representing a variable.

|Field|Type|Description|
|---|---|---|
|value|$\text{String}$|the name of the variable.|

---

**$\text{UnaryOperator} \lbrace \text{operator},\ \text{operand} \rbrace$** $\rightarrow$ a variant representing a unary operation.

|Field|Type|Description|
|---|---|---|
|operator|$\text{char}$|the unary operator character.|
|operand|$\text{Box}\langle\text{ASTNode}\rangle$|a boxed reference to the operand node.|

### Step-by-Step Breakdown

The step-by-step breakdown in time of execution of the AST node enum definition is as follows:

1. Define the enum $\text{ASTNode}$ to represent different types of nodes in the AST.
2. Define the $\text{Number}$ variant to represent numeric values.
3. Define the $\text{Operator}$ variant to represent binary operations, including fields for the operator character and references to left and right child nodes.
4. Define the $\text{Function}$ variant to represent function calls, including fields for the function name and a reference to the argument node.
5. Define the $\text{Variable}$ variant to represent variables, including a field for the variable name.
6. Define the $\text{UnaryOperator}$ variant to represent unary operations, including a field for the operator character and a reference to the operand node.
7. Use $\text{Box}$ to allocate child nodes on the heap, enabling recursive structures.

### Implementation

```rust
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
```

---

## 5. Evaluation Context: EvaluationContext

File: src/evaluator.rs

This is the EvaluationContext implementation. The objective is to tell the evaluator the context necessary for the evaluation. If the expression has no need for context, the method would just pass as it is... while for expressions with context like Variables or Functions, the methods of the EvaluationContext should be able to tell the evaluator the needed context.

### What and How

The context represents the 'What' and 'How' of the evaluation process.

we need to define which variables hold what and, using that in mind, assign a type to get back.

### Hashmaps

Using hashmaps to hold variable names and their corresponding values.

The context holds two hashmaps:

- $\text{variables}: \text{HashMap}\langle\text{String},\ f64\rangle$ $\rightarrow$ maps variable names to their numeric values.
- $\text{functions}: \text{HashMap}\langle\text{String},\ \text{fn}(\text{Vec}\langle f64\rangle) \to f64\rangle$ $\rightarrow$ maps function names to their implementations, where each function takes a vector of $f64$ arguments and returns a single $f64$.

Hashmaps work like this:

$1$ $\rightarrow$ Put key-value pairs into the hashmap: $\text{key} \mapsto \text{value}$.

$\quad 1.1$ $\rightarrow$ It will store the variable name as the key and its value as the value.

$2$ $\rightarrow$ When evaluating an expression with variables, look up the variable name in the hashmap.

$3$ $\rightarrow$ Return values using their keys.

### Methods

The EvaluationContext exposes the following methods:

|Method|Signature|Description|
|---|---|---|
|`new`|`(variables, functions) -> Self`|creates a new context with the given variables and functions.|
|`set_variable`|`(&mut self, String, f64)`|inserts or updates a variable in the context.|
|`set_function`|`(&mut self, String, fn(Vec<f64>) -> f64)`|inserts or updates a function in the context.|
|`get_variable`|`(&self, &str) -> Option<f64>`|looks up a variable by name, returns its value if found.|
|`get_function`|`(&self, &str) -> Option<fn(Vec<f64>) -> f64>`|looks up a function by name, returns its implementation if found.|

> **&String vs &str:** For now: &str can be used for substrings, i.e. they are slices, while &String always references the whole thing (See: https://users.rust-lang.org/t/whats-the-difference-between-string-and-str/10177/2). That's why the getter methods take `&str` -- they can accept both `&String` and `&str`.

### Implementation

```rust
pub struct EvaluationContext {
    variables: HashMap<String, f64>,
    functions: HashMap<String, fn(Vec<f64>) -> f64>,
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
```

---

## 6. Evaluation Result: DetailedEvaluationResult

File: src/evaluator.rs

Here we should define what the end result of the evaluation is, or better, the direction we want to go with it.

### Fields

we have:

- $\text{value}: \text{Result}\langle f64,\ \text{EvaluationError}\rangle$ $\rightarrow$ the final evaluated result of the expression, or an error if one occurred. The evaluation is terminated when an error occurs.
- $\text{steps}: \text{Vec}\langle\text{String}\rangle$ $\rightarrow$ a vector of strings representing the steps taken during the evaluation process, for debugging or tracing.

### Methods

The DetailedEvaluationResult exposes the following methods:

|Method|Signature|Description|
|---|---|---|
|`ok`|`(f64) -> Self`|creates a successful result with the given value and empty steps.|
|`err`|`(EvaluationError) -> Self`|creates a failed result with the given error and empty steps.|
|`with_step`|`(mut self, String) -> Self`|appends a single step to the steps vector and returns self.|
|`with_steps`|`(mut self, Vec<String>) -> Self`|extends the steps vector with multiple steps and returns self.|

The methods `with_step` and `with_steps` use a builder pattern -- they take ownership of `self`, modify it, and return it, allowing method chaining like:

```rust
DetailedEvaluationResult::ok(result)
    .with_steps(left_result.steps)
    .with_steps(right_result.steps)
    .with_step(step)
```

### Implementation

```rust
pub struct DetailedEvaluationResult {
    pub value: Result<f64, EvaluationError>,
    pub steps: Vec<String>,
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
```

---

## 7. Evaluation Errors: EvaluationError

File: src/evaluator.rs

Here we define possible errors that can occur during the evaluation process:

$1$ $\rightarrow$ $\text{DivisionByZero}$ -- error when there is an attempt to divide by zero ($x \div 0$).

$2$ $\rightarrow$ $\text{UndefinedVariable}(\text{String})$ -- error when a variable used in the expression is not defined in the context.

$3$ $\rightarrow$ $\text{UndefinedFunction}(\text{String})$ -- error when a function used in the expression is not defined in the context.

$4$ $\rightarrow$ $\text{SyntaxError}(\text{String})$ -- error when there is a syntax error in the expression being evaluated.

The enum derives `Debug` for error reporting.

### Implementation

```rust
#[derive(Debug)]
#[allow(dead_code)]
pub enum EvaluationError {
    DivisionByZero,
    UndefinedVariable(String),
    UndefinedFunction(String),
    SyntaxError(String),
}
```

---

## 8. The Evaluate Function

File: src/evaluator.rs

The `evaluate` function is the core of the evaluator. It takes a reference to an $\text{ASTNode}$ and a reference to an $\text{EvaluationContext}$, then recursively walks the tree to produce a $\text{DetailedEvaluationResult}$.

### How It Works

The function pattern-matches on the node variant:

$1$ $\rightarrow$ **Number** -- returns the value directly via `DetailedEvaluationResult::ok`.

$2$ $\rightarrow$ **Variable** -- looks up the variable name in the context. If found, returns the value. If not, returns an `UndefinedVariable` error.

$3$ $\rightarrow$ **Operator** -- recursively evaluates the left and right children. If both succeed, applies the operator ($+, -, \times, \div$) and records the step. Division by zero is caught and returns a `DivisionByZero` error. Unknown operators return a `SyntaxError`.

$4$ $\rightarrow$ **Function** -- recursively evaluates the argument, then looks up the function in the context. If found, calls it with the evaluated argument wrapped in a vector and records the step. If not found, returns an `UndefinedFunction` error.

$5$ $\rightarrow$ **UnaryOperator** -- recursively evaluates the operand, then applies the unary operator. Supports $-$ (negation) and $+$ (identity). Unknown unary operators return a `SyntaxError`.

### Step Accumulation

Each branch collects the steps from its children via `with_steps` before appending its own step via `with_step`. This means the final `DetailedEvaluationResult` contains the full trace of the evaluation in order, from the deepest leaves up to the root.

### Signature

```rust
pub fn evaluate(node: &ASTNode, context: &EvaluationContext) -> DetailedEvaluationResult
```

### Implementation

```rust
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
                                let step = format!("{} + {} = {}", left_val, right_val, result);
                                DetailedEvaluationResult::ok(result)
                                    .with_steps(left_result.steps)
                                    .with_steps(right_result.steps)
                                    .with_step(step)
                            }
                            '-' => {
                                let result = left_val - right_val;
                                let step = format!("{} - {} = {}", left_val, right_val, result);
                                DetailedEvaluationResult::ok(result)
                                    .with_steps(left_result.steps)
                                    .with_steps(right_result.steps)
                                    .with_step(step)
                            }
                            '*' => {
                                let result = left_val * right_val;
                                let step = format!("{} * {} = {}", left_val, right_val, result);
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
                                    let step =
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
                        DetailedEvaluationResult::ok(operand_val)
                            .with_steps(operand_result.steps)
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
```

## 9. The Lexer

File: src/lexer.rs

The lexer's job is to take the raw input string and convert it into a series of tokens that can be easily processed by the parser.

Each token represents a meaningful unit in the mathematical expression, such as numbers, operators, parentheses, functions, and variables.

### Struct Fields

|Field|Type|Description|
|---|---|---|
|position|$\text{usize}$|the current position in the input string that the lexer is processing.|
|input|$\text{String}$|the raw string input that the lexer will process.|

### Why Not Use `current_char` and `token` as Fields?

- **current_char:** Given that the lexer's job is to create tokens, there is no need to compute the next token at the exact moment we are processing the actual token. The future computation should be done in the future.
- **token:** The lexer should not hold the current token as a field, because the lexer is responsible for generating tokens from the input string, not storing them. The tokens should be generated on-the-fly as needed.

### Lexer Process Example

Example of how the lexer would see the expression $(3 + 5 \times (2 - 8))$:

|Position|Character|Action|
|---|---|---|
|0|`(`|token|
|1|`3`|token|
|2||whitespace, skip|
|3|`+`|token|
|4||whitespace, skip|
|5|`5`|token|
|6||whitespace, skip|
|7|`*`|token|
|8||whitespace, skip|
|9|`(`|token|
|10|`2`|token|
|11||whitespace, skip|
|12|`-`|token|
|13||whitespace, skip|
|14|`8`|token|
|15|`)`|token|
|16|`)`|token|

---

## 10. Lexer Implementation Rules

File: src/lexer.rs

How do we implement the lexer? What underlying rules should we follow? What logic to implement?

### Step-by-Step Rules

$1$ $\rightarrow$ **Define tokens and non-tokens** (whitespaces, invalid characters, etc.). The data here are raw strings and positions.

Example: `"3 + 5 * (2 - 8)"` $\to$ tokens: $[3, +, 5, \times, (, 2, -, 8, )]$

$2$ $\rightarrow$ **Define patterns** to be matched for each token type (numbers, operators, functions, variables, parentheses). The data here are patterns and identifiers.

Example: `number_pattern = r"\d+(\.\d+)?"`

$3$ $\rightarrow$ **Implement logic** to iterate on which pattern to apply and when. The data here are rules and conditions.

Example: if `current_char` is digit $\to$ match `number_pattern`

$4$ $\rightarrow$ **Create tokens** based on the matched patterns and store them in a structured format (like an enum or struct). The data here are tokens and types.

Example: `Token::Number(3.0)`, `Token::Operator('+')`

$5$ $\rightarrow$ **Handle errors** for invalid characters or sequences. The data here are error types and messages.

Example: `Error::InvalidCharacter('@')`

$6$ $\rightarrow$ **Return the list of tokens** for further processing by the parser. The data here are token lists and structures.

Example: `vec![Token::Number(3.0), Token::Operator('+'), Token::Number(5.0), ...]`

---

## 11. Lexer Methods

File: src/lexer.rs

### Method Overview

|Method|Signature|Description|
|---|---|---|
|`new`|`(String) -> Self`|initializes the lexer with the input string at position 0.|
|`peek`|`(&self) -> Option<char>`|returns the current character without advancing the position.|
|`advance`|`(&mut self)`|advances the position to the next character.|
|`skip_whitespace`|`(&mut self)`|skips all whitespace characters from current position.|
|`read_number`|`(&mut self) -> f64`|reads consecutive digits and dots, parses them as $f64$.|
|`read_identifier`|`(&mut self) -> String`|reads consecutive alphanumeric characters and underscores.|
|`next_token`|`(&mut self) -> Option<ExpressionTokens>`|returns the next token, or `None` if input is exhausted.|
|`tokenize`|`(&mut self) -> Vec<ExpressionTokens>`|consumes all input and returns a vector of tokens.|

### Method: `skip_whitespace`

The idea here is to run `skip_whitespace`, which gives us a notion of whether the current character is a whitespace or not. It does that by generating a loop using `while`, where we take the `peek` results and map them using the `map_or` method.

The `map_or` method works like this:

```
map_or<U, F>(default_value, closure)
```

- If the Option is `None`, return the default value.
- If the Option is `Some(x)`, apply the closure to `x` and return the result.

Example:

```rust
let x = Some("foo");
assert_eq!(x.map_or(42, |v| v.len()), 3);

let x: Option<&str> = None;
assert_eq!(x.map_or(42, |v| v.len()), 42);
```

### Method: `read_number`

The idea here is to run `read_number` to check if the char is a number or dot, and return `false` if not in the while block.

Given that we cannot access the context of the `ch` from `map_or` (since it only exists there), we may access the `ch` from the `peek` method itself. If it exists, it pushes the `ch` into the string, appending the value.

After that, advance until the end of the while loop.

The method `read_number` expects a return of the type $f64$. To achieve this, we must use the `parse` method of the `String` type:

```rust
the_string.parse::<f64>().unwrap()
```

- `::<>` $\rightarrow$ this syntax is called **turbofish**; it indicates directly to the program the type you want to parse into.
- `unwrap()` $\rightarrow$ this method helps us retrieve the parsed value directly (`Some`) and returns a panic if the result is `None`.

### Method: `next_token`

The function `next_token` is expected to return the next token of the lexer.

The function returns a type `ExpressionTokens` encapsulated by `Option`. The `Option` is an enum used to handle the absence of a value, eliminating null pointer errors:

$$\text{Option}\langle T \rangle \to T = \text{Types}$$

Returns:

- $\text{Some}(T)$ $\rightarrow$ whichever variant that takes the value of type $T$.
- $\text{None}$ $\rightarrow$ whichever variant that takes the absence of value; `None` can be assigned directly as `None` (without the `;` at the end).

The function takes as param `&mut self`:

- `self` $\rightarrow$ the method takes ownership of the instance.
- `&self` $\rightarrow$ the method borrows the instance immutably.
- `&mut self` $\rightarrow$ the method borrows the instance mutably.

The function operates in the following steps:

$1$ $\rightarrow$ Assert that the actual char is not a whitespace by calling `skip_whitespace`.

$2$ $\rightarrow$ Put the current char into a variable; using `?` will assert that if the value is not what we expected, return `None`.

$3$ $\rightarrow$ Match the char and execute the corresponding method:

- If a digit from `'0'` to `'9'`, call `self.read_number()` which transforms the String into a number of type $f64$, encapsulated into a `Number` type established on the `ExpressionTokens` enum, wrapped in `Some()`.
- The `_` option means everything else, returning `None`.

### Method: `tokenize`

The purpose of the function `tokenize` is to create a vector of the type `ExpressionTokens`.

The function operates like:

$1$ $\rightarrow$ Declare a new `Vec`.

$2$ $\rightarrow$ Initialize a loop with `while let Some(token) = self.next_token()`:

- If `Some`: push the token to the `Vec`.
- `push`: Appends an element to the back of a collection.

$3$ $\rightarrow$ At the end, return a vec of tokens like: `['(', '3', '+', '8', ')', '+', '5']`

### Implementation

```rust
pub struct Lexer {
    position: usize,
    input: String,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer { position: 0, input }
    }

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, |ch| ch.is_whitespace()) {
            self.advance();
        }
    }

    fn read_number(&mut self) -> f64 {
        let mut valores = String::new();

        while self
            .peek()
            .map_or(false, |ch| ch.is_ascii_digit() || ch == '.')
        {
            if let Some(ch) = self.peek() {
                valores.push(ch);
            }
            self.advance()
        }

        valores.parse::<f64>().unwrap()
    }

    fn read_identifier(&mut self) -> String {
        let mut identificador = String::new();

        while self
            .peek()
            .map_or(false, |ch| ch.is_alphanumeric() || ch == '_')
        {
            if let Some(ch) = self.peek() {
                identificador.push(ch);
            }
            self.advance();
        }

        identificador
    }

    fn next_token(&mut self) -> Option<ExpressionTokens> {
        self.skip_whitespace();

        let ch = self.peek()?;

        match ch {
            '0'..='9' => Some(ExpressionTokens::Number(self.read_number())),
            '(' => {
                self.advance();
                Some(ExpressionTokens::LeftParenthesis)
            }
            ')' => {
                self.advance();
                Some(ExpressionTokens::RightParenthesis)
            }
            '+' | '-' | '*' | '/' => {
                self.advance();
                Some(ExpressionTokens::Operator(ch))
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                Some(ExpressionTokens::Variable(self.read_identifier()))
            }
            _ => {
                self.advance();
                None
            }
        }
    }

    pub fn tokenize(&mut self) -> Vec<ExpressionTokens> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }
}
```
