use std::fs;

// Loads all necessary environment files with `dotenv`
pub fn load_env() -> Result<(), String> {
    // Load the environment-specific environment variable files (checking whether we're in development/debug or production/release)
    if cfg!(debug_assertions) {
        load_env_file_if_present("../.env.development")?; // Non-secret
        load_env_file_if_present("../.env.development.local")?; // Secret
    } else {
        load_env_file_if_present("../.env.production")?; // Non-secret
        load_env_file_if_present("../.env.production.local")?; // Secret
    }
    // Load the files for all environments
    load_env_file_if_present("../.env")?; // Non-secret
    load_env_file_if_present("../.env.local")?; // Secret

    Ok(())
}

// Loads the given environment file if it's present, otherwise does nothing (side-effect based function)
fn load_env_file_if_present(filename: &str) -> Result<(), String> {
    // Check if the file exists
    if fs::metadata(filename).is_ok() {
        let res = dotenv::from_filename(filename);
        match res {
            Ok(_) => return Ok(()),
            Err(err) => return Err(format!(
                "Error fetching environment file '{filename}', {:?}",
                err,
                filename=filename
            ))
        };
    }
    // If it doesn't exist, we don't worry about it, that's the point of this function
    Ok(())
}
