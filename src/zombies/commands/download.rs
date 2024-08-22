use crate::utils::base64::encode_file_to_base64::encode_file_to_base64;
use std::io::Error;

pub fn download_data(args: &Vec<&str>) -> Result<String, Error> {
    if args.len() >= 1 {
        let file_path = args[1];
        let base64_file = encode_file_to_base64(&file_path);

        if base64_file.is_ok() {
            return Ok(String::from(format!(
                "download--{}--{}",
                file_path,
                base64_file.unwrap()
            )));
        }

        return base64_file;
    } else {
        return Err(Error::new(
            std::io::ErrorKind::InvalidInput,
            "No file specified for 'download' command",
        ));
    }
}
