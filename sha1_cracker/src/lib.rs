use std::{env, error::Error, fs::File, io::BufRead};

use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize = 40;

pub fn crack_sha1_hash(hash_to_crack: &str, wordlist: &mut dyn BufRead) -> Option<String> {
    for line in wordlist.lines() {
        if let Ok(common_password) = line {
            let common_password = common_password.trim();
            if hash_to_crack == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
                return Some(common_password.to_string());
            }
        }
    }
    None
}

fn print_usage() {
    println!("Usage: ");
    println!("sha1_cracker <wordlist.txt> <sha1_hash>");
}

pub fn read_wordlist(file_path: &str) -> Result<File, Box<dyn Error>> {
    File::open(file_path).map_err(Into::into)
}

pub fn parse_arguments() -> Result<(String, String), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        print_usage();
        return Err("Invalid arguments".into());
    }

    if args[2].len() != SHA1_HEX_STRING_LENGTH {
        return Err("Invalid sha1 hash".into());
    }

    Ok((args[1].clone(), args[2].clone()))
}
