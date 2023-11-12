#[cfg(test)]
mod test;
pub mod token;

use crate::scanner::token::TokenType;
use token::Token;

pub fn scan_code(code: &str) -> Result<Vec<Token>, String> {
    let mut tokens: Vec<Token> = vec![];
    let mut position = 0;
    let chars: Vec<char> = code.chars().collect();
    for (index, char) in chars.clone().into_iter().enumerate() {
        if index < position {
            continue;
        }
        let mut length: usize = 1;
        let mut value = char.to_string();
        if chars.get(index + 1).is_none() {
            break;
        }
        if char.is_numeric() {
            let mut token = Token::new(TokenType::NumberLiteral, None, 1, position);

            while chars.get(index + length).unwrap().is_numeric() {
                value.push(chars.get(index + length).unwrap().to_owned());
                length = length + 1;
            }

            token.value = Some(value.clone());
            tokens.push(token);
            position = position + length;
        } else if char.is_whitespace() {
            tokens.push(Token::new(TokenType::Whitespace, None, 1, position));
            position = position + 1;
        } else if char == '+' {
            tokens.push(Token::new(TokenType::Plus, None, 1, position));
            position = position + 1;
        } else if char == '(' {
            tokens.push(Token::new(TokenType::OpenParam, None, 1, position));
            position = position + 1;
        } else if char == '*' {
            tokens.push(Token::new(TokenType::Asterisk, None, 1, position));
            position = position + 1;
        } else if char == ')' {
            tokens.push(Token::new(TokenType::CloseParam, None, 1, position));
            position = position + 1;
        } else if char == ';' {
            tokens.push(Token::new(TokenType::SemiColon, None, 1, position));
            position = position + 1;
        } else {
            return Err(format!(
                "Invalid Code on line 1 at position {position} - {char}"
            ));
        }
    }
    Ok(tokens)
}
