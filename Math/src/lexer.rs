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
struct Lexer {
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
    fn new(input: String) -> Self {
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

      while self.peek().map_or(false, |ch| ch.is_ascii_digit() || ch == '.'){
        if let Some(ch) = self.peek(){
          valores.push(ch);
        }
        self.advance()
      }

      valores.parse::<f64>().unwrap()
    }
}
