/// # One-time pad
///
/// A one-time pad is a shannon cipher epsilon = (E, D),
/// where the keys, messages, and ciphertexts are bit strings
/// of the same length
use crate::{Error, Result};
use crate::utils::xor;

/// Encrypts a plaintext with a key
pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    if key.len() != plaintext.len() {
        return Err(Error::OneTimePadLengthViolation);
    }
    xor(key, plaintext)
}

/// Decrypts a ciphertext with a key
pub fn decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    if key.len() != ciphertext.len() {
        return Err(Error::OneTimePadLengthViolation);
    }
    xor(key, ciphertext)
}
