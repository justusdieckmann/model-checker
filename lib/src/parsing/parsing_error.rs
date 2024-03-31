use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorKind {
    UnexpectedToken,
    ShittySyntax,
    UnmatchedOpenParenthesis,
    UnmatchedCloseParenthesis,
    EmptyParenthesis,
    NoAPs,
}

#[derive(Debug, PartialEq)]
pub struct ParsingError {
    kind: ErrorKind,
    formula: String,
    at: usize,
}

impl ParsingError {
    pub fn new(kind: ErrorKind, str: &str, at: Option<usize>) -> ParsingError {
        ParsingError {
            kind,
            formula: str.to_string(),
            at: at.unwrap_or(0),
        }
    }

    pub fn kind(&self) -> &ErrorKind {
        &self.kind
    }
}

impl Error for ParsingError {}

impl Display for ParsingError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let error_msg = match self.kind {
            ErrorKind::UnexpectedToken => "Unexpected token",
            ErrorKind::ShittySyntax => "Syntax error",
            ErrorKind::UnmatchedOpenParenthesis => "Unmatched open parenthesis",
            ErrorKind::UnmatchedCloseParenthesis => "Unmatched close parenthesis",
            ErrorKind::EmptyParenthesis => "Empty parenthesis",
            ErrorKind::NoAPs => "No atomic propositions",
        };
        write!(f, "{}", error_msg)
    }
}
