/*
* Lexer file
* */

use crate::evaluator::ExpressionTokens;

/*
  The lexer job is to take the raw input string and convert it into a series of tokens that can be easily processed by the parser.

  Each token represents a meaningful unit in the mathematical expression, such as numbers, operators, parentheses, functions, and variables. For this case, we will use two main fields: input and position.
  -> input: the raw string input that the lexer will process.
  -> position: the current position in the input string that the lexer is processing.

  The other implementations will be done on the lexer methods.

  Why not use current_char and token as fields?

  current_char: given that the lexer structure job is to create tokens, there is no need to compute the next tokes in to the exact moment we are processing the actual token, the future computation should be done in the future.
  token: the lexer should not hold the current token as a field, because the lexer is responsible for generating tokens from the input string, not storing them. The tokens should be generated on-the-fly as needed, rather than being stored in the lexer structure.

  Example how the lexer would see the expression (3 + 5 * (2 - 8)):
  position 0: '('
  position 1: '3'
  position 2: ' ' -> whitespace, skip
  position 3: '+'
  position 4: ' ' -> whitespace, skip
  position 5: '5'
  position 6: ' ' -> whitespace, skip
  position 7: '*'
  position 8: ' ' -> whitespace, skip
  position 9: '('
  position 10: '2'
  position 11: ' ' -> whitespace, skip
  position 12: '-'
  position 13: ' ' -> whitespace, skip
  position 14: '8'
  position 15: ')'
  position 16: ')'
*/
pub struct Lexer {
    // Lexer implementation
    position: usize,
    input: String,
}
/*
How do we implement the lexer, what underlying rules should we follow? What logic implement?

1 -> Define what tokens and what are not-tokens (whitespaces, invalid characters etc). The data here are Raw Strings and positions.
  Example of the data: "3 + 5 * (2 - 8)" -> tokens: [3, +, 5, *, (, 2, -, 8, )]
2 -> Define the patterns to be matched for each token type (numbers, operators, functions, variables, parentheses).  The data here are Patterns and Identifiers.
  Example of the data: number_pattern = r"\d+(\.\d+)?
3 -> Implement the logic to iterate on which pattern is to apply and whne. The data here are rules and conditions.
  Example of the data: if current_char is digit -> match number_pattern
4 -> Create the tokens based on the matched patterns and store them in a structured format (like an enum or struct). The data here are Tokens and Types.
  Example of the data: Token::Number(3.0), Token::Operator('+')
5 -> Handle errors for invalid characters or sequences. The data here are Error types and Messages.
  Example of the data: Error::InvalidCharacter('@')
6 -> Return the list of tokens for further processing by the parser. The data here are Token lists and Structures.
  Example of the data: vec![Token::Number(3.0), Token::Operator('+'), Token::Number(5.0), ...]

On the methods per se, we can have:
- fn new(input: String) -> Self: initializes the lexer with the input string and sets
    Why do we need to initialize?

- fn peek(&self) -> Option<char>: returns the next character without advancing the position.
    Why do we need to peek?

- fn advance(&mut self): advances the position to the next character.
    Why do we need to advance?


Given the methods, when met with a proposition like 3 + 5 * (2 - 8), the lexer would process it as follows:
1 - Initialize the lexer with the input string "3 + 5 * (2 - 8)". Being at position 0, which is '('.
2 - Use peek to look at the current character. It sees '3', which is a digit.
3 - Recognize '(' as a left parenthesis token, create Token::LeftParen, and advance the position.
4 - Use peek to look at the current character. It sees '3', which is a digit.
5 - Skip whitespace using advance until it reaches '+'.
6 - Recognize '+' as an operator token, create Token::Operator('+'), and advance the position.
7 - Repeat the process for '5', '*', '(', '2', '-', '8', and ')', creating the corresponding tokens.
8 - Handle any invalid characters by generating error tokens if necessary.
9 - Return the list of tokens for further processing by the parser: [Token::LeftParen,Token::Number(3.0), Token::Operator('+'), Token::Number(5.0), Token::Operator('*'), Token::LeftParen, Token::Number(2.0), Token::Operator('-'), Token::Number(8.0), Token::RightParen]

*/
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

    /*
      The idea here is to run skip_whitespace, which give us an notion of the self is a white space or not. It does that by generating a loop using While, where we take the peek results and map them using map_or method... this methods works like this:
      map_or<U, F> (default value when wrong, function result(closure) i want to return if correct)
      if true, enter the while block and use the method advance of the lexer. Example:

      fn main() {
        let x = Some("foo");
        assert_eq!(x.map_or(42, |v| v.len()), 3);

        let x: Option<&str> = None;
        assert_eq!(x.map_or(42, |v| v.len()), 42);
      }

    */
    fn skip_whitespace(&mut self) {
        while self.peek().map_or(false, |ch| ch.is_whitespace()) {
            self.advance();
        }
    }

    /*
      The idea here is to run read_number to check if the char is number or dot and return false if not on the while block.
      Given that I cannot access the context of the ch of the map_or, given it only exist there, I may access the ch from the peek method itself, if exist, he pushes the ch in the string, appeding the value.

      After that advance until the end of the while loop.

      The method read_number expects an return of the type f64, to achive so, I must get the method parse of the type String.

      The method parse works like this:

      thestring.parse::<type I wanto to parse into>.unwrap()
      ::<> -> this sintaxe is called turbofish, it indicates directly to the program the type you wanto to parse into.
      unwrap() -> this method helps us to retrive the value parsed directly(Some) and return a panic if the result is the type None.
    */
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

    /*
     *
     * The function next_token is expexted to reach the next token of the lexer.
     *
     * The fucntion returns a type ExpressionTokens encapsulated by Option, the option is an enum
     * used to Handle absence of an value, eliminating the null pointer errors like:
     *
     * Option<T> -> T = Types
     * Returns:
     *   Some(T) -> Whichever variant that takes the value of type T
     *   None -> Whichever variant that takes the absence of value; None can be assigned directly
     *   as None(Without the ; at the end)
     *
     * The function pass as param: &mut self -> https://idiomatic-rust-snippets.org/essentials/std-lib/self.html
     *
     * It may be understood as:
     *  Self is used to refer to the instance of the struct or enum with its own associated methods.
     *  As param, it indicates thats the method takes ownershiip, borrows the immutability or mutabilility of the instance.
     *  That means that the methods takes characteristics from the instance per se, given whatever meaning the instance has
     *  stablished prior, it dos as:
     *   - self -> it means the method takes the ownershiip of the instance.
     *   - &self -> it means the method borrows the instance immutability.
     *   - &mut self -> it means the method borrows the instance mutably.
     *
     * The function function on the following steps:
     *
     * Step 1 - We should assert that the actual char is not a white space by calling skip_whitespace
     * Step 2 - After that we should put the current char to a variable, using ? will assert that if the
     *   value is not what we expected, returns None.
     * Step 3 - We will match the char and execute the accordingly method.
     *   - If a digit from 0 to 9, return self.read_number() which transforms the String into a number of type f64, this is encapsulated into a Number type stablished on the ExpressionTokens enum,
     *   encapsulated on Some()(An variant of the Option<T> enum).
     *
     * Given that we could likely define some points:
     *
     * - The match methods will be encapsulated in the Some wing, where it brings something or
     * None. The desired methods then, will be encapsulated in the methods of the evaluator.rs:
     * ExpressionTokens, Number, Operator etc.
     * -
     * - the _ option means everything else, returning the None. It could call something like panic.
     * But I decided to leave to None.
     *
     * */
    fn next_token(&mut self) -> Option<ExpressionTokens> {
        self.skip_whitespace();

        let ch = self.peek()?;

        match ch {
            '0'..='9' => {
                return Some(ExpressionTokens::Number(self.read_number()));
            }
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
                return Some(ExpressionTokens::Variable(self.read_identifier()));
            }
            _ => {
                self.advance();
                None
            }
        }
    }

    /*
     *   The purpose of the function tokenize is to create a vector of the type ExpressionTokens.
     *
     *   The function operates like:
     *
     *   1 - Declare a new Vec.
     *   2 - Initialize an look with:
     *       - IF some(token) which means there is some token, then, if not, None.
     *           - If Some: push the token to the Vec. PUSH: Appends an element to the back of a
     *           collection. (https://doc.rust-lang.org/std/vec/struct.Vec.html#method.push)
     *   3 - At the end, return a vec of tokens like this: ['(','3','+','8',')','+','5']
     * */
    pub fn tokenize(&mut self) -> Vec<ExpressionTokens> {
        let mut tokens = Vec::new();

        while let Some(token) = self.next_token() {
            tokens.push(token);
        }

        tokens
    }
}
