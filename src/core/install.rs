use std::{env::current_exe, error::Error};

use colored::Colorize;
use inquire::{Confirm, Text};
use tokio::fs;

use crate::{core::{create_homeref, get_homedir, get_homeref_path}, pathutil::{is_directory_empty, is_file, join_string_path}};

use super::get_lpm_home;

pub fn ask_should_be_installed() -> Result<bool, Box<dyn Error>> {
    Ok(
        Confirm::new(format!("Do you want to install {}?", "LPM".bright_blue()).as_str())
        .with_default(false)
        .prompt()?
    )
}

pub fn ask_if_user_agrees() -> Result<bool, Box<dyn Error>> {
    Ok(
        Confirm::new("Do you agree?")
        .with_default(false)
        .prompt()?
    )
}

pub fn ask_new_homedir(current_homedir: String) -> Result<String, Box<dyn Error>> {
    Ok(
        Text::new("Path:")
        .with_default(&current_homedir)
        .with_placeholder("Type or copy the home dir path here")
        .prompt()?
    )
}

pub async fn first_run() -> Result<(), Box<dyn Error>> {
    println!(
        "This appears to be the first time you have run {} ({}).\n{} needs a {} to call {} so it can download and install {} versions and {} you want to use.",
        "LuaPackageManager".bright_blue(),
        "LPM".bright_blue(),
        "LPM".bright_blue(),
        "directory".yellow(),
        "home".bright_green(),
        "lua".bright_blue(),
        "libraries".bright_green()
    );

    if !ask_should_be_installed()? {
        return Ok(());
    }

    let mut homedir = join_string_path(get_homedir().as_str(), "lpm");
    let default_homedir = homedir.clone();

    println!("By default {} will be installed on {}", "LPM".bright_blue(), homedir.yellow());

    if !ask_if_user_agrees()? {
        println!("Where to install it then?");
        homedir = ask_new_homedir(default_homedir.clone())?;
    }

    while is_file(homedir.as_str())? || !is_directory_empty(homedir.as_str())? {
        println!("{} is a file or is not a empty directory, please type a new home dir path", homedir.yellow());
        homedir = ask_new_homedir(default_homedir.clone())?;
    }

    println!(
        "To ensure that {} always knows where its own home is, a {} file will be generated to store its location.",
        "LPM".bright_blue(),
        get_homeref_path().yellow()
    );

    create_homeref(&homedir).await?;
    fs::create_dir_all(&homedir).await?;
    fs::create_dir_all(join_string_path(&homedir, "downloads")).await?;

    match current_exe() {
        Ok(current_path) => {
            let current_path = current_path.to_str().expect("Unable to convert executable's current path to string");
            let new_path = join_string_path(&homedir, "lpm.exe");
            
            fs::rename(current_path, new_path.clone()).await?;
            println!("{} was moved to {}.", current_path.yellow(), new_path.bright_green());
        },
        Err(e) => {
            return Err(Box::new(e))
        },
    };

    println!("You need to add {} to your system's PATH", homedir.bright_green());

    Ok(())
}

pub async fn uninstall() -> Result<(), Box<dyn Error>> {
    let homedir = get_lpm_home().await?;

    fs::remove_dir_all(&homedir).await?;
    fs::remove_dir_all(join_string_path(&homedir, "downloads")).await?;
    fs::remove_file(&get_homeref_path()).await?;

    Ok(())
}