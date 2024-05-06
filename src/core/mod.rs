use std::{error::Error, io::ErrorKind, path::Path};
use tokio::{fs::{read_to_string, File}, io::AsyncWriteExt};
use simple_home_dir::home_dir;

use crate::pathutil::join_string_path;

pub fn get_homeref_path() -> String {
    let home = home_dir()
        .expect("Impossible to retrieve home directory");
    let home = home
        .to_str()
        .expect("Impossible to convert home directory to string");

    return join_string_path(home, &".LPM_HOMEREF");
}

pub async fn is_first_run() -> Result<bool, Box<dyn Error>> {
    if Path::new(get_homeref_path().as_str()).exists() {
        return Ok(false);
    }

    Ok(true)
}

pub async fn create_homeref(lpm_home: &str) -> Result<(), Box<dyn Error>> {
    if !is_first_run().await? {
        return Ok(());
    }

    {
        let mut file = File::create(get_homeref_path()).await?;
        file.write_all(lpm_home.as_bytes()).await?;
        file.sync_all().await?;
    }

    Ok(())
}

pub async fn get_lpm_home() -> Result<String, Box<dyn Error>> {
    if is_first_run().await? {
        return Err(Box::new(std::io::Error::new(ErrorKind::NotFound, "This is the first run so the .LPM_HOMEREF file doesn't exist yet.")));
    }
    
    Ok(read_to_string(get_homeref_path()).await?)
}