use std::{env, error::Error, fs::File, io::{BufRead, BufReader}};
use sha1::Digest;

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main()-> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        println!("Not enough arguments.");
        return Ok(())
    }

    let hashed_value = args[2].trim();
    if hashed_value.len() != SHA1_HEX_STRING_LENGTH {
        return Err("Hash length is not valid.".into())
    }

    let wordlist_file = File::open(&args[1])?;
    let reader = BufReader::new(&wordlist_file);
    for line in reader.lines() {
        let line = line?;
        let common_password = line.trim();
        if hashed_value == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
            println!("Password found: {}", &common_password);
            return Ok(());
        }
    }
    println!("password not found in wordlist :(");

    Ok(())
}
