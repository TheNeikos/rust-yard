use token;
use peekable_string_iterator as peek;

/// The Lexer struct represents a lexer that tokenizes
/// the string input.
pub struct Lexer<'a> {
    iter: peek::PeekableStringIterator<'a>,
    pub ast: Vec<token::Token>,
    pub errors: Vec<String> 
}

impl<'a> Lexer<'a> {
    pub fn new(input: &str) -> Lexer {
        let mut l = Lexer { 
            ast: Vec::new(),
            errors: vec![],
            iter: peek::PeekableStringIterator::new(input)
        };
        l.lex();
        l
    }

    fn lex(&mut self) {
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
            Some(c) if c.is_whitespace() => (), // Ignore whitespace
            Some(c) if c.is_numeric() => {
                // Grab the number (allowing for possibly decimals)
                let number = self.consume_number();
                // Add a numeric token to the list of tokens
                match number.parse() {
                    Ok(val) => {
                        self.ast.push(token::Token::DecimalNumber(val));
                    },
                    Err(e) => self.errors.push(format!("FATAL: {}", e))
                }
                skip_advance = true;
            },
            Some(c) if c == '+' || c == '-' => {
                // Add the operator and advance the iterator
                self.ast.push(token::Token::Operator(c, token::LEFT_ASSOCIATIVE, 2));
            },
            Some(c) if c == '*' || c == '/' => {
                // Add the operator and advance the iterator
                self.ast.push(token::Token::Operator(c, token::LEFT_ASSOCIATIVE, 3));
            },
            Some(c) if c == '(' => self.ast.push(token::Token::LeftParenthesis),
            Some(c) if c == ')' => self.ast.push(token::Token::RightParenthesis),
            Some(c) => self.errors.push(format!("Unknown identifier: {}", c)),
            None => return
        }
        // Advance the iterator and continue consuming the input
        if !skip_advance {
            self.iter.advance();
        }
        self.consume_input();
    }

    // Consumes the iterator until it reaches the end of a number
    fn consume_number(&mut self) -> String {
        let mut chars = vec![];

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
}

