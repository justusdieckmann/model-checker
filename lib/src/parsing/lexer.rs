use crate::parsing::parsing_error::{ErrorKind, ParsingError};
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum LTLTokenUnaryPrefix {
    Next,
    Not,
    Future,
    Generally,
}

#[derive(Debug, PartialEq)]
pub enum LTLTokenBinaryInfix {
    And,
    Or,
    #[allow(dead_code)]
    Implies,
    Until,
    WeakUntil,
    Release,
}

#[derive(Debug, PartialEq)]
pub enum LTLTokenAtomic {
    AP(u8),
    True,
    False,
}

#[derive(Debug, PartialEq)]
pub enum LTLToken {
    BinaryInfix(LTLTokenBinaryInfix),
    UnaryPrefix(LTLTokenUnaryPrefix),
    Atomic(LTLTokenAtomic),
    OpenParenthesis,
    CloseParenthesis,
}

pub fn lexer(text: &str) -> Result<(Vec<LTLToken>, HashMap<String, u8>), ParsingError> {
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

                tokens.push(LTLToken::Atomic(LTLTokenAtomic::AP(val)));
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
                '!' => LTLToken::UnaryPrefix(LTLTokenUnaryPrefix::Not),
                '&' => LTLToken::BinaryInfix(LTLTokenBinaryInfix::And),
                '|' => LTLToken::BinaryInfix(LTLTokenBinaryInfix::Or),
                'U' => LTLToken::BinaryInfix(LTLTokenBinaryInfix::Until),
                'X' | 'O' => LTLToken::UnaryPrefix(LTLTokenUnaryPrefix::Next),
                'R' => LTLToken::BinaryInfix(LTLTokenBinaryInfix::Release),
                'W' => LTLToken::BinaryInfix(LTLTokenBinaryInfix::WeakUntil),
                'G' => LTLToken::UnaryPrefix(LTLTokenUnaryPrefix::Generally),
                'F' => LTLToken::UnaryPrefix(LTLTokenUnaryPrefix::Future),
                '0' => LTLToken::Atomic(LTLTokenAtomic::False),
                '1' => LTLToken::Atomic(LTLTokenAtomic::True),
                _ => return Err(ParsingError::new(ErrorKind::UnexpectedToken, text, Some(i))),
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
        tokens.push(LTLToken::Atomic(LTLTokenAtomic::AP(val)));
    }

    Ok((tokens, aps))
}

#[cfg(test)]
mod tests {
    use super::*;
    use LTLToken as L;
    use LTLTokenAtomic as A;
    use LTLTokenBinaryInfix as B;
    use LTLTokenUnaryPrefix as U;

    #[test]
    fn test_operators() {
        assert_eq!(
            lexer("a!Ub&)Xa(").unwrap().0,
            vec![
                L::Atomic(A::AP(0)),
                L::UnaryPrefix(U::Not),
                L::BinaryInfix(B::Until),
                L::Atomic(A::AP(1)),
                L::BinaryInfix(B::And),
                L::CloseParenthesis,
                L::UnaryPrefix(U::Next),
                L::Atomic(A::AP(0)),
                L::OpenParenthesis
            ]
        );
    }

    #[test]
    fn test_long_variables() {
        assert_eq!(
            lexer("aUntilB U a_until_b aUntilB").unwrap().0,
            vec![
                L::Atomic(A::AP(0)),
                L::BinaryInfix(B::Until),
                L::Atomic(A::AP(1)),
                L::Atomic(A::AP(0))
            ]
        );
    }

    #[test]
    fn test_invalid_chars() {
        assert_eq!(
            lexer("Zahl").unwrap_err().kind(),
            ErrorKind::UnexpectedToken
        );
        assert_eq!(
            lexer("a ? b").unwrap_err().kind(),
            ErrorKind::UnexpectedToken
        );
    }
}
