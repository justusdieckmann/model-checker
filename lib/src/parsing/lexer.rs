use std::collections::HashMap;
use crate::parsing::parsing_error::{ErrorKind, ParsingError};

#[derive(Debug, PartialEq)]
pub enum LTLToken {
    AP(u8),
    Not,
    And,
    Next,
    Until,
    OpenParenthesis,
    CloseParenthesis,
}

pub fn lexer(text: &str) -> Result<Vec<LTLToken>, ParsingError> {
    #[derive(PartialEq)]
    enum State {
        None,
        InAP,
    }

    let mut tokens = Vec::<LTLToken>::new();
    let mut aps = HashMap::<String, u8>::new();
    let mut state = State::None;
    let mut ap_name = String::new();

    for (i, c) in text.chars().enumerate() {
        if state == State::InAP {
            if c.is_alphanumeric() || c == '_' {
                ap_name.push(c);
                continue;
            } else {
                let ap_option = aps.get(&ap_name);
                let val: u8;
                if ap_option.is_some() {
                    val = *ap_option.unwrap();
                } else {
                    val = aps.len() as u8;
                    aps.insert(ap_name, val);
                }

                tokens.push(LTLToken::AP(val));
                ap_name = String::new();
                state = State::None;
            }
        }

        if c.is_whitespace() {
            continue;
        }

        if c.is_ascii_lowercase() {
            state = State::InAP;
            ap_name.push(c);
        } else {
            let token = match c {
                '(' => LTLToken::OpenParenthesis,
                ')' => LTLToken::CloseParenthesis,
                '!' => LTLToken::Not,
                '&' => LTLToken::And,
                'U' => LTLToken::Until,
                'X' | 'O' => LTLToken::Next,
                _ => return Err(ParsingError::new(ErrorKind::UnexpectedToken, text, Some(i)))
            };

            tokens.push(token);
        }
    }

    if state == State::InAP {
        let ap_option = aps.get(&ap_name);
        let val: u8;
        if ap_option.is_some() {
            val = *ap_option.unwrap();
        } else {
            val = aps.len() as u8;
            aps.insert(ap_name, val);
        }
        tokens.push(LTLToken::AP(val));
    }

    return Ok(tokens);
}


#[cfg(test)]
mod tests {
    use super::*;
    use LTLToken::*;

    #[test]
    fn test_operators() {
        assert_eq!(lexer("a!Ub&)Xa("), Ok(vec![AP(0), Not, Until, AP(1), And, CloseParenthesis, Next, AP(0), OpenParenthesis]));
    }

    #[test]
    fn test_long_variables() {
        assert_eq!(lexer("aUntilB U a_until_b aUntilB"), Ok(vec![AP(0), Until, AP(1), AP(0)]));
    }

    #[test]
    fn test_invalid_chars() {
        assert_eq!(lexer("Zahl").unwrap_err().kind(), ErrorKind::UnexpectedToken);
        assert_eq!(lexer("a ? b").unwrap_err().kind(), ErrorKind::UnexpectedToken);
    }
}
