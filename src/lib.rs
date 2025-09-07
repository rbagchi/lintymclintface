use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyntaxError {
    pub line: usize,
    pub column: usize,
    pub message: String,
}

#[derive(Debug)]
pub enum LinterError {
    Io(String),
    Parse(String),
    TreeSitterParseError(String),
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
