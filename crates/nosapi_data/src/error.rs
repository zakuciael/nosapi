use thiserror::Error;

#[derive(Error, Debug)]
#[error("{0}")]
pub(crate) struct ReadError(pub String);
