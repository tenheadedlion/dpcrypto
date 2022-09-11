/// # One-time pad
///
/// A one-time pad is a shannon cipher epsilon = (E, D),
/// where the keys, messages, and ciphertexts are bit strings
/// of the same length
struct OneTimePad;

#[derive(Debug)]
struct Error;
type Result<T> = std::result::Result<T, Error>;

/// Bitwise operation between plaintext and key
///
/// # Arguments
///
/// The arguments should have the same length
fn xor(key: &[u8], msg: &[u8]) -> Result<Vec<u8>> {
    Ok(key.iter().zip(msg).map(|(k, m)| k ^ m).collect::<Vec<u8>>())
}

fn encrypt(key: &[u8], plaintext: &[u8]) -> Result<Vec<u8>> {
    xor(key, plaintext)
}

fn decrypt(key: &[u8], ciphertext: &[u8]) -> Result<Vec<u8>> {
    xor(key, ciphertext)
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn it_works() {
        let key = "abcdef";
        let plaintext = "hello!";
        // test the "correctness property"
        assert_eq!(
            &decrypt(
                key.as_bytes(),
                &encrypt(key.as_bytes(), plaintext.as_bytes()).unwrap()
            )
            .unwrap(),
            plaintext.as_bytes()
        );
    }
}
