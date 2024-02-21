use crate::parsing::LTLFormula;
use crate::parsing::lexer::LTLToken;
use crate::parsing::parsing_error::{ErrorKind, ParsingError};
use crate::parsing::parsing_error::ErrorKind::ShittySyntax;

const OPERATOR_PRIO_AP: u32 = 1000;
const OPERATOR_PRIO_NOT: u32 = 800;
const OPERATOR_PRIO_NEXT: u32 = 600;
const OPERATOR_PRIO_AND: u32 = 400;
const OPERATOR_PRIO_UNTIL: u32 = 200;

#[derive(Debug)]
enum LTLFormulaBuilding {
    AP(u8),
    Not(Option<Box<LTLFormulaBuilding>>),
    And(Option<Box<LTLFormulaBuilding>>, Option<Box<LTLFormulaBuilding>>),
    Next(Option<Box<LTLFormulaBuilding>>),
    Until(Option<Box<LTLFormulaBuilding>>, Option<Box<LTLFormulaBuilding>>),
    Identity(Option<Box<LTLFormulaBuilding>>)
}

impl LTLFormulaBuilding {
    fn add(&mut self, other: LTLFormulaBuilding) -> Result<(), ()> {
        match self {
            LTLFormulaBuilding::AP(_) => {return Err(())}
            LTLFormulaBuilding::Not(ref mut phi) |
            LTLFormulaBuilding::And(_, ref mut phi) |
            LTLFormulaBuilding::Next(ref mut phi) |
            LTLFormulaBuilding::Until(_, ref mut phi) |
            LTLFormulaBuilding::Identity(ref mut phi) => {
                if phi.is_some() {
                    return Err(());
                }
                *phi = Some(Box::new(other))
            }
        }
        return Ok(());
    }

    fn get_rightmost_leaf(&mut self) -> &mut LTLFormulaBuilding {
        return match self {
            LTLFormulaBuilding::Not(Some(phi)) => phi.get_rightmost_leaf(),
            LTLFormulaBuilding::And(_, Some(phi)) => phi.get_rightmost_leaf(),
            LTLFormulaBuilding::Next(Some(phi)) => phi.get_rightmost_leaf(),
            LTLFormulaBuilding::Until(_, Some(phi)) => phi.get_rightmost_leaf(),
            LTLFormulaBuilding::Identity(Some(phi)) => phi.get_rightmost_leaf(),
            _ => self
        }
    }

    fn get_highest_node_with_prio_greater_than(&mut self, prio: u32) -> &mut LTLFormulaBuilding {
        let mut current = self;
        loop {
            let next = current.get_right_child();

            if next.is_none() {
                return current;
            }
            let next_content = next.unwrap();
            if next_content.operator_prio() >= prio {
                return current;
            }
            current = current.get_right_child_mut().unwrap();
        }
    }

    fn get_right_child_mut(&mut self) -> Option<&mut LTLFormulaBuilding> {
        return match self {
            LTLFormulaBuilding::Not(Some(phi)) |
            LTLFormulaBuilding::And(_, Some(phi)) |
            LTLFormulaBuilding::Next(Some(phi)) |
            LTLFormulaBuilding::Until(_, Some(phi)) |
            LTLFormulaBuilding::Identity(Some(phi)) => Some(phi),
            _ => None
        }
    }

    fn get_right_child(&self) -> Option<&LTLFormulaBuilding> {
        return match self {
            LTLFormulaBuilding::Not(Some(phi)) |
            LTLFormulaBuilding::And(_, Some(phi)) |
            LTLFormulaBuilding::Next(Some(phi)) |
            LTLFormulaBuilding::Until(_, Some(phi)) |
            LTLFormulaBuilding::Identity(Some(phi)) => Some(phi),
            _ => None
        }
    }

    fn get_right_content_mut(&mut self) -> Result<&mut Option<Box<LTLFormulaBuilding>>, ()> {
        return match self {
            LTLFormulaBuilding::AP(_) => { Err(()) }
            LTLFormulaBuilding::Not(ref mut phi) |
            LTLFormulaBuilding::And(_, ref mut phi) |
            LTLFormulaBuilding::Next(ref mut phi) |
            LTLFormulaBuilding::Until(_, ref mut phi) |
            LTLFormulaBuilding::Identity(ref mut phi) => { Ok(phi) }
        }
    }

    fn operator_prio(&self) -> u32 {
        return match self {
            LTLFormulaBuilding::AP(_) => OPERATOR_PRIO_AP,
            LTLFormulaBuilding::Not(_) => OPERATOR_PRIO_NOT,
            LTLFormulaBuilding::And(_, _) => OPERATOR_PRIO_AND,
            LTLFormulaBuilding::Next(_) => OPERATOR_PRIO_NEXT,
            LTLFormulaBuilding::Until(_, _) => OPERATOR_PRIO_UNTIL,
            LTLFormulaBuilding::Identity(_) => 9999
        };
    }

    fn to_formula(&self) -> Result<LTLFormula, ()> {
        return Ok(match self {
            LTLFormulaBuilding::AP(id) => LTLFormula::AP(*id),
            LTLFormulaBuilding::Not(Some(phi1)) => LTLFormula::Not(Box::new(phi1.as_ref().to_formula()?)),
            LTLFormulaBuilding::And(Some(phi1), Some(phi2)) => LTLFormula::And(Box::new(phi1.as_ref().to_formula()?), Box::new(phi2.as_ref().to_formula()?)),
            LTLFormulaBuilding::Next(Some(phi1)) => LTLFormula::Next(Box::new(phi1.as_ref().to_formula()?)),
            LTLFormulaBuilding::Until(Some(phi1), Some(phi2)) => LTLFormula::Until(Box::new(phi1.as_ref().to_formula()?), Box::new(phi2.as_ref().to_formula()?)),
            LTLFormulaBuilding::Identity(Some(phi1)) => phi1.as_ref().to_formula()?,
            _ => {return Err(());}
        });
    }
}

pub fn parser(tokens: Vec<LTLToken>) -> Result<LTLFormula, ParsingError> {
    let mut current = vec![LTLFormulaBuilding::Identity(None)];

    for token in &tokens {
        match token {
            LTLToken::AP(id) => {
                current.last_mut().unwrap().get_rightmost_leaf().add(LTLFormulaBuilding::AP(*id)).map_err(|_| {
                    ParsingError::new(ShittySyntax, "", None)
                })?;
            }
            LTLToken::Not => {
                current.last_mut().unwrap().get_rightmost_leaf().add(LTLFormulaBuilding::Not(None)).map_err(|_| {
                    ParsingError::new(ShittySyntax, "", None)
                })?;
            }
            LTLToken::Next => {
                current.last_mut().unwrap().get_rightmost_leaf().add(LTLFormulaBuilding::Next(None)).map_err(|_| {
                    ParsingError::new(ShittySyntax, "", None)
                })?;
            }
            LTLToken::And => {
                let parent_node = current.last_mut().unwrap().get_highest_node_with_prio_greater_than(OPERATOR_PRIO_AND).get_right_content_mut().unwrap();
                let old = parent_node.replace(Box::new(LTLFormulaBuilding::And(None, None)));
                if let LTLFormulaBuilding::And(ref mut new, _) = *parent_node.as_mut().unwrap().as_mut() {
                    *new = old;
                } else {
                    panic!("at the disco!");
                }
            }
            LTLToken::Until => {
                let option = current.last_mut().unwrap().get_right_content_mut().unwrap();
                let old = option.replace(Box::new(LTLFormulaBuilding::Until(None, None)));
                if let LTLFormulaBuilding::Until(ref mut new, _) = *option.as_mut().unwrap().as_mut() {
                    *new = old;
                } else {
                    panic!("at the disco!");
                }
            }
            LTLToken::OpenParenthesis => {
                current.push(LTLFormulaBuilding::Identity(None));
            }
            LTLToken::CloseParenthesis => {
                if current.len() <= 1 {
                    return Err(ParsingError::new(ErrorKind::UnmatchedCloseParenthesis, "", None));
                }
                let mut last = current.pop().unwrap();
                if let LTLFormulaBuilding::Identity(None) = last.get_rightmost_leaf() {
                    return Err(ParsingError::new(ErrorKind::EmptyParenthesis, "", None));
                }
                let _ = current.last_mut().unwrap().get_rightmost_leaf().add(last);
            }
        }
    }

    if current.len() > 1 {
        return Err(ParsingError::new(ErrorKind::UnmatchedOpenParenthesis, "", None));
    }

    return if current.first().is_some() {
        current.first().unwrap().to_formula().map_err(|_| ParsingError::new(ShittySyntax, "", None))
    } else {
        Err(ParsingError::new(ErrorKind::ShittySyntax, "", None))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use LTLToken as T;
    use LTLFormula as F;

    #[test]
    fn test_basic_parsing() {
        assert_eq!(parser(vec![T::Next, T::AP(0), T::And, T::AP(1), T::Until, T::Not, T::AP(0)]),
                   Ok(F::Until(
                       Box::new(F::And(
                           Box::new(F::Next(Box::new(F::AP(0)))),
                           Box::new(F::AP(1)),
                       )),
                       Box::new(F::Not(Box::new(F::AP(0)))),
                   ))
        );
    }

    #[test]
    fn test_proper_errors() {
        assert_eq!(parser(vec![]).unwrap_err().kind(), ShittySyntax);
        assert_eq!(parser(vec![T::AP(0), T::AP(1)]).unwrap_err().kind(), ShittySyntax);
        assert_eq!(parser(vec![T::Until, T::AP(0)]).unwrap_err().kind(), ShittySyntax);
        assert_eq!(parser(vec![T::AP(0), T::Next]).unwrap_err().kind(), ShittySyntax);
        assert_eq!(parser(vec![T::OpenParenthesis, T::OpenParenthesis, T::AP(0), T::CloseParenthesis])
                       .unwrap_err().kind(), ErrorKind::UnmatchedOpenParenthesis);
        assert_eq!(parser(vec![T::OpenParenthesis, T::AP(0), T::CloseParenthesis, T::CloseParenthesis])
                       .unwrap_err().kind(), ErrorKind::UnmatchedCloseParenthesis);
        assert_eq!(parser(vec![T::CloseParenthesis, T::AP(0), T::OpenParenthesis])
                       .unwrap_err().kind(), ErrorKind::UnmatchedCloseParenthesis);
        assert_eq!(parser(vec![T::AP(0), T::And, T::OpenParenthesis, T::CloseParenthesis]).unwrap_err().kind(),
                   ErrorKind::EmptyParenthesis)
    }

}