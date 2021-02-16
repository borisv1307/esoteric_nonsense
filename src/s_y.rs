use std::iter;
use std::str;
use std;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(char, u32, u32),   // Operator, associativity, precedence
    Variable(char),
    WholeNumber(i64),
    DecimalNumber(f64),
    FunctionCall(String),
    Comma,
    LeftParenthesis,
    RightParenthesis,
    Whitespace
}

pub const LEFT_ASSOCIATIVE: u32 = 1;
pub const RIGHT_ASSOCIATIVE: u32 = 2;


pub struct PeekableStringIterator<'a> {
    iter: iter::Peekable<str::Chars<'a>>
}

impl<'a> PeekableStringIterator<'a> {
    pub fn new() -> PeekableStringIterator<'a> {
        PeekableStringIterator {
            iter: "".chars().peekable()
        }
    }

    pub fn set_input(&mut self, raw_input: &'a str) {
        self.iter = raw_input.chars().peekable();
    }

    pub fn advance(&mut self) {
        self.iter.next();
    }

    pub fn peek(&mut self) -> Option<&char> {
        self.iter.peek()
    }
}




pub struct Lexer<'a> {
    iter: PeekableStringIterator<'a>,
    sign: Option<char>,
    pub ast: Vec<Token>,
    pub errors: Vec<String> 
}

impl<'a> Lexer<'a> {
    pub fn new() -> Lexer<'a> {
        Lexer { 
            ast: Vec::new(),
            errors: vec![],
            iter: PeekableStringIterator::new(),
            sign: None
        }
    }

    pub fn lex(&mut self, raw_input: &'a str) {
        // Clear out everything
        self.ast.clear();
        self.errors.clear();

        self.iter.set_input(raw_input);
        self.consume_input();
    }

    // Recursively consume the input
    fn consume_input(&mut self) {
        // Should we skip advancing if a sub method has done it for us?
        let mut skip_advance = false;

        // Peek the next character
        let peeked: Option<char> = match self.iter.peek() {
            Some(&c) => Some(c),
            None => None
        };

        // Decide what to do
        match peeked {
            Some(c) if c.is_whitespace() => {
                // Reset the sign
                self.sign = None;

                self.ast.push(Token::Whitespace);
            }, 
            Some(c) if c.is_numeric() => {
                // Grab the number (allowing for possible decimals)
                let number = self.consume_number();
                // Add a numeric token to the list of tokens
                match number.parse() {
                    Ok(val) => {
                        self.ast.push(Token::DecimalNumber(val));
                    },
                    Err(e) => self.errors.push(format!("FATAL: {}", e))
                }
                skip_advance = true;
            },
            Some(c) if c == '+' || c == '-' => {
                // Add the operator and advance the iterator
                self.ast.push(Token::Operator(c, LEFT_ASSOCIATIVE, 2));
            },
            Some(c) if c == '*' || c == '/' => {
                // Add the operator and advance the iterator
                self.ast.push(Token::Operator(c, LEFT_ASSOCIATIVE, 3));
            },
            Some(c) if c == '^' => self.ast.push(Token::Operator(c, RIGHT_ASSOCIATIVE, 4)),
            Some(c) if c == '(' => self.ast.push(Token::LeftParenthesis),
            Some(c) if c == ')' => self.ast.push(Token::RightParenthesis),
            Some(c) if c == ',' => self.ast.push(Token::Comma),
            Some(c) if c == 'x' => self.ast.push(Token::Variable(c)),
            Some(c) if c == '𝜋' => self.ast.push(Token::DecimalNumber(std::f64::consts::PI)),
            Some(c) if c == '𝑒' => self.ast.push(Token::DecimalNumber(std::f64::consts::E)),
            Some(c) if c.is_alphabetic() && c != 'x' => {
                let ident = self.consume_identifier();
                self.ast.push(Token::FunctionCall(ident));
                skip_advance = true;
            },
            None => return,
            _ => return
        }
        // Advance the iterator and continue consuming the input
        if !skip_advance {
            self.iter.advance();
        }
        self.consume_input();
    }

    // Consumes the iterator until it reaches the end of a number
    fn consume_number(&mut self) -> String {
        // Decipher the sign of the number we want to consume
        self.decipher_sign();

        // Initialize our number with the given sign
        let mut chars = vec![self.sign.unwrap_or('+')];

        // Reset the sign
        self.sign = None;

        // Loop over every character until we reach a non-numeric one
        loop {
            match self.iter.peek() {
                Some(c) if c.is_numeric() || *c == '.' => chars.push(*c),
                Some(c) if !c.is_numeric() => break,
                //Some(c) => println!("Peeking at: {}", c),
                None => break,
                _ => break
            }
            self.iter.advance();
        }

        // Return out number as a String
        chars.into_iter().collect()
    }

    // Consumes an identifier until we don't have any other letters available
    fn consume_identifier(&mut self) -> String {
        let mut result = vec![];
        loop {
            match self.iter.peek() {
                Some(c) if c.is_alphabetic() => result.push(*c),
                _ => break
            }
            self.iter.advance();
        }

        result.into_iter().collect()
    }

    fn decipher_sign(&mut self) {
        // If the last operator was a sign ... set the sign
        let last_op = self.ast.last().cloned();
        match last_op {
            Some(Token::Operator(o, _, _)) => {
                if o == '+' || o == '-' {
                    // Pop the operator (because its not an operator .. its indicating the numbers'
                    // sign) and store our sign
                    self.ast.pop();
                    self.sign = Some(o);
                }
            },
            _ => ()
        }
    }
}



pub fn calculate(input: &Vec<Token>) -> Result<f64, Vec<String>> {
    let mut input = input.clone();
    let mut stack = Vec::new();
    let mut len = input.len();
    let mut errors = Vec::new();

    // Iterate over the tokens and calculate a result
    while len > 0 {
        let tok = input.remove(0);
        match tok {
            Token::DecimalNumber(n) => stack.push(Token::DecimalNumber(n)),
            Token::Operator(o, _, _) => {
                let right = stack.pop();
                let left = stack.pop();

                match (left, right) {
                    (Some(Token::DecimalNumber(n1)), Some(Token::DecimalNumber(n2))) => stack.push(Token::DecimalNumber(operate(o, n1, n2))),
                    _ => break
                }
            },
            Token::WholeNumber(n) => stack.push(Token::DecimalNumber(n as f64)),
            Token::FunctionCall(function_name) => {
                match &function_name as &str {
                    "log" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).log10())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).log10())),
                            _ => ()
                        }
                    },
                    //TODO: log(n, base)
                    "ln" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).ln())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).ln())),
                            _ => ()
                        }
                    },
                    //TODO: negative check
                    "sqrt" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).sqrt())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).sqrt())),
                            _ => ()
                        }
                    },
                    "abs" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).abs())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).abs())),
                            _ => ()
                        }
                    },
                    "sin" => {
                        let arg = stack.pop();

                        if let Some(Token::DecimalNumber(n1)) = arg {
                            stack.push(Token::DecimalNumber(n1.sin()));
                        }
                    },
                    "cos" => {
                        let arg = stack.pop();

                        if let Some(Token::DecimalNumber(n1)) = arg {
                            stack.push(Token::DecimalNumber(n1.cos()));
                        }
                    },
                    "tan" => {
                        let arg = stack.pop();

                        if let Some(Token::DecimalNumber(n1)) = arg {
                            stack.push(Token::DecimalNumber(n1.tan()));
                        }
                    },
                    "csc" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber(
                                if (n as f64).sin() == 0.0 {
                                    std::f64::NAN 
                                } else {
                                    1.0 / (n as f64).sin()
                                })),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber(
                                if (n as f64).sin() == 0.0 {
                                    std::f64::NAN 
                                } else {
                                    1.0 / (n as f64).sin()
                                })),
                            _ => ()
                        }
                    },
                    "sec" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber(
                                if (n as f64).cos() == 0.0 {
                                    std::f64::NAN 
                                } else { 
                                    1.0 / (n as f64).cos() 
                                })),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber(
                                if (n as f64).cos() == 0.0 {
                                    std::f64::NAN 
                                } else { 
                                    1.0 / (n as f64).cos() 
                                })),
                            _ => ()
                        }
                    },
                    "cot" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber(
                                if (n as f64).tan() == 0.0 { 
                                    std::f64::NAN 
                                } else { 
                                    1.0 / (n as f64).tan()
                                })),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber(
                                if (n as f64).tan() == 0.0 { 
                                    std::f64::NAN 
                                } else { 
                                    1.0 / (n as f64).tan()
                                })),
                            _ => ()
                        }
                    },
                    "sinh" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).sinh())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).sinh())),
                            _ => ()
                        }
                    },
                    "cosh" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).cosh())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).cosh())),
                            _ => ()
                        }
                    },
                    "tanh" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).tanh())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).tanh())),
                            _ => ()
                        }
                    },
                    "asin" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).asin())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).asin())),
                            _ => ()
                        }
                    },
                    "acos" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).acos())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).acos())),
                            _ => ()
                        }
                    },
                    "atan" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).atan())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).atan())),
                            _ => ()
                        }
                    },
                    "asinh" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).asinh())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).asinh())),
                            _ => ()
                        }
                    },
                    "acosh" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).acosh())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).acosh())),
                            _ => ()
                        }
                    },
                    "atanh" => {
                        let arg = stack.pop();
                        match arg {
                            Some(Token::DecimalNumber(n)) => stack.push(Token::DecimalNumber((n as f64).atanh())),
                            Some(Token::WholeNumber(n)) => stack.push(Token::DecimalNumber((n as f64).atanh())),
                            _ => ()
                        }
                    },
                    //TODO: acsc, asec, acot, csch, sech, coth, acsch, asech, acoth
                     "floor" => {
                        let arg = stack.pop();

                        if let Some(Token::DecimalNumber(n1)) = arg {
                            stack.push(Token::DecimalNumber(n1.floor()));
                        }
                    },
                    "ceil" => {
                        let arg = stack.pop();

                        if let Some(Token::DecimalNumber(n1)) = arg {
                            stack.push(Token::DecimalNumber(n1.ceil()));
                        }
                    },
                    "round" => {
                        let arg = stack.pop();

                        if let Some(Token::DecimalNumber(n1)) = arg {
                            stack.push(Token::DecimalNumber(n1.round()));
                        }
                    },
                    "trunc" => {
                        let arg = stack.pop();

                        if let Some(Token::DecimalNumber(n1)) = arg {
                            stack.push(Token::DecimalNumber(n1.trunc()));
                        }
                    },
                    "fract" => {
                        let arg = stack.pop();

                        if let Some(Token::DecimalNumber(n1)) = arg {
                            stack.push(Token::DecimalNumber(n1.fract()));
                        }
                    },
                    "max" => {
                        let right = stack.pop();
                        let left = stack.pop();

                        if let (Some(Token::DecimalNumber(n1)), Some(Token::DecimalNumber(n2))) = (left, right) {
                            stack.push(Token::DecimalNumber(n1.max(n2)));
                        }
                    },
                    "min" => {
                        let right = stack.pop();
                        let left = stack.pop();

                        if let (Some(Token::DecimalNumber(n1)), Some(Token::DecimalNumber(n2))) = (left, right) {
                            stack.push(Token::DecimalNumber(n1.min(n2)));
                        }
                    },                    
                    _ => errors.push(format!("Unknown identifier: {}", function_name))
                }
            },
            _ => ()
        }
        len = input.len();
    }

    if errors.len() > 0 {
        Err(errors)
    } else {
        let result = stack.pop();

        match result {
            Some(Token::DecimalNumber(n)) => Ok(n),
            _ => Err(errors)
        }
    }
}

fn operate(operator: char, left: f64, right: f64) -> f64 {
    match operator {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        '^' => left.powf(right),
        _ => 0f64
    }
}





/// The ShuntingYard struct transforms an expression
/// to a 64-bit floating point value
pub struct ShuntingYard<'a> {
    lexer: Lexer<'a>,
    output_queue: Vec<Token>,
    stack: Vec<Token>,
    errors: Vec<String>
}

impl<'a> ShuntingYard<'a> {
    pub fn new() -> ShuntingYard<'a> {
        ShuntingYard {
            lexer: Lexer::new(),
            output_queue: vec![],
            stack: vec![],
            errors: vec![]
        }
    }

    /// calculate returns a 64-bit floating value after
    /// parsing the Reverse Polish Notation represented
    /// by the output_queue.
    pub fn calculate(&mut self, raw_input: &'a str) -> Result<f64, Vec<String>> {
        // Clear out everything
        self.output_queue.clear();
        self.stack.clear();
        self.errors.clear();

        // Lex the input
        self.lexer.lex(raw_input);

        // If there were Lexer errors, add them now.
        let lexer_errors = self.lexer.errors.clone();
        self.errors.extend(lexer_errors);

        // Transform the Lexer input via the Shunting Yard algorithm
        self.transform();

        // If there are lexer errors, return early with them
        if self.errors.len() > 0 {
            println!("Errors: {:?}", self.errors);
            return Err(self.errors.clone())
        }

        calculate(&self.output_queue)
    }

    // Transforms the input from the Lexer in to the output_queue
    // and stack based on the Shunting Yard algorithm
    fn transform(&mut self) {
        // Iterate over each token and move it based on the algorithm
        for tok in &self.lexer.ast {
            // If the token is a number, then add it to the output queue
            match *tok {
                Token::WholeNumber(_) => self.output_queue.push(tok.to_owned()),
                Token::DecimalNumber(_) => self.output_queue.push(tok.to_owned()),
                Token::FunctionCall(_) => self.stack.push(tok.to_owned()),
                Token::Operator(o1, o1_associativity, o1_precedence) => {
                    while self.stack.len() > 0 {
                        match self.stack.last() {
                            Some(&Token::Operator(_, _, o2_precedence)) => {
                                if (o1_associativity == LEFT_ASSOCIATIVE &&
                                   o1_precedence <= o2_precedence) ||
                                   (o1_associativity == RIGHT_ASSOCIATIVE &&
                                   o1_precedence < o2_precedence) { 
                                       self.output_queue.push(self.stack.pop().unwrap());
                                   } else {
                                       break
                                   }
                            },
                            Some(&Token::FunctionCall(_)) => {
                                self.output_queue.push(self.stack.pop().unwrap());
                            },
                            _ => break
                        }
                    }
                    self.stack.push(Token::Operator(o1, o1_associativity, o1_precedence));
                },
                Token::LeftParenthesis => self.stack.push(Token::LeftParenthesis),
                Token::RightParenthesis => {
                    loop {
                        match self.stack.last() {
                            Some(&Token::LeftParenthesis) => {
                                self.stack.pop().unwrap(); 
                                break;
                            },
                            None => {
                                self.errors.push("Unbalanced parenthesis".to_string());
                                break;
                            },
                            _ => self.output_queue.push(self.stack.pop().unwrap()),
                        }
                    }
                },
                Token::Comma => {
                    loop {
                        match self.stack.last() {
                            Some(&Token::LeftParenthesis) => {
                                break;
                            },
                            _ => {
                                if let Some(tok) = self.stack.pop() {
                                    self.output_queue.push(tok);
                                } else {
                                    self.errors.push("Syntax error.".to_string());
                                    break;
                                }
                            }
                        }
                    }
                },
                _ => ()
            }
        }

        // Are there any operators left on the stack?
        while self.stack.len() > 0 {
            // Pop them off and push them to the output_queue
            let op = self.stack.pop();
            match op {
                Some(Token::LeftParenthesis) => {
                    self.errors.push("Unbalanced parenthesis".to_string());
                    break;
                },
                Some(Token::RightParenthesis) => {
                    self.errors.push("Unbalanced parenthesis".to_string());
                    break;
                },
                _ => self.output_queue.push(op.unwrap())
            }
        }
    }

    /// to_string_ast returns the string representation of the
    /// Lexer tokens.
    pub fn to_string_ast(&self) -> String {
        let mut result = String::new(); // String to output the result to

        // Loop over each item in the AST and print a String representation of it
        for tok in &self.lexer.ast {
            match *tok {
                Token::Operator(c, _, _) => result.push(c),
                Token::FunctionCall(ref f) => result.push_str(&f.clone()[..]),
                Token::DecimalNumber(n) => result.push_str(&n.to_string()[..]),
                Token::LeftParenthesis => result.push_str("("),
                Token::RightParenthesis => result.push_str(")"),
                Token::Comma => result.push_str(","),
                _ => ()
            };

            if *tok != Token::Whitespace {
                result.push_str(" "); // Space separated
            }
        }

        // Return the result
        result
    }
}

impl<'a> std::string::ToString for ShuntingYard<'a> {
    /// to_string returns the string representation of the Shunting Yard
    /// algorithm in Reverse Polish Notation.
    fn to_string(&self) -> String {
        let mut result = String::new(); // String to output the result

        // Iterate over the output queue and print each one to the result
        for tok in &self.output_queue {
            match *tok {
                Token::Operator(c, _, _) => result.push(c),
                Token::FunctionCall(ref f) => result.push_str(&f.clone()[..]),
                Token::DecimalNumber(n) => result.push_str(&n.to_string()[..]),
                Token::LeftParenthesis => result.push_str("("),
                Token::RightParenthesis => result.push_str(")"),
                Token::Comma => result.push_str(","),
                _ => ()
            };

            if *tok != Token::Whitespace {
                result.push_str(" "); // Space separated
            }
        }

        result 
    }

}
