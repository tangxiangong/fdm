use thiserror::Error;

pub type FDMResult<T> = Result<T, FDMError>;

#[derive(Error, Debug)]
pub enum FDMError {
    #[error("Unknown Error")]
    Unknown,
}
