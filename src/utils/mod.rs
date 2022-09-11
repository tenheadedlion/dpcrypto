// the result of XORing two bits is the same as that of adding those two bits mod 2
pub fn xor(i: u32, j: u32) -> u32 {
    (i + j) % 2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_xor() {
        assert_eq!(xor(1, 1), 0);
        assert_eq!(xor(0, 1), 1);
        assert_eq!(xor(0, 0), 0);
        assert_eq!(0 ^ 0, 0);
        assert_eq!(0 ^ 1, 1);
        assert_eq!(1 ^ 1, 0);
    }
}
