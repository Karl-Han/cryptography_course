use crate::lib::{key_msg::Key, Algor::EncDec};
use rand::RngCore;
use std::convert::TryInto;
use std::fs::{File, OpenOptions};
use std::io::{Read, Result, Write};
use std::mem::replace;

pub fn xor(mut a: [u8; 16], b: &[u8; 16]) -> [u8; 16] {
    for i in 0..16 {
        a[i] ^= b[i];
    }
    a
}

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
                println!("last block length = {}", res);
                let a = Self::T::new(buf);
                temp_file.write(&a.msg());
                output.write(&a.encrypt(&key));

                // record the length in little endian
                // use slice to recover
                let mut temp = res.to_le_bytes();
                println!("temp in bytes = {:02x?}", temp);
                //replace(&mut buf, temp);
                for i in 0..8 {
                    buf[i] = temp[i];
                }
                for i in 8..16 {
                    buf[i] = 0;
                }
                println!("buf of last block length = {:02x?}", buf);
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

    // used to destruct the cipher with key
    fn ecb_destruct(key: Key, input: &str, output: &str) {
        let mut input = File::open(input).expect("Error while opening input file");
        let mut output = OpenOptions::new()
            .write(true)
            .create(true)
            .open(output)
            .expect("Error while opening output file");
        let mut buf = [0u8; 16];
        let mut buf_next = [0u8; 16];
        // counter is one less then the exact segment we read
        let mut counter: u64 = 0;
        //let mut flag: bool = true;

        let res = input.read(&mut buf_next);
        if let Ok(_) = res {
            buf = buf_next;
        } else {
            panic!("ERROR, file is broken with wrong length");
        }

        while let Ok(res) = input.read(&mut buf_next) {
            if res == 16 {
                // the whole block
                let a = Self::T::new(buf);
                output.write(&a.decrypt(&key));
                buf = buf_next;
                counter += 1;
            } else if res == 0 {
                // now buf store the length of the last segment
                println!("last buf = {:02x?}", buf);
                println!("counter = {}, {} bytes total", counter, counter * 16);
                let a = Self::T::new(buf);
                let last_length = u64::from_le_bytes(
                    a.decrypt(&key)[..8]
                        .try_into()
                        .expect("Wrong length in the last length"),
                );
                println!("last_length = {}", last_length);
                let total_length = (counter - 1) * 16 + last_length;
                println!("total length = {}", total_length);
                output
                    .set_len(total_length)
                    .expect("Error when set length of file");
                break;
            } else {
                panic!("ERROR, file is broken with wrong length");
            }
        }
    }
}

pub trait CTR_mode {
    type T: EncDec;
    //type U: AsMut<[u8]> + Default;

    // the main difference is that it start with a random number
    fn ctr_build(key: Key, input: &str, output: &str) {
        let mut input = File::open(input).expect("Error while opening input file");
        let mut output = File::create(output).expect("Error while opening output file");
        let mut buf = [0u8; 16];
        let mut temp_file = File::create("temp.plain").expect("Error while opening output file");

        let mut rng = rand::thread_rng();
        let mut arr = [0u8; 16];
        rng.fill_bytes(&mut arr);
        arr[arr.len() - 1] = 0;
        arr[0] = 0;
        let mut nounce: u128 = u128::from_le_bytes(arr);
        println!("arr = {:02x?}", arr);
        println!("nounce = {}", nounce);

        let mut f = File::create("nounce").expect("Failed to save nonce");
        // it store the nounce in little endian
        f.write(&arr);

        while let Ok(res) = input.read(&mut buf) {
            if res < 16 {
                // it needs to be padded and get recorded
                // the higher bytes are paded with 0
                for i in res..16 {
                    buf[i] = 0;
                }
                println!("last block length = {}", res);

                // difference emerge here from ecb
                let a = Self::T::new(nounce.to_le_bytes());
                nounce += 1;
                temp_file.write(&a.msg());
                let out = xor(a.encrypt(&key), (&buf));
                output.write(&out);

                // record the length in little endian
                // use slice to recover
                let mut temp = res.to_le_bytes();
                println!("temp in bytes = {:02x?}", temp);
                //replace(&mut buf, temp);
                for i in 0..8 {
                    buf[i] = temp[i];
                }
                for i in 8..16 {
                    buf[i] = 0;
                }
                println!("buf of last block length = {:02x?}", buf);
                //let a = Self::T::new(buf);
                //temp_file.write(&a.msg());
                //output.write(&a.encrypt(&key));
                let a = Self::T::new(nounce.to_le_bytes());
                nounce += 1;
                temp_file.write(&a.msg());
                let out = xor(a.encrypt(&key), &buf);
                output.write(&out);
                break;
            } else {
                // the whole block

                //let a = Self::T::new(buf);
                //temp_file.write(&a.msg());
                //output.write(&a.encrypt(&key));
                let a = Self::T::new(nounce.to_le_bytes());
                nounce += 1;
                temp_file.write(&a.msg());
                let out = xor(a.encrypt(&key), &buf);
                output.write(&out);
            }
        }
    }

    // used to destruct the cipher with key
    fn ctr_destruct(key: Key, input: &str, output: &str) {
        let mut input = File::open(input).expect("Error while opening input file");
        let mut output = File::create(output).expect("Error while opening output file");
        let mut buf = [0u8; 16];

        let mut noun_file = File::open("nounce").expect("Failed to open nounce file");
        let mut arr = [1u8; 16];
        noun_file.read(&mut arr);
        let mut nounce: u128 = u128::from_le_bytes(arr);
        println!("arr = {:02x?}", arr);
        println!("nounce = {}", nounce);

        //let a = Self::T::new(nounce.to_le_bytes());
        //nounce += 1;
        //temp_file.write(&a.msg());
        //let out = xor(a.encrypt(&key), &buf);
        //output.write(&out);
        let mut buf = [0u8; 16];
        let mut buf_next = [0u8; 16];
        let mut counter = 0;

        let res = input.read(&mut buf_next);
        if let Ok(_) = res {
            buf = buf_next;
        } else {
            panic!("ERROR, file is broken with wrong length");
        }
        while let Ok(res) = input.read(&mut buf_next) {
            if res == 16 {
                // the whole block
                //let a = Self::T::new(buf);
                //output.write(&a.decrypt(&key));
                let a = Self::T::new(nounce.to_le_bytes());
                nounce += 1;
                let out = xor(a.encrypt(&key), &buf);
                output.write(&out);
                buf = buf_next;
                counter += 1;
            } else if res == 0 {
                // now buf store the length of the last segment
                println!("last buf = {:02x?}", buf);
                println!("counter = {}, {} bytes total", counter, counter * 16);
                let a = Self::T::new(nounce.to_le_bytes());
                nounce += 1;
                let out = xor(a.encrypt(&key), &buf);
                println!("out = {:02x?}", out);
                let last_length = u64::from_le_bytes(
                    out[..8]
                        .try_into()
                        .expect("Wrong length in the last length"),
                );
                println!("last_length = {}", last_length);
                let total_length = (counter - 1) * 16 + last_length;
                println!("total length = {}", total_length);
                output
                    .set_len(total_length)
                    .expect("Error when set length of file");
                break;
            } else {
                panic!("ERROR, file is broken with wrong length");
            }
        }
    }
}

pub trait CBC_mode {
    type T: EncDec;
    //type U: AsMut<[u8]> + Default;

    // the main difference is that it use the output of last one
    // xor the msg first before encrypt it
    // and input adds a initial vector
    fn cbc_build(key: Key, input: &str, output: &str, iv: u128) {
        let mut input = File::open(input).expect("Error while opening input file");
        let mut output = File::create(output).expect("Error while opening output file");
        // iv [u8; 16] is used to store the thing to xor
        let mut iv = iv.to_le_bytes();
        let mut buf = [0u8; 16];
        let mut temp_file = File::create("temp.plain").expect("Error while opening output file");

        while let Ok(res) = input.read(&mut buf) {
            if res < 16 {
                // it needs to be padded and get recorded
                // the higher bytes are paded with 0
                for i in res..16 {
                    buf[i] = 0;
                }
                println!("last block length = {}", res);

                // difference emerge here from ecb
                buf = xor(iv, &buf);
                let a = Self::T::new(buf);
                temp_file.write(&a.msg());
                iv = a.encrypt(&key);
                output.write(&iv);

                // record the length in little endian
                // use slice to recover
                let mut temp = res.to_le_bytes();
                println!("temp in bytes = {:02x?}", temp);
                //replace(&mut buf, temp);
                for i in 0..8 {
                    buf[i] = temp[i];
                }
                for i in 8..16 {
                    buf[i] = 0;
                }
                println!("buf of last block length = {:02x?}", buf);
                //let a = Self::T::new(nounce.to_le_bytes());
                //nounce += 1;
                //temp_file.write(&a.msg());
                //let out = xor(a.encrypt(&key), &buf);
                //output.write(&out);
                buf = xor(iv, &buf);
                let a = Self::T::new(buf);
                temp_file.write(&a.msg());
                iv = a.encrypt(&key);
                output.write(&iv);
                break;
            } else {
                // the whole block

                //let a = Self::T::new(buf);
                //temp_file.write(&a.msg());
                //output.write(&a.encrypt(&key));
                //let a = Self::T::new(nounce.to_le_bytes());
                //nounce += 1;
                //temp_file.write(&a.msg());
                //let out = xor(a.encrypt(&key), &buf);
                //output.write(&out);
                buf = xor(iv, &buf);
                let a = Self::T::new(buf);
                temp_file.write(&a.msg());
                iv = a.encrypt(&key);
                output.write(&iv);
            }
        }
    }

    // used to destruct the cipher with key
    fn cbc_destruct(key: Key, input: &str, output: &str, iv: u128) {
        let mut input = File::open(input).expect("Error while opening input file");
        let mut output = File::create(output).expect("Error while opening output file");
        let mut iv = iv.to_le_bytes();
        let mut buf = [0u8; 16];
        let mut buf_next = [0u8; 16];
        let mut counter = 0;
        let mut out = [0u8; 16];

        let res = input.read(&mut buf_next);
        if let Ok(_) = res {
            buf = buf_next;
        } else {
            panic!("ERROR, file is broken with wrong length");
        }

        while let Ok(res) = input.read(&mut buf_next) {
            if res == 16 {
                // the whole block
                //let a = Self::T::new(nounce.to_le_bytes());
                //nounce += 1;
                //let out = xor(a.encrypt(&key), &buf);
                //output.write(&out);
                //buf = buf_next;
                let a = Self::T::new(buf.clone());
                out = xor(iv, &a.decrypt(&key));
                output.write(&out);
                iv = buf;
                buf = buf_next;
                counter += 1;
            } else if res == 0 {
                // now buf store the length of the last segment
                println!("last buf = {:02x?}", buf);
                println!("last iv = {:02x?}", iv);
                println!("counter = {}, {} bytes total", counter, counter * 16);
                let a = Self::T::new(buf.clone());
                let out = xor(iv, &a.decrypt(&key));
                output.write(&out);
                //iv = buf;
                println!("out = {:02x?}", out);
                let last_length = u64::from_le_bytes(
                    out[..8]
                        .try_into()
                        .expect("Wrong length in the last length"),
                );
                println!("last_length = {}", last_length);
                let total_length = (counter - 1) * 16 + last_length;
                println!("total length = {}", total_length);
                output
                    .set_len(total_length)
                    .expect("Error when set length of file");
                break;
            } else {
                panic!("ERROR, file is broken with wrong length");
            }
        }
    }
}
