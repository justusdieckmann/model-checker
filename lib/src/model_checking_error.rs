use crate::parsing::parsing_error::ParsingError;
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum ModelCheckingErrorKind {
    ModelNoStart,
    ModelInvalid,
    FormulaNoAPs,
    FormulaSytaxError(ParsingError),
}

#[derive(Debug, PartialEq)]
pub struct ModelCheckingError {
    kind: ModelCheckingErrorKind,
}

impl ModelCheckingError {
    pub fn new(kind: ModelCheckingErrorKind) -> ModelCheckingError {
        ModelCheckingError { kind }
    }

    pub fn kind(&self) -> &ModelCheckingErrorKind {
        &self.kind
    }
}

impl Error for ModelCheckingError {}

impl Display for ModelCheckingError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let error_msg = match &self.kind {
            ModelCheckingErrorKind::ModelNoStart => "Model has no start",
            ModelCheckingErrorKind::ModelInvalid => "Model is invalid",
            ModelCheckingErrorKind::FormulaNoAPs => "Formula must contain an atomic proposition",
            ModelCheckingErrorKind::FormulaSytaxError(parse) => {
                return parse.fmt(f);
            }
        };
        write!(f, "{}", error_msg)
    }
}
