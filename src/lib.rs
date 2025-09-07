use serde::{Deserialize, Serialize};

/// Represents a syntax error found by the linter.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyntaxError {
    /// The line number where the error occurred (1-based).
    pub line: usize,
    /// The column number where the error occurred (1-based).
    pub column: usize,
    /// A descriptive message about the error.
    pub message: String,
}

/// Represents various errors that can occur during the linting process.
#[derive(Debug)]
pub enum LinterError {
    /// An I/O error, typically when reading a file.
    Io(String),
    /// A parsing error, indicating an issue with the input code's structure.
    Parse(String),
    /// An error specifically from the Tree-sitter parser.
    TreeSitterParseError(String),
    /// An error indicating that the requested language is not supported by the linter.
    UnsupportedLanguage(String),
}

impl From<std::io::Error> for LinterError {
    fn from(err: std::io::Error) -> Self {
        LinterError::Io(err.to_string())
    }
}

impl From<LinterError> for std::io::Error {
    fn from(err: LinterError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, format!("{:?}", err))
    }
}