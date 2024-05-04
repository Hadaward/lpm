use std::{error::Error, path::Path};
use std::{env, fs};
use inquire::{Confirm, Text};
use simple_home_dir::home_dir;

use super::environment::{get_env_var, set_env_var};
use super::join_path;

pub fn create_lpm_home() -> Result<String, Box<dyn Error>> {
    let default_home = home_dir().expect("Failed to get home dir");
    let mut default_home = join_path(default_home.to_str().expect("Failed to convert path to string"), "lpm");

    let answer = Confirm::new(format!("Lua Package Manager will be installed on '{}' by default, do you agree?", default_home).as_str())
    .with_default(true)
    .with_help_message("Home directory of LPM")
    .prompt()?;

    if !answer {
        default_home = Text::new("Where to install LPM then?: ").prompt()?;
    }

    set_env_var("LPM_HOME", default_home.as_str())?;
    
    Ok(default_home)
}
/// Return LPM HOME dir
pub async fn check_lpm_install() -> Result<String, Box<dyn Error>> {
    let lpm_home_dir: String;

    match get_env_var("LPM_HOME") {
        Ok(lpm_path) => {
            lpm_home_dir = lpm_path;
        },
        Err(_) => {
            lpm_home_dir = create_lpm_home()?;
        }
    }

    fs::create_dir_all(lpm_home_dir.clone())?;
    fs::create_dir_all(join_path(lpm_home_dir.clone().as_str(), "downloads"))?;

    match env::current_exe() {
        Ok(exe_path) => {
            let exe_path = exe_path.to_str().expect("Couldn't convert exe path to string");
            let exe_bin_path = Path::new(lpm_home_dir.as_str()).join("lpm.exe");
            let exe_bin_path = exe_bin_path.to_str().expect("Couldn't convert path to string");
            
            fs::rename(exe_path, exe_bin_path)?;
        },
        Err(e) => return Err(Box::new(e)),
    };

    let mut path_var = get_env_var("Path")?;

    if !path_var.contains(&lpm_home_dir) {
        let answer = Confirm::new(format!("Add '{}' to PATH environment variable?", lpm_home_dir).as_str())
        .with_default(false)
        .with_help_message(format!("Adds the path '{}' to the Path environment variable, making it globally accessible.", lpm_home_dir).as_str())
        .prompt()?;

        if answer {
            if !path_var.ends_with(";") {
                path_var += ";";
            }

            path_var += &lpm_home_dir;
            path_var += ";";

            set_env_var("Path", &path_var)?;
        }
    }

    Ok(lpm_home_dir)
}