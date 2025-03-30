#[derive(Debug)]
pub enum ErrorCode {
    FileNotFound,
    FileReadError,
    FileWriteError,
    UnsupportedObjectType,
    IntegerParseError,
    EncodingError,
    DecodingError,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::FileNotFound => write!(f, "File not found"),
            ErrorCode::FileReadError => write!(f, "Error reading file"),
            ErrorCode::FileWriteError => write!(f, "Error writing file"),
            ErrorCode::UnsupportedObjectType => write!(f, "Unsupported object type"),
            ErrorCode::IntegerParseError => write!(f, "Integer parse error"),
            ErrorCode::EncodingError => write!(f, "Encoding error"),
            ErrorCode::DecodingError => write!(f, "Decoding error"),
        }
    }
}
