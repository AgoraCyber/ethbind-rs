use thiserror::Error;

#[derive(Debug, Error)]
pub enum AbiJsonError {
    #[error("Invalid fixed type declare {0}, {1}")]
    FixedMN(String, String),

    #[error("Invalid integer type declare {0}, {1}")]
    IntegerM(String, String),

    #[error("Invalid fixed length binary type declare {0}, {1}")]
    BytesM(String, String),

    #[error("Invalid tuple type declare {0}, {1}")]
    Tuple(String, String),

    #[error("Invalid fixed-length Array type declare {0}, {1}")]
    ArrayM(String, String),

    #[error("Invalid Array type declare {0}, {1}")]
    Array(String, String),

    #[error("Invalid Type declare {0}")]
    UnknownType(String),
}
