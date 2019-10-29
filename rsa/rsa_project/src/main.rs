extern crate clap;
extern crate num_bigint;
extern crate sha3;

mod lib;
mod tests;

use clap::{App, Arg, SubCommand};
use lib::{
    cipher_plain::{Cipher, Plaintext},
    key::{PrivateKey, PublicKey},
};
use num_bigint::BigInt;
use std::io;
use std::str::FromStr;

fn argument_parse() -> io::Result<()> {
    let matches = App::new("RSA in Rust")
        .subcommand(
            SubCommand::with_name("encrypt")
                .arg(Arg::with_name("auth").short("a").long("auth"))
                .arg(Arg::with_name("new").long("new").help("Use new key pair"))
                .arg(
                    Arg::with_name("e")
                        .short("e")
                        .long("e-part")
                        .takes_value(true)
                        .help("Exp part of public key"),
                )
                .arg(
                    Arg::with_name("n")
                        .short("n")
                        .long("n-part")
                        .takes_value(true)
                        .help("Modulus part of the public key"),
                )
                .arg(
                    Arg::with_name("text")
                        .short("t")
                        .long("text")
                        .takes_value(true)
                        .help("Text to be encrypt"),
                )
                .arg(
                    Arg::with_name("filename")
                        .short("f")
                        .long("filename")
                        .takes_value(true)
                        .help("File contains plaintext to encrypt"),
                )
                .arg(
                    Arg::with_name("output_filename")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .help("Specify the output filename."),
                )
                .arg(
                    Arg::with_name("key_size")
                        .short("l")
                        .long("key-length")
                        .takes_value(true)
                        .help("Specify the length of key"),
                )
                .arg(
                    Arg::with_name("key_file")
                        .short("k")
                        .long("key-file")
                        .takes_value(true)
                        .help("Specify the file contains public key"),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .arg(Arg::with_name("sign").short("s").long("sign"))
                .arg(
                    Arg::with_name("d")
                        .short("d")
                        .long("d-part")
                        .takes_value(true)
                        .help("Exp part of private key"),
                )
                .arg(
                    Arg::with_name("n")
                        .short("n")
                        .long("n-part")
                        .takes_value(true)
                        .help("Modulus part of the private key"),
                )
                .arg(
                    Arg::with_name("cipher")
                        .short("ci")
                        .long("cipher")
                        .takes_value(true)
                        .help("Cipher to be decrypt"),
                )
                .arg(
                    Arg::with_name("filename")
                        .short("f")
                        .long("filename")
                        .takes_value(true)
                        .help("File contains cipher to decrypt"),
                )
                .arg(
                    Arg::with_name("output_filename")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .help("Specify the output filename."),
                )
                .arg(
                    Arg::with_name("key_file")
                        .short("k")
                        .long("key-file")
                        .takes_value(true)
                        .help("Specify the file contains public key"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("encrypt", Some(sub_matches)) => {
            if sub_matches.is_present("new") {
                let key_size = sub_matches.value_of("key_size").unwrap();
                let pr = PrivateKey::new_with_key_size(
                    FromStr::from_str(key_size).expect("Unable to parse key_size in ENCRYPT"),
                );
                println!("{}", pr);
                let pu = PublicKey::from(pr);

                println!("{}", pu);
                return Ok(());
            }
            let e = sub_matches.value_of("e").unwrap();
            let n = sub_matches.value_of("n").unwrap();
            let text = sub_matches.value_of("text").unwrap();

            let pu = PublicKey::from_e_n(
                BigInt::from_str(e).expect("Unable to parse e-part"),
                BigInt::from_str(n).expect("Unable to parse n-part"),
            );
            if sub_matches.is_present("auth") {
                // authorize process with just public key
                let filename = sub_matches.value_of("filename").unwrap();
                if pu.authorize(filename, text) {
                    println!("Match signature of {}", filename);
                } else {
                    println!("WRONG signature of {}", filename);
                }
            }

            let cipher = pu.encrypt(Plaintext::new(text));
            println!("{}", cipher);
            Ok(())
        }
        ("decrypt", Some(sub_matches)) => {
            let d = sub_matches.value_of("d").unwrap();
            let n = sub_matches.value_of("n").unwrap();
            if sub_matches.is_present("sign") {
                let filename = sub_matches.value_of("filename").unwrap();
                let pr = PrivateKey::new_with_dn(
                    BigInt::from_str(d).expect("Unable to parse d-part in DECRYPT"),
                    BigInt::from_str(n).expect("Unable to parse n-part in DECRYPT"),
                );

                let signature = pr.sign(filename);
                println!("{}", signature);
                return Ok(());
            }
            let cipher = sub_matches.value_of("cipher").unwrap();
            let pr = PrivateKey::new_with_dn(
                BigInt::from_str(d).expect("Unable to parse d-part"),
                BigInt::from_str(n).expect("Unable to parse n-part"),
            );

            let plaintext = pr.decrypt(Cipher::new(cipher));
            println!("{}", plaintext);

            let s = plaintext
                .into_string()
                .expect("Unable to parse plaintext to string");
            println!("{}", s);
            Ok(())
        }
        _ => Ok(()),
    }
}

fn main() {
    argument_parse();
}
