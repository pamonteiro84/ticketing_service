#[derive(Debug, PartialEq)]
pub enum AppError {
    InvalidSeatRef(String),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidSeatRef(msg) => write!(f, "Invalid seat ref: {}", msg),
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;

