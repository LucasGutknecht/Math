/* The parser file*/
struct BindPower {
    operator: char,
    lbp: i32,
    rbp: i32,
}

struct Parser {
    position: usize,
    tokens: Vec<ExpressionTokens>
}

impl BindPower{
    fn get_bind_power(ch: char) -> Option<(i32, i32)> {
        match ch {
            '+' | '-' => {
                return Some((10, 9))
            },
            '*' | '/' => {
                return Some((20, 19))
            },
            _ => None,
        }  
    }

}

impl Parser {
    fn new(tokens: Vec<ExpressionTokens>) -> Self {
        Parser { position: 0, tokens }
    }

    fn peek(&self) -> Option<&ExpressionTokens> {
        self.tokens.get(self.position)
    }
    
    fn advance(&mut self) {
        self.position += 1;
    }

    fn parse_expression(&mut self, min_bp: i32) -> Option<ASTNode>{
        let mut left = match self.peek() {
            Some(ExpressionTokens::Number(token)) => {
                self.advance();
                ASTNode::Number(*token)
            },
            Some(ExpressionTokens::Variable(token)) => {
                self.advance();
                ASTNode::Variable(token.clone())
            },
            _ => {
                return None
            }
        };
            
        loop{
            
            let token = self.peek();
            match token {
                Some(ExpressionTokens::Operator(token)) => {
                    let (lbp, rbp)  = match get_bind_power(*token) {
                        Some(bp) => bp,
                        None => break,
                    };
                    if lbp <= min_bp {
                        break;
                    }

                    let op = *token;
                    self.advance();

                    let right = self.parse_expression(rbp)?;

                    left = ASTNode::Operator {
                        operator: op,
                        left: Box::new(left),
                        right: Box::new(right),
                    };

                },
                _ => break
            }
        }
        Some(left)
    }
}
