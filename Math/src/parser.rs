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

/*
* This struct details the Parser which consists of:
*   
*   - Position of the type usize, which is a primitive pointer-sized unsigned integer. It takes to
*   reference how many bytes into locations of the memory. Thats being, it maximum value is 2^64 -1
*   on 64-bit targets. And the minimum is Zero. TLDR: Most of the rust integer types use consistent
*   memory regardless of the system. As example: u32, always use 32 bits (4 bytes) of memory an go
*   one for types like u8, i64 etc. In contrast, usize and isize are architecture-dependent type
*   whose size adapts to the underling system. Those types are more like aliases/nicknames than integer types
*   per se... we can define them like this (See: https://towardsdev.com/understanding-rusts-dynamic-integer-types-usize-and-isize-60b44dd581b6):
*       - usize: An unsigned integer type for representing sizes and indices
*           let's that be:
*               let days: usize = 55; It is equivalent u32 on a 32-bit system and u64 on a 64-bit
*               system.
*               Use this for array indices, collections lengths, and memory sizes.
*       - isize: A signed integer type that can represent both positive and negative values
*           let's that be:
*               let count: isize = -15000; It is the equivalent to i32 on a 32-bit system and i64
*               on a 64-bit system.
*               Use this when you need signed integers that match the platform's word size.
*
*       - Signed vs Unsigned
*           Signed: It may contain both positive or negative integers.
*           Unsigned: It contains only positive integers.
*
*           From Google:
*
*            +------------------+-----------------------------------------------+-----------------------------------------------+
*            | Feature          | Signed Integers (i8, i32, isize, etc.)          | Unsigned Integers (u8, u32, usize, etc.)        |
*            +------------------+-----------------------------------------------+-----------------------------------------------+
*            | Values           | Can be negative, positive, or zero.             | Can be zero or positive only (non-negative).   |
*            |                  | Stored using two's complement representation.   |                                               |
*            +------------------+-----------------------------------------------+-----------------------------------------------+
*            | Range            | -(2^(n-1)) to 2^(n-1) - 1                        | 0 to 2^n - 1                                   |
*            |                  | Roughly split between negative and positive.    | Twice the max positive value of signed type.   |
*            +------------------+-----------------------------------------------+-----------------------------------------------+
*            | Use Cases        | General math, values that may go negative,      | Counting items, array indexing, memory         |
*            |                  | temperature, financial debt.                    | addresses/offsets, bitwise operations.         |
*            +------------------+-----------------------------------------------+-----------------------------------------------+
*            | Compiler Safety  | Overflow panics in debug builds.                | Negative literals rejected at compile time.    |
*            |                  | Helps catch unexpected behavior.                | Underflow (e.g., 0 - 1) panics in debug builds.|
*            +------------------+-----------------------------------------------+-----------------------------------------------+
*
*
*   - Tokens of the type Vec, encapsulating ExpressionTokens, which is a type defined on
*   evaluator.rs. Vecs can be defined as: A contiguous (One after other like a chain where a piece
*   touches the next one and vice-versa) ever growing array type. (See: https://doc.rust-lang.org/std/vec/struct.Vec.html)
*
* So given the fields of the struct we can define the whole structured like this: The parser must
* know the position and the tokens akin of that position on the evaluator.
* */

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
