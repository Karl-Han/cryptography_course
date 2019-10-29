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
use num_bigint::{BigInt, Sign};
use std::fs::File;
use std::io::{self, Read, Write};
use std::str::FromStr;

fn store_pr(pr: PrivateKey) -> Result<(), io::Error> {
    let d_filename = "d.key";
    let n_filename = "n.key";

    let mut d_file = File::create(d_filename)?;
    let mut n_file = File::create(n_filename)?;
    let d_buf = pr.d().to_bytes_le().1;
    let n_buf = pr.n().to_bytes_le().1;

    d_file.write_all(&d_buf)?;
    n_file.write_all(&n_buf)?;

    //dbg!(&d_buf);
    d_file.flush()?;
    n_file.flush()?;
    Ok(())
}

fn store_pu(pu: PublicKey) -> Result<(), io::Error> {
    let e_filename = "e.key";
    let n_filename = "n.key";

    let mut e_file = File::create(e_filename)?;
    let mut n_file = File::create(n_filename)?;
    let e_buf = pu.e().to_bytes_le().1;
    let n_buf = pu.n().to_bytes_le().1;

    dbg!(&e_buf);
    e_file.write_all(&e_buf)?;
    n_file.write_all(&n_buf)?;

    e_file.flush()?;
    n_file.flush()?;
    Ok(())
}

fn argument_parse() -> io::Result<()> {
    //let yml = load_yaml!("../cli.yml");
    //let matches = App::from(yml).get_matches();
    //dbg!(&matches);
    let matches = App::new("RSA in Rust")
        .subcommand(
            SubCommand::with_name("encrypt")
                .arg(Arg::with_name("auth").short("a").long("auth"))
                .arg(Arg::with_name("new").long("new").help("Use new key pair"))
                .arg(
                    Arg::with_name("e-key")
                        .short("e")
                        .long("e-key")
                        .takes_value(true)
                        .help("Exp part of public key"),
                )
                .arg(
                    Arg::with_name("n-key")
                        .short("n")
                        .long("n-key")
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
                    Arg::with_name("key_size")
                        .short("l")
                        .long("key-length")
                        .takes_value(true)
                        .help("Specify the length of key"),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .arg(Arg::with_name("sign").short("s").long("sign"))
                .arg(
                    Arg::with_name("d-key")
                        .short("d")
                        .long("d-key")
                        .takes_value(true)
                        .help("Exp part of private key"),
                )
                .arg(
                    Arg::with_name("n-key")
                        .short("n")
                        .long("n-key")
                        .takes_value(true)
                        .help("Modulus part of the private key"),
                )
                .arg(
                    Arg::with_name("cipher")
                        .short("c")
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
                let pu = PublicKey::from(pr.clone());

                println!("{}", pr);
                println!("{}", pu);

                println!("Writing private key and public key to pr.key and pu.key");
                store_pr(pr).expect("Failed to store private key.");
                store_pu(pu).expect("Failed to store public key.");
                return Ok(());
            }
            let e_file_path = sub_matches.value_of("e-key").unwrap();
            let n_file_path = sub_matches.value_of("n-key").unwrap();
            let text = sub_matches.value_of("text").unwrap();

            let mut e_file = File::open(e_file_path)?;
            let mut n_file = File::open(n_file_path)?;
            let mut e_vec = [0u8; 512];
            let mut n_vec = [0u8; 512];

            e_file.read(&mut e_vec)?;
            n_file.read(&mut n_vec)?;

            let e = BigInt::from_bytes_le(Sign::Plus, &e_vec);
            let n = BigInt::from_bytes_le(Sign::Plus, &n_vec);

            let pu = PublicKey::from_e_n(e, n);
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
            let d_file_path = sub_matches.value_of("d-key").unwrap();
            let n_file_path = sub_matches.value_of("n-key").unwrap();

            let mut d_file = File::open(d_file_path)?;
            let mut n_file = File::open(n_file_path)?;
            let mut d_vec = [0u8; 512];
            let mut n_vec = [0u8; 512];

            d_file.read(&mut d_vec)?;
            n_file.read(&mut n_vec)?;

            let d = BigInt::from_bytes_le(Sign::Plus, &d_vec);
            let n = BigInt::from_bytes_le(Sign::Plus, &n_vec);

            if sub_matches.is_present("sign") {
                let filename = sub_matches.value_of("filename").unwrap();
                let pr = PrivateKey::new_with_dn(d, n);

                let signature = pr.sign(filename);
                println!("{}", signature);
                return Ok(());
            }
            let cipher = sub_matches.value_of("cipher").unwrap();
            let pr = PrivateKey::new_with_dn(d, n);

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
