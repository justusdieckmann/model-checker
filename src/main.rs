use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

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
    let hallo_du = "Na";

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

    return Result::Ok(tokens);
}

pub enum LTLFormulaBuilding {
    AP(u8),
    Not(Option<Box<LTLFormulaBuilding>>),
    And(Option<Box<LTLFormulaBuilding>>, Option<Box<LTLFormulaBuilding>>),
    Next(Option<Box<LTLFormulaBuilding>>),
    Until(Option<Box<LTLFormulaBuilding>>, Option<Box<LTLFormulaBuilding>>),
}

fn formula_building_for_token(token: LTLToken) -> Option<LTLFormulaBuilding> {
    return match token {
        LTLToken::AP(id) => Some(LTLFormulaBuilding::AP(id)),
        LTLToken::Not => Some(LTLFormulaBuilding::Not(None)),
        LTLToken::And => Some(LTLFormulaBuilding::And(None, None)),
        LTLToken::Next => Some(LTLFormulaBuilding::Next(None)),
        LTLToken::Until => Some(LTLFormulaBuilding::Until(None, None)),
        _ => None
    };
}

fn operator_precedence(operator: LTLToken) -> u32 {
    return match operator {
        LTLToken::AP(_) => 1000,
        LTLToken::Not => 800,
        LTLToken::Next => 600,
        LTLToken::And => 400,
        LTLToken::Until => 200,
        _ => 0,
    };
}

fn formula_building_to_formula(formula_building: &LTLFormulaBuilding) -> LTLFormula {
    return match formula_building {
        LTLFormulaBuilding::AP(id) => LTLFormula::AP(*id),
        LTLFormulaBuilding::Not(phi1) => LTLFormula::Not(Box::new(formula_building_to_formula(phi1.as_ref().unwrap().as_ref()))),
        LTLFormulaBuilding::And(phi1, phi2) => LTLFormula::And(Box::new(formula_building_to_formula(phi1.as_ref().unwrap().as_ref())), Box::new(formula_building_to_formula(phi2.as_ref().unwrap().as_ref()))),
        LTLFormulaBuilding::Next(phi1) => LTLFormula::Next(Box::new(formula_building_to_formula(phi1.as_ref().unwrap().as_ref()))),
        LTLFormulaBuilding::Until(phi1, phi2) => LTLFormula::Until(Box::new(formula_building_to_formula(phi1.as_ref().unwrap().as_ref())), Box::new(formula_building_to_formula(phi2.as_ref().unwrap().as_ref()))),
    };
}

pub fn parser(tokens: Vec<LTLToken>) -> Option<LTLFormula> {
    if tokens.is_empty() {
        return None;
    }

    let mut list = Vec::<LTLFormulaBuilding>::new();
    let mut root: Option<LTLFormulaBuilding> = None;

    for token in &tokens {
        match token {
            LTLToken::AP(_) => {}
            LTLToken::Not => {}
            LTLToken::And => {}
            LTLToken::Next => {}
            LTLToken::Until => {}
            LTLToken::OpenParenthesis => {}
            LTLToken::CloseParenthesis => {}
        }
    }

    return if root.is_some() {
        Some(formula_building_to_formula(&root.unwrap()))
    } else {
        None
    };
}

pub enum LTLFormula {
    AP(u8),
    Not(Box<LTLFormula>),
    And(Box<LTLFormula>, Box<LTLFormula>),
    Next(Box<LTLFormula>),
    Until(Box<LTLFormula>, Box<LTLFormula>),
}
