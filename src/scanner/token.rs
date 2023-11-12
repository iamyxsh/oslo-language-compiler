#[derive(Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
    pub line: usize,
    pub position: usize,
}

#[derive(Debug, PartialEq)]
pub enum TokenType {
    NumberLiteral,

    OpenParam,
    CloseParam,
    Plus,
    Dash,
    Asterisk,
    Slash,

    SemiColon,
    Whitespace,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>, line: usize, position: usize) -> Self {
        Token {
            token_type,
            value,
            line,
            position,
        }
    }
}
