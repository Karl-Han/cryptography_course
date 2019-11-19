use crate::lib::{key_msg::Key, Algor::Enc_Dec};
use std::fs::File;
use std::io::{Read, Result, Write};

struct ECB_mode<T>
where
    T: Enc_Dec,
{
    reader: File,
    writer: File,
    method: T,
    key: Key,
}

impl<T> ECB_mode<T>
where
    T: Enc_Dec,
{
    pub fn new(method: T, key: Key, input: &str, output: &str) -> ECB_mode<T> {
        let mut input = File::open(input).expect("Error while opening input file");
        let mut output = File::create(output).expect("Error while opening output file");
        Self {
            reader: input,
            writer: output,
            method,
            key,
        }
    }

    pub fn build(mut self) {
        let mut buf = [0u8; 16];

        while let Ok(res) = self.reader.read(&mut buf) {
            if res < 16 {
                // it needs to be padded and get recorded
                // the higher bytes are paded with 0
                for i in res..16 {
                    buf[i] = 0;
                }
            } else {
                // the whole block
                let a = self.method.Self::new();
                //self.writer.write(&)
            }
        }
    }
}

struct CRT_mode {}
struct CBC_mode {}
