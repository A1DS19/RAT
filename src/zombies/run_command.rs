use std::{
    env,
    io::{Error, ErrorKind},
    path::Path,
    process::Command,
};

pub fn run_command(text: &str) -> Result<String, Error> {
    let args: Vec<&str> = text.split_whitespace().collect();
    println!("Running command: {:?}", text);

    if args[0] == "cd" {
        if args.len() > 1 {
            let new_dir = args[1];
            let path = Path::new(new_dir);
            env::set_current_dir(path).map_err(|e| {
                eprintln!("Failed to change directory: {}", e);
                Error::new(
                    ErrorKind::Other,
                    format!("Failed to change directory: {}", e),
                )
            })?;
            println!("Changed directory to: {}", new_dir);
            return Ok(format!("Changed directory to: {}", new_dir));
        } else {
            eprintln!("No directory specified for 'cd' command.");
            return Err(Error::new(
                ErrorKind::InvalidInput,
                "No directory specified",
            ));
        }
    }

    let output = Command::new(args[0])
        .args(&args[1..])
        .output()
        .map_err(|e| {
            eprintln!("Failed to execute command: {}", e);
            Error::new(
                ErrorKind::Other,
                format!("Failed to execute command: {}", e),
            )
        })?;

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Ok(stdout)
}
