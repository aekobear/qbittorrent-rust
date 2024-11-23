use std::fmt::{Display, Formatter};

use super::error_type::ErrorType;

#[derive(Debug)]
pub struct Error {
    pub err_type: ErrorType,
    pub message: String,
    pub code: Option<u16>
} impl Error {
    pub(crate) fn build(err_type: ErrorType, code: Option<u16>) -> Error {
        let message = err_type.get_message();
        Error { err_type, message, code }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}


impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.err_type {
            ErrorType::JsonSerdeError(e) => Some(e.as_ref()),
            ErrorType::ReqwestError(e) => Some(e.as_ref()),
            _ => None
        }
    }

    fn description(&self) -> &str {
        "description() is deprecated; use Display."
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}