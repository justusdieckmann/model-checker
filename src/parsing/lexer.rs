use std::collections::HashMap;

pub enum LTLToken {
    AP(u8),
    Not,
    And,
    Next,
    Until,
    OpenParenthesis,
    CloseParenthesis,
}

pub fn lexer(text: &str) -> Result<Vec<LTLToken>, usize> {
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
                _ => return Result::Err(i)
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