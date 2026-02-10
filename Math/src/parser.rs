/* The parser file*/


/* 
* This struct details the BindPower which consists of:
*   - Operator of the type char, which receives the operator itself.
*   - LBP(Left bind power): Which is the bind power of the given operator is in how tight it
*   attracts its operands on the left. 
*   - RBP(Right bind power): Entails the bind power of the given operator on how tight it attracts its operands from the right. 
*   
*   As example: if the left of the operator + as a bind of 10 and its right has a bind of 9, given 3 + 4 + 5, it would evaluate (3 + 4) + 5. 
*   
*   Both, lbp and rbp use the type i32 (Which is an 32-bit signed integer type as in: https://doc.rust-lang.org/std/primitive.i32.html)
*
* */
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
            Some(ExpressionTokens::LeftParenthesis) => {
                self.advance();
                let inner = self.parse_expression(0)?;
                match self.peek(){
                    Some(ExpressionTokens::RightParenthesis) => self.advance(),
                    _ => return None,
                }
                inner
            },
            _ => {
                return None
            }
        };
            
        loop{
            
            let token = self.peek();
            match token {
                Some(ExpressionTokens::Operator(token)) => {
                    let (lbp, rbp)  = match BindPower::get_bind_power(*token) {
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

    fn parse(&mut self) -> Option<ASTNode> {
        self.parse_expression(0)
    }
}
