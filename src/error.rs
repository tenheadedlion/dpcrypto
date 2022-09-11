use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("A shannon cipher requires keys, messages, and ciphertexts to be of the same length")]
    OneTimePadLengthViolation,
}

pub type Result<T> = std::result::Result<T, Error>;
