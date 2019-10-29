extern crate num_bigint;

use num_bigint::{BigInt, Sign};
use std::fmt;
use std::str::FromStr;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub struct Cipher {
    // fixed length per element
    pub fragments: BigInt,
}

impl Cipher {
    pub fn new(s: &str) -> Cipher {
        let num = BigInt::from_str(s).expect("Unable to parse Cipher from str");
        //println!("Cipher {} with {} bits", num, num.bits());

        Cipher { fragments: num }
    }
}

impl fmt::Display for Cipher {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fragments)
    }
}

#[derive(Debug)]
pub struct Plaintext {
    pub fragments: BigInt,
}

impl Plaintext {
    pub fn new(s: &str) -> Self {
        Plaintext {
            fragments: BigInt::from_bytes_le(Sign::Plus, s.as_bytes()),
        }
    }
    pub fn into_string(&self) -> Result<String, FromUtf8Error> {
        let (_, bytes) = self.fragments.to_bytes_le();
        return String::from_utf8(bytes.to_vec());
    }
}

impl fmt::Display for Plaintext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fragments)
    }
}
