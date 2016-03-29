
#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Associativity {
    Left, Right
}

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    Operator(char, Associativity, u32),   // Operator, associativity, precedence
    WholeNumber(i64),
    DecimalNumber(f64),
    FunctionCall(String),
    Variable(String),
    Comma,
    LeftParenthesis,
    RightParenthesis,
    Whitespace
}
