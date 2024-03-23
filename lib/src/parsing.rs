use std::collections::HashMap;
use crate::parsing::parsing_error::ParsingError;

mod parser;
mod lexer;
mod parsing_error;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum LTLFormula {
    AP(u8),
    Not(Box<LTLFormula>),
    And(Box<LTLFormula>, Box<LTLFormula>),
    Next(Box<LTLFormula>),
    Until(Box<LTLFormula>, Box<LTLFormula>),
}

pub fn parse(text: &str) -> Result<(LTLFormula, HashMap<String, u8>), ParsingError> {
    let (tokens, ap_map) = lexer::lexer(text)?;
    let ast = parser::parser(tokens)?;
    return Ok((ast, ap_map));
}
