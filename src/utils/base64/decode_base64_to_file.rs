use base64::{prelude::BASE64_STANDARD, Engine};

use std::{
    env::current_dir,
    fs::File,
    io::{self, Error, Result, Write},
};

use tracing::info;

pub fn decode_base64_to_file(base64: &str, file_path: &str) -> Result<()> {
    match File::create(file_path) {
        Ok(mut file) => {
            let base64_as_bytes = base64.as_bytes();

            info!(
                "File saved to: {}/{}",
                current_dir().unwrap().to_str().unwrap(),
                file_path
            );

            match BASE64_STANDARD.decode(base64_as_bytes) {
                Ok(decoded) => {
                    file.write_all(&decoded)?;
                    Ok(())
                }

                Err(e) => Err(Error::new(io::ErrorKind::InvalidData, e)),
            }
        }

        Err(e) => return Err(Error::new(io::ErrorKind::NotFound, e)),
    }
}
