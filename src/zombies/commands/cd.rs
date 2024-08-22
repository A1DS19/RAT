use std::{
    env,
    io::{Error, ErrorKind},
    path::Path,
};
use tracing::info;

pub fn cd_to_dir(args: &Vec<&str>) -> Result<String, Error> {
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
