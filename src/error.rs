use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The one-time pad shannon cipher requires keys, messages, and ciphertexts to be of the same length")]
    OneTimePadLengthViolation,
    #[error("The variable length one-time pad requires the length of the messages to be no more than that of the key")]
    MsgLengthViolationInVariableLengthOneTimePad
}

pub type Result<T> = std::result::Result<T, Error>;
