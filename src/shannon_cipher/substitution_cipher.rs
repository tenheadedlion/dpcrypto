/// # Substitution cipher
///
/// A substitution cipher is a shannon cipher epsilon = (E, D) of the following form:
/// Let sigma be a finite alphabet of symbols. The message space M and the ciphertext space C
/// are both sequences of symbols from sigma of some fixed length L:
/// M := C := sigma^L, that means the Message space is the same size as sigma^L, in plain English, that is,
///     every character from M comes from the finite alphabet of symbols, denoted as sigma^L
/// Think of K as a transpiler, that translate A to Z, and there is a reverse transpiler,
/// denoted as K^-1, translate Z back to A. Apply this back and forth mechanism on cipher methodology,
/// we have encryption as encrypt(K, plaintext), and decryption as decrypt(K^-1, plaintext)
///
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct Key {
    map: HashMap<u8, u8>,
    rev_map: HashMap<u8, u8>,
}

const ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

impl Key {
    /// Generate a key from the alphabet and dump the key into a string
    pub(self) fn gen() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut bytes = ALPHABET.to_string().into_bytes();
        bytes.shuffle(&mut rng);
        let perm = bytes.to_vec();
        let mut key = Key::default();
        _ = ALPHABET.as_bytes().iter().zip(perm).map(|(orig, curr)| {
            key.map.insert(*orig, curr);
            key.rev_map.insert(curr, *orig);
        });
        base64::encode(ron::to_string(&key).unwrap())
            .as_bytes()
            .to_vec()
    }
    /// restore the key from a string
    pub(self) fn restore(dump: &[u8]) -> Self {
        ron::from_str(&String::from_utf8_lossy(&base64::decode(dump).unwrap())).unwrap()
    }
}

pub fn gen_key() -> Vec<u8> {
    Key::gen()
}

pub fn encrypt(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let key = Key::restore(key);
    let mut res: Vec<u8> = Vec::new();
    for u in plaintext {
        match key.map.get(u) {
            Some(v) => res.push(*v),
            None => res.push(*u),
        }
    }
    res
}

pub fn decrypt(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
    let key = Key::restore(key);
    let mut res: Vec<u8> = Vec::new();
    for u in plaintext {
        match key.rev_map.get(u) {
            Some(v) => res.push(*v),
            None => res.push(*u),
        }
    }
    res
}
