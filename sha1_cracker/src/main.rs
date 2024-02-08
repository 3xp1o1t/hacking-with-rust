use std::{error::Error, io::BufReader};

use sha1_cracker::{parse_arguments, read_wordlist};

fn main() -> Result<(), Box<dyn Error>> {
    let (wordlist_path, hash_to_crack) = parse_arguments()?;
    let wordlist_file = read_wordlist(&wordlist_path)?;

    match sha1_cracker::crack_sha1_hash(&hash_to_crack, &mut BufReader::new(wordlist_file)) {
        Some(password) => println!("Password found: {}", password),
        None => println!("Password not found in wordlist"),
    }

    Ok(())
}
