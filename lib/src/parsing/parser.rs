use crate::parsing::LTLFormula;
use crate::parsing::lexer::{LTLToken, LTLTokenAtomic, LTLTokenBinaryInfix, LTLTokenUnaryPrefix};
use crate::parsing::parsing_error::{ErrorKind, ParsingError};
use crate::parsing::parsing_error::ErrorKind::ShittySyntax;

#[derive(Debug)]
enum LTLBinaryInfixKind {
    And,
    Or,
    Implies,
    Until,
    WeakUntil,
    Release
}

impl LTLBinaryInfixKind {
    fn operator_precedence(&self) -> u32 {
        return match self {
            LTLBinaryInfixKind::And => 800,
            LTLBinaryInfixKind::Or => 600,
            LTLBinaryInfixKind::Implies => 400,
            LTLBinaryInfixKind::Until |
            LTLBinaryInfixKind::WeakUntil |
            LTLBinaryInfixKind::Release => 200
        }
    }
}

#[derive(Debug)]
enum LTLUnaryPrefixKind {
    Not,
    Next,
    Future,
    Generally,
    Identity
}

impl LTLUnaryPrefixKind {
    fn operator_precedence(&self) -> u32 {
        return 1000;
    }
}

#[derive(Debug)]
enum LTLAtomicKind {
    AP(u8),
    True,
    False
}

#[derive(Debug)]
enum LTLFormulaBuilding {
    BinaryInfix(LTLBinaryInfixKind, Option<Box<LTLFormulaBuilding>>, Option<Box<LTLFormulaBuilding>>),
    UnaryPrefix(LTLUnaryPrefixKind, Option<Box<LTLFormulaBuilding>>),
    Atomics(LTLAtomicKind)
}

impl LTLFormulaBuilding {

    fn get_right_child_mut(&mut self) -> Option<&mut LTLFormulaBuilding> {
        return match self {
            LTLFormulaBuilding::BinaryInfix(_, _, Some(phi)) |
            LTLFormulaBuilding::UnaryPrefix(_, Some(phi)) =>  Some(phi),
            _ => None
        }
    }

    fn get_right_child(&self) -> Option<&LTLFormulaBuilding> {
        return match self {
            LTLFormulaBuilding::BinaryInfix(_, _, Some(phi)) |
            LTLFormulaBuilding::UnaryPrefix(_, Some(phi)) =>  Some(phi),
            _ => None
        }
    }

    fn get_rightmost_leaf_mut(&mut self) -> &mut LTLFormulaBuilding {
        let mut current = self;
        loop {
            let next = current.get_right_child();

            if next.is_none() {
                return current;
            }
            current = current.get_right_child_mut().unwrap();
        }
    }

    fn get_right_content_mut(&mut self) -> Result<&mut Option<Box<LTLFormulaBuilding>>, ()> {
        return match self {
            LTLFormulaBuilding::BinaryInfix(_, _, ref mut phi) |
            LTLFormulaBuilding::UnaryPrefix(_, ref mut phi) => Ok(phi),
            LTLFormulaBuilding::Atomics(_) => Err(())
        }
    }

    fn add(&mut self, other: LTLFormulaBuilding) -> Result<(), ()> {
        return match self {
            LTLFormulaBuilding::BinaryInfix(_, _, ref mut phi) |
            LTLFormulaBuilding::UnaryPrefix(_, ref mut phi) => {
                if phi.is_some() {
                    return Err(());
                }
                *phi = Some(Box::new(other));
                Ok(())
            },
            _ => Err(())
        };
    }

    fn get_highest_node_with_prio_greater_than(&mut self, prio: u32) -> &mut LTLFormulaBuilding {
        let mut current = self;
        loop {
            let next = current.get_right_child();

            if next.is_none() {
                return current;
            }
            let next_content = next.unwrap();
            if next_content.operator_precedence() >= prio {
                return current;
            }
            current = current.get_right_child_mut().unwrap();
        }
    }

    fn operator_precedence(&self) -> u32 {
        return match self {
            LTLFormulaBuilding::BinaryInfix(kind, _, _) => kind.operator_precedence(),
            LTLFormulaBuilding::UnaryPrefix(kind, _) => kind.operator_precedence(),
            LTLFormulaBuilding::Atomics(_) => 1200
        };
    }

    fn to_formula(&self) -> Result<LTLFormula, ()> {
        return Ok(match self {
            LTLFormulaBuilding::BinaryInfix(LTLBinaryInfixKind::And, Some(phi1), Some(phi2)) =>
                LTLFormula::and(phi1.to_formula()?, phi2.to_formula()?),
            LTLFormulaBuilding::BinaryInfix(LTLBinaryInfixKind::Or, Some(phi1), Some(phi2)) =>
                LTLFormula::not(LTLFormula::and(
                    LTLFormula::not(phi1.to_formula()?),
                    LTLFormula::not(phi2.to_formula()?),
                )),
            LTLFormulaBuilding::BinaryInfix(LTLBinaryInfixKind::Implies, Some(phi1), Some(phi2)) =>
                LTLFormula::not(LTLFormula::and(
                    phi1.to_formula()?,
                    LTLFormula::not(phi2.to_formula()?),
                )),
            LTLFormulaBuilding::BinaryInfix(LTLBinaryInfixKind::Until, Some(phi1), Some(phi2)) =>
                LTLFormula::until(phi1.to_formula()?, phi2.to_formula()?, false),
            LTLFormulaBuilding::BinaryInfix(LTLBinaryInfixKind::WeakUntil, Some(phi1), Some(phi2)) =>
                LTLFormula::until(phi1.to_formula()?, phi2.to_formula()?, true),
            LTLFormulaBuilding::BinaryInfix(LTLBinaryInfixKind::Release, Some(phi1), Some(phi2)) =>
                LTLFormula::not(LTLFormula::until(
                    LTLFormula::not(phi1.to_formula()?),
                    LTLFormula::not(phi2.to_formula()?),
                    false
                )),
            LTLFormulaBuilding::UnaryPrefix(LTLUnaryPrefixKind::Not, Some(phi)) =>
                LTLFormula::not(phi.to_formula()?),
            LTLFormulaBuilding::UnaryPrefix(LTLUnaryPrefixKind::Next, Some(phi)) =>
                LTLFormula::next(phi.to_formula()?),
            LTLFormulaBuilding::UnaryPrefix(LTLUnaryPrefixKind::Future, Some(phi)) =>
                LTLFormula::until(LTLFormula::not(LTLFormula::and(LTLFormula::ap(0), LTLFormula::not(LTLFormula::ap(0)))), phi.to_formula()?, false),
            LTLFormulaBuilding::UnaryPrefix(LTLUnaryPrefixKind::Generally, Some(phi)) =>
                LTLFormula::not(LTLFormula::until(LTLFormula::not(LTLFormula::and(LTLFormula::ap(0), LTLFormula::not(LTLFormula::ap(0)))), LTLFormula::not(phi.to_formula()?), false)),
            LTLFormulaBuilding::UnaryPrefix(LTLUnaryPrefixKind::Identity, Some(phi)) => phi.to_formula()?,
            LTLFormulaBuilding::Atomics(LTLAtomicKind::AP(ap)) => LTLFormula::ap(*ap),
            LTLFormulaBuilding::Atomics(LTLAtomicKind::False) => LTLFormula::and(LTLFormula::ap(0), LTLFormula::not(LTLFormula::ap(0))),
            LTLFormulaBuilding::Atomics(LTLAtomicKind::True) => LTLFormula::not(LTLFormula::and(LTLFormula::ap(0), LTLFormula::not(LTLFormula::ap(0)))),
            _ => return Err(())
        });
    }
}

pub fn parser(tokens: Vec<LTLToken>) -> Result<LTLFormula, ParsingError> {
    let mut current = vec![LTLFormulaBuilding::UnaryPrefix(LTLUnaryPrefixKind::Identity, None)];

    for token in &tokens {
        match token {
            LTLToken::Atomic(atomic) => {
                let to_insert = match atomic {
                    LTLTokenAtomic::AP(ap) => LTLAtomicKind::AP(*ap),
                    LTLTokenAtomic::True => LTLAtomicKind::True,
                    LTLTokenAtomic::False => LTLAtomicKind::False
                };
                current.last_mut().unwrap().get_rightmost_leaf_mut().add(LTLFormulaBuilding::Atomics(to_insert)).map_err(|_| {
                    ParsingError::new(ShittySyntax, "", None)
                })?;
            }
            LTLToken::UnaryPrefix(unary_prefix) => {
                let to_insert = match unary_prefix {
                    LTLTokenUnaryPrefix::Next => LTLUnaryPrefixKind::Next,
                    LTLTokenUnaryPrefix::Not => LTLUnaryPrefixKind::Not,
                    LTLTokenUnaryPrefix::Future => LTLUnaryPrefixKind::Future,
                    LTLTokenUnaryPrefix::Generally => LTLUnaryPrefixKind::Generally,
                };
                current.last_mut().unwrap().get_rightmost_leaf_mut().add(LTLFormulaBuilding::UnaryPrefix(to_insert, None)).map_err(|_| {
                    ParsingError::new(ShittySyntax, "", None)
                })?;
            }
            LTLToken::BinaryInfix(binary_infix) => {
                let to_insert = match binary_infix {
                    LTLTokenBinaryInfix::And => LTLBinaryInfixKind::And,
                    LTLTokenBinaryInfix::Or => LTLBinaryInfixKind::Or,
                    LTLTokenBinaryInfix::Implies => LTLBinaryInfixKind::Implies,
                    LTLTokenBinaryInfix::Until => LTLBinaryInfixKind::Until,
                    LTLTokenBinaryInfix::WeakUntil => LTLBinaryInfixKind::WeakUntil,
                    LTLTokenBinaryInfix::Release => LTLBinaryInfixKind::Release
                };
                let parent = current.last_mut().unwrap().get_highest_node_with_prio_greater_than(to_insert.operator_precedence());
                let right_side = parent.get_right_content_mut().map_err(|_| {
                    ParsingError::new(ShittySyntax, "", None)
                })?;
                let right_side_value = right_side.take();
                parent.add(LTLFormulaBuilding::BinaryInfix(to_insert, right_side_value, None)).map_err(|_| {
                    ParsingError::new(ShittySyntax, "", None)
                })?;
            }
            LTLToken::OpenParenthesis => {
                current.push(LTLFormulaBuilding::UnaryPrefix(LTLUnaryPrefixKind::Identity, None));
            }
            LTLToken::CloseParenthesis => {
                if current.len() <= 1 {
                    return Err(ParsingError::new(ErrorKind::UnmatchedCloseParenthesis, "", None));
                }
                let mut last = current.pop().unwrap();
                if let LTLFormulaBuilding::UnaryPrefix(LTLUnaryPrefixKind::Identity, None) = last.get_rightmost_leaf_mut() {
                    return Err(ParsingError::new(ErrorKind::EmptyParenthesis, "", None));
                }
                let _ = current.last_mut().unwrap().get_rightmost_leaf_mut().add(last);
            }
        }
    }

    if current.len() > 1 {
        return Err(ParsingError::new(ErrorKind::UnmatchedOpenParenthesis, "", None));
    }

    return if let Some(first) = current.first() {
        first.to_formula().map_err(|_| ParsingError::new(ShittySyntax, "", None))
    } else {
        Err(ParsingError::new(ErrorKind::ShittySyntax, "", None))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use LTLToken as L;
    use LTLTokenAtomic as A;
    use LTLTokenUnaryPrefix as U;
    use LTLTokenBinaryInfix as B;
    use LTLFormula as F;

    #[test]
    fn test_basic_parsing() {
        assert_eq!(parser(vec![L::UnaryPrefix(U::Next), L::Atomic(A::AP(0)), L::BinaryInfix(B::And), L::Atomic(A::AP(1)), L::BinaryInfix(B::Until), L::UnaryPrefix(U::Not), L::Atomic(A::AP(0))]),
                   Ok(F::until(
                       F::and(
                           F::next(F::ap(0)),
                           F::ap(1),
                       ),
                       F::not(F::ap(0)),
                   false))
        );
    }

    #[test]
    fn test_proper_errors() {
        assert_eq!(parser(vec![]).unwrap_err().kind(), ShittySyntax);
        assert_eq!(parser(vec![L::Atomic(A::AP(0)), L::Atomic(A::AP(1))]).unwrap_err().kind(), ShittySyntax);
        assert_eq!(parser(vec![L::BinaryInfix(B::Until), L::Atomic(A::AP(0))]).unwrap_err().kind(), ShittySyntax);
        assert_eq!(parser(vec![L::Atomic(A::AP(0)), L::UnaryPrefix(U::Next)]).unwrap_err().kind(), ShittySyntax);
        assert_eq!(parser(vec![L::OpenParenthesis, L::OpenParenthesis, L::Atomic(A::AP(0)), L::CloseParenthesis])
                       .unwrap_err().kind(), ErrorKind::UnmatchedOpenParenthesis);
        assert_eq!(parser(vec![L::OpenParenthesis, L::Atomic(A::AP(0)), L::CloseParenthesis, L::CloseParenthesis])
                       .unwrap_err().kind(), ErrorKind::UnmatchedCloseParenthesis);
        assert_eq!(parser(vec![L::CloseParenthesis, L::Atomic(A::AP(0)), L::OpenParenthesis])
                       .unwrap_err().kind(), ErrorKind::UnmatchedCloseParenthesis);
        assert_eq!(parser(vec![L::Atomic(A::AP(0)), L::BinaryInfix(B::And), L::OpenParenthesis, L::CloseParenthesis]).unwrap_err().kind(),
                   ErrorKind::EmptyParenthesis)
    }

}