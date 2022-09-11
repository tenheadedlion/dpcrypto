#[test]
fn test_one_time_pad() {
    use dpcrypto::shannon_cipher::one_time_pad::*;
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
