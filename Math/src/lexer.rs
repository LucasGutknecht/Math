/*
* Lexer file
* */

use crate::evaluator::ExpressionTokens;

pub struct Lexer {
    // Lexer implementation
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
            'a'..='z' | 'A'..='Z' | '_' => Some(ExpressionTokens::Variable(self.read_identifier())),
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
