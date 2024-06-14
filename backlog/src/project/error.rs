use thiserror::Error;

#[derive(Debug, Error)]
#[error("Validation error: {}", .0)]
pub struct ValidationError(String);

impl ValidationError {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

#[derive(Debug, Error)]
#[error("Parsing error: {}", .0)]
pub struct ParsingError(String);

impl ParsingError {
    pub fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}
