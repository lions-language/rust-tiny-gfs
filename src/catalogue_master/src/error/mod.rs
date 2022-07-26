pub type Result<T> = anyhow::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io failed")]
    IoError(#[from] std::io::Error),
    #[error("innner error = {0}")]
    InnerError(String),
    #[error("{0} already exist")]
    AlreadyExist(String),
    #[error("{0} unimplemented")]
    Unimplemented(String),
}
