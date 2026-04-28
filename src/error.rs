#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ReaderError {
    #[error("Tried to parse a number but found nothing")]
    NumberIsEmpty,
    #[error("{0}")]
    ParseError(String),
    #[error("Expected start of quote")]
    ExpectedStartOfQuote,
    #[error("Expected end of quote")]
    ExpectedEndOfQuote,
    #[error("Invalid escape")]
    InvalidEscape,
    #[error("Expected a bool but found nothing")]
    ExpectedBool,
    #[error("Invalid bool, must be either 'true' or 'false'")]
    InvalidBool(String),
    #[error("Expected a char but found nothing")]
    ExpectedChar,
    #[error("Found a closing char before an opening char")]
    FoundClosingBeforeOpening,
    #[error("Expected a closing char after an opening but no closing was found")]
    NoClosing,
}
