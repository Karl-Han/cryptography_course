use crate::lib::key_msg::Key;

pub trait Enc_Dec {
    fn encrypt(self, k: Key) -> Self;
    fn decrypt(self, k: Key) -> Self;
}

pub trait Encrypt {
    fn encrypt(self, k: Key) -> Self;
}

pub trait Decrypt {
    fn decrypt(self, k: Key) -> Self;
}
