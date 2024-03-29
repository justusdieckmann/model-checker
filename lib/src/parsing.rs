use std::collections::HashMap;
use crate::parsing::parsing_error::{ErrorKind, ParsingError};

mod parser;
mod lexer;
mod parsing_error;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LTLFormula {
    AP(u8),
    Not(Box<LTLFormula>),
    And(Box<LTLFormula>, Box<LTLFormula>),
    Next(Box<LTLFormula>),
    Until(bool, Box<LTLFormula>, Box<LTLFormula>),
}

impl LTLFormula {

    pub fn ap(ap: u8) -> Self {
        return Self::AP(ap)
    }

    pub fn not(phi: LTLFormula) -> Self {
        return Self::Not(Box::new(phi))
    }

    pub fn and(phi1: LTLFormula, phi2: LTLFormula) -> Self {
        return Self::And(Box::new(phi1), Box::new(phi2))
    }

    pub fn next(phi: LTLFormula) -> Self {
        return Self::Next(Box::new(phi))
    }

    pub fn until(phi1: LTLFormula, phi2: LTLFormula, weak: bool) -> Self {
        return Self::Until(weak, Box::new(phi1), Box::new(phi2))
    }

}

pub fn parse(text: &str) -> Result<(LTLFormula, HashMap<String, u8>), ParsingError> {
    let (tokens, ap_map) = lexer::lexer(text)?;
    if ap_map.is_empty() {
        return Err(ParsingError::new(ErrorKind::NoAPs, "", None));
    }
    let ast = parser::parser(tokens)?;
    return Ok((ast, ap_map));
}

#[cfg(test)]
mod tests {
    use super::*;
}