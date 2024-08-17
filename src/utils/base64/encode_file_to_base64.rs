use base64::{prelude::BASE64_STANDARD, Engine};
use std::{
    fs::File,
    io::{self, Read},
};

pub fn encode_file_to_base64(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;

    Ok(BASE64_STANDARD.encode(&mut buffer))
}
