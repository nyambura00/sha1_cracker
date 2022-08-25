use std::{env, error::Error};

const SHA1_HEX_STRING_LENGTH: usize = 40;

fn main()-> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args !=3 {
        println!("sha1_cracker: <wordlist.txt> <sha1_hash>");
        return Ok(())
    }

    let hashed_value = args[2].trim();
    if hashed_value.len() != SHA1_HEX_STRING_LENGTH {
        return Err("Hash length is not valid."into())
    }

    Ok(())
}
