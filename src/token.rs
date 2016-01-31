#[derive(Clone)]
pub enum Token {
    Operator(char, u32, u32),   // Operator, associativity, precedence
    WholeNumber(i64),
    DecimalNumber(f64),
    FunctionCall(String, Vec<String>),
    LeftParenthesis,
    RightParenthesis
}

pub const LEFT_ASSOCIATIVE: u32 = 1;
pub const RIGHT_ASSOCIATIVE: u32 = 2;