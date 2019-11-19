use crate::lib::{key_msg::Key, Algor::EncDec};
use std::fs::File;
use std::io::{Read, Result, Write};
use std::mem::replace;

pub trait ECB_mode {
    type T: EncDec;
    //type U: AsMut<[u8]> + Default;

    fn ecb_build(key: Key, input: &str, output: &str) {
        let mut input = File::open(input).expect("Error while opening input file");
        let mut output = File::create(output).expect("Error while opening output file");
        let mut buf = [0u8; 16];
        let mut temp_file = File::create("temp.plain").expect("Error while opening output file");

        while let Ok(res) = input.read(&mut buf) {
            if res < 16 {
                // it needs to be padded and get recorded
                // the higher bytes are paded with 0
                for i in res..16 {
                    buf[i] = 0;
                }
                let a = Self::T::new(buf);
                temp_file.write(&a.msg());
                output.write(&a.encrypt(&key));

                // record the length in little endian
                // use slice to recover
                let mut temp = res.to_le_bytes();
                //replace(&mut buf, temp);
                for i in 0..8 {
                    buf[i] = temp[i];
                }
                for i in 8..16 {
                    buf[i] = 0;
                }
                let a = Self::T::new(buf);
                temp_file.write(&a.msg());
                output.write(&a.encrypt(&key));
                break;
            } else {
                // the whole block
                let a = Self::T::new(buf);
                temp_file.write(&a.msg());
                output.write(&a.encrypt(&key));
            }
        }
    }
}

trait CRT_mode {}
trait CBC_mode {}
