mod parser;
mod lexer;

#[derive(PartialEq, Debug)]
pub enum LTLFormula {
    AP(u8),
    Not(Box<LTLFormula>),
    And(Box<LTLFormula>, Box<LTLFormula>),
    Next(Box<LTLFormula>),
    Until(Box<LTLFormula>, Box<LTLFormula>),
}

pub fn parse(text: &str) -> Result<LTLFormula, ()> {
    let tokens = lexer::lexer(text);
    if tokens.is_err() {
        return Err(());
    }
    let ast = parser::parser(tokens.unwrap());
    if ast.is_err() {
        return Err(());
    }
    return Ok(ast.unwrap());
}
