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
/*
parse_expression(min_bp){
    
}
*/
impl Parser {
    fn new(tokens: Vec<ExpressionTokens>) -> Self {
        Parser { position: 0, tokens }
    }

    fn peek(&self) -> Option<&ExpressionTokens> {
        self.input.chars().nth(self.position)
    }
}
