/// # Additive one-time pad
///
/// We may also define a "additive mod n" variation of the one-time pad;
/// This is a cipher epsilon = (E, D),
/// defined over (K, M, C) where K := M := C := {0,...,n-1},
/// where n is a positive integer.
use crate::{Error, Result};

/// Encrypts a plaintext with a key
///
/// Encryption is defined as follows:
/// E(k, m) := m + k mod n
pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    if key.len() != plaintext.len() {
        return Err(Error::OneTimePadLengthViolation);
    }
    Ok(key
        .iter()
        .zip(plaintext.iter())
        .map(|(k, m)| m + k % u8::MAX)
        .collect::<Vec<u8>>())
}

/// Decrypts a ciphertext with a key
///
/// Decryption is defined as follows:
/// D(k, m) := c - k mod n
pub fn decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    if key.len() != ciphertext.len() {
        return Err(Error::OneTimePadLengthViolation);
    }
    Ok(key
        .iter()
        .zip(ciphertext.iter())
        .map(|(k, c)| c - k % u8::MAX)
        .collect::<Vec<u8>>())
}
