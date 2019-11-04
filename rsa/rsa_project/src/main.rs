extern crate clap;
extern crate num_bigint;
extern crate sha3;

mod lib;
mod tests;

use clap::{App, Arg, SubCommand};
use fraction::Ratio;
use lib::{
    cipher_plain::{Cipher, Plaintext},
    key::{PrivateKey, PublicKey},
};
use num_bigint::{BigInt, Sign};
use std::fs::File;
use std::io::{self, BufReader, Read, Write};
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
    Ok(())
}

fn store_pu(pu: PublicKey) -> Result<(), io::Error> {
    let e_filename = "e.key";
    let n_filename = "n.key";

    let mut e_file = File::create(e_filename)?;
    let mut n_file = File::create(n_filename)?;
    let e_buf = pu.e().to_bytes_le().1;
    let n_buf = pu.n().to_bytes_le().1;

    println!("Writing public key e, n\n{}", pu);
    e_file.write_all(&e_buf)?;
    n_file.write_all(&n_buf)?;

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
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .help("Redirect cipher to output file"),
                ),
        )
        .subcommand(
            SubCommand::with_name("decrypt")
                .arg(Arg::with_name("sign").short("s").long("sign"))
                .arg(Arg::with_name("utf-8").long("utf-8"))
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
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .takes_value(true)
                        .help("Redirect cipher to output file"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("encrypt", Some(sub_matches)) => {
            // generate new key with key size
            if sub_matches.is_present("new") {
                let key_size = sub_matches
                    .value_of("key_size")
                    .expect("Please use `-l` to specify the key size");
                let pr = PrivateKey::new_with_key_size(
                    FromStr::from_str(key_size).expect("Unable to parse key_size in ENCRYPT"),
                );
                let pu = PublicKey::from(pr.clone());

                store_pr(pr).expect("Failed to store private key.");
                store_pu(pu).expect("Failed to store public key.");
                return Ok(());
            }
            // read e and n from file
            let e_file_path = sub_matches.value_of("e-key").unwrap();
            let n_file_path = sub_matches.value_of("n-key").unwrap();

            let mut e_file =
                File::open(e_file_path).expect("Please use `--e-key` to specify the e");
            let mut n_file =
                File::open(n_file_path).expect("Please use `--n-key` to specify the n");
            let mut e_vec = [0u8; 512];
            let mut n_vec = [0u8; 512];

            e_file.read(&mut e_vec)?;
            n_file.read(&mut n_vec)?;

            let e = BigInt::from_bytes_le(Sign::Plus, &e_vec);
            let n = BigInt::from_bytes_le(Sign::Plus, &n_vec);
            let pu = PublicKey::from_e_n(e, n);

            // authorize with local file
            if sub_matches.is_present("auth") {
                // authorize process with just public key
                let filename = sub_matches
                    .value_of("filename")
                    .expect("Please specify the file you want to verify with `-f`");
                if pu.authorize(
                    filename,
                    sub_matches
                        .value_of("text")
                        .expect("Unable to authorize without signature, use `-t`"),
                ) {
                    println!("Match signature of {}", filename);
                } else {
                    println!("WRONG signature of {}", filename);
                }
                return Ok(());
            }

            // read from text or file
            let mut buf = [0u8; 512];
            if let Some(t) = sub_matches.value_of("text") {
                let mut bufreader = BufReader::new(t.as_bytes());
                bufreader.read(&mut buf).expect("Error read from `-t`");
            } else {
                if !sub_matches.is_present("auth") {
                    println!("Reading from file to buffer");
                    let mut file = File::open(
                        sub_matches
                            .value_of("filename")
                            .expect("Please specify filename"),
                    )?;
                    file.read(&mut buf).expect("Error when read from filename");
                }
            }

            let cipher = pu.encrypt(Plaintext::new(&buf));
            // redirect cipher to file
            if let Some(output) = sub_matches.value_of("output") {
                let mut file = File::create(output)?;
                let (_, s) = cipher.fragments.to_bytes_le();

                file.write(&s).expect("Write file failed");
                return Ok(());
            }
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
            let pr = PrivateKey::new_with_dn(d, n);

            if sub_matches.is_present("sign") {
                // if it is sign, it needs to be in the file
                let filename = sub_matches
                    .value_of("filename")
                    .expect("Can only sign a file");

                let signature = pr.sign(filename);
                if let Some(output) = sub_matches.value_of("output") {
                    let mut file = File::create(output)?;
                    let (_, s) = signature.to_bytes_le();

                    file.write(&s).expect("Write file failed");
                    return Ok(());
                }
                println!("{}", signature);
                return Ok(());
            }

            let plaintext: Plaintext;

            if let Some(cipher) = sub_matches.value_of("cipher") {
                // cipher is in the text
                plaintext = pr.decrypt(Cipher::new(cipher));
            } else {
                let filename = sub_matches
                    .value_of("filename")
                    .expect("No cipher provided by `-c` or filename");
                let mut file = File::open(filename)?;
                let mut buf = [0u8; 512];

                file.read(&mut buf)?;

                plaintext = pr.decrypt(Cipher::new_from_u8(&buf));
            }

            if sub_matches.is_present("utf-8") {
                let s = plaintext
                    .into_string()
                    .expect("Unable to parse plaintext to string");
                if let Some(output) = sub_matches.value_of("output") {
                    let mut file = File::create(output)?;
                    let (_, s) = plaintext.fragments.to_bytes_le();

                    file.write(&s).expect("Write file failed");
                    return Ok(());
                }
                println!("{}", s);
            }

            if let Some(output) = sub_matches.value_of("output") {
                let mut file = File::create(output)?;
                let (_, s) = plaintext.fragments.to_bytes_le();

                file.write(&s).expect("Write file failed");
                return Ok(());
            }
            println!("{}", plaintext);

            Ok(())
        }
        _ => Ok(()),
    }
}

fn main() {
    //argument_parse().expect("Error when parsing arguments, please use `--help`.");
    //let n = BigInt::from_str("3351434899016066636045491452890486808714908934340001357148989")
    //    .expect("Unable to parse string");
    //let sqrt_n = n.clone().sqrt();
    //println!("{}", sqrt_n);
    //let mut counter = BigInt::from(2u32);
    //while counter.clone() < sqrt_n {
    //    if n.clone() % counter.clone() == BigInt::from(0u32) {
    //        break;
    //    }
    //    counter = counter + 1;
    //}
    //println!("A prime is {}", counter);
    let a = Ratio::new(1u32, 1u32);
    let b = Ratio::new(2u32, 1u32);

    println!("{}", a - b);
}
