use crate::lib::key_msg::Key;

pub trait EncDec {
    fn new(s: [u8; 16]) -> Self;

    fn encrypt(self, k: &Key) -> [u8; 16];
    fn decrypt(self, k: &Key) -> [u8; 16];

    fn msg(&self) -> [u8; 16];
}

pub trait Encrypt {
    fn encrypt(self, k: Key) -> Self;
}

pub trait Decrypt {
    fn decrypt(self, k: Key) -> Self;
}
