#[derive(Debug, PartialEq)]
pub enum TokenType {
    Keyword,
    Identifier,
    Symbol,
    TypeName,
    Constant,
    FunctionName,
    Terminator,
    Error,
}

#[derive(Debug)]
pub struct Token {
    pub tok_string: String,
    pub tok_type: TokenType,
    pub tok_value: i32,
}

impl Token {
    pub fn new(tok_string: &str, tok_type: TokenType, tok_value: i32) -> Self {
        Token {
            tok_string: tok_string.to_string(),
            tok_type,
            tok_value,
        }
    }
}
