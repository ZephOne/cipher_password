extern crate clap;
extern crate pwhash;

use clap::{App, Arg};
use pwhash::{bcrypt, sha1_crypt, sha512_crypt};

fn main() {
    let matches = App::new("cipher_password")
        .version("0.1")
        .author("ZephOne <zephone@protonmail.com>")
        .about("Cipher a password using bcrypt algorithm")
        .arg(
            Arg::with_name("pwd")
                .help("The password you want to cipher")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("algo")
                .short("a")
                .long("algorithm")
                .help("The algorithm you want to use: sha1, sha512, bcrypt (default)")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Print a more verbose output"),
        )
        .get_matches();

    let algo = matches.value_of("algo").unwrap_or("bcrypt");
    match matches.occurrences_of("verbose") {
        1 => println!("Ciphering password using {}", algo),
        _ => (),
    }
    let password = matches.value_of("pwd").unwrap_or("");
    match algo {
        "sha1" => println!("{}", sha1_crypt::hash(password).unwrap()),
        "sha512" => println!("{}", sha512_crypt::hash(password).unwrap()),
        _ => println!("{}", bcrypt::hash(password).unwrap()),
    }
}
