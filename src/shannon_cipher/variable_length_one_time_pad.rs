/// # A variable length one-time pad
///
/// A variable length one-time pad is a shannon cipher epsilon = (E, D), where the keys are
/// bit strings of some fixed length L, while messages and ciphertexts are variable length bit strings,
/// of length at most L

use crate::{Error, Result};
use crate::utils::xor;

/// Encrypts a plaintext with a key
pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    if key.len() < plaintext.len() {
        return Err(Error::MsgLengthViolationInVariableLengthOneTimePad);
    }
    // since the text is shorter than the key, we only use a fraction of the key;
    // in another words, the truncation of the key
    xor(&key[0..plaintext.len()], plaintext)
}

/// Decrypts a ciphertext with a key
pub fn decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    if key.len() < ciphertext.len() {
        return Err(Error::MsgLengthViolationInVariableLengthOneTimePad);
    }
    xor(&key[0..ciphertext.len()], ciphertext)
}
