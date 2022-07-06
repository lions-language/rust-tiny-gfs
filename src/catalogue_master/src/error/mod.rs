pub(crate) type Result<T> = anyhow::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io failed")]
    IoError(#[from] std::io::Error),
    #[error("innner error = {0}")]
    InnerError(String),
}
