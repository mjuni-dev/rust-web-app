pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Debug)]
pub enum AppError {}

impl std::fmt::Display for AppError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "{self:?}")
    }
}

impl std::error::Error for AppError {}
