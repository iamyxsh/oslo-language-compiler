use crate::scanner::scan_code;
use crate::scanner::token::{Token, TokenType};

#[cfg(test)]
use pretty_assertions::{assert_eq, assert_ne};

#[test]
fn it_can_scan_simple_code() {
    let code = "100 + (2 * 3);";

    let tokens = vec![
        Token::new(TokenType::NumberLiteral, Some("100".to_string()), 1, 0),
        Token::new(TokenType::Whitespace, None, 1, 3),
        Token::new(TokenType::Plus, None, 1, 4),
        Token::new(TokenType::Whitespace, None, 1, 5),
        Token::new(TokenType::OpenParam, None, 1, 6),
        Token::new(TokenType::NumberLiteral, Some("2".to_string()), 1, 7),
        Token::new(TokenType::Whitespace, None, 1, 8),
        Token::new(TokenType::Asterisk, None, 1, 9),
        Token::new(TokenType::Whitespace, None, 1, 10),
        Token::new(TokenType::NumberLiteral, Some("3".to_string()), 1, 11),
        Token::new(TokenType::CloseParam, None, 1, 12),
        Token::new(TokenType::SemiColon, None, 1, 13),
    ];

    assert_eq!(scan_code(code).unwrap(), tokens);
}
